//! C-010 pre-dispatch HTTP admission. Integration must not bypass these checks.
use crate::{protocol::parse_request, Reject, MCP_VERSION};
use serde_json::Value;
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr,SocketAddr};

pub const MAX_MCP_BODY: usize = 1_048_576;

/// Do not permit a public/non-loopback binding even through caller misconfiguration.
pub fn require_loopback(addr: SocketAddr) -> Result<(),Reject> {
    match addr.ip() {
        IpAddr::V4(v) if v==Ipv4Addr::LOCALHOST=>Ok(()),
        IpAddr::V6(v) if v==Ipv6Addr::LOCALHOST=>Ok(()),
        _=>Err(Reject::ForbiddenHost),
    }
}

/// Exact expected Host and Origin (only local configured address/port).
/// Header extraction and duplicate-header rejection remain a responsibility of
/// the future HTTP adapter; never choose the first of multiple values.
pub fn validate_host_origin(host: &str, origin: Option<&str>, listen: SocketAddr)->Result<(),Reject> {
    require_loopback(listen)?;
    let expected=match listen.ip() {
        IpAddr::V4(_) =>format!("{}:{}",listen.ip(),listen.port()),
        IpAddr::V6(_) =>format!("[{}]:{}",listen.ip(),listen.port()),
    };
    if host!=expected {return Err(Reject::ForbiddenHost);}
    if let Some(o)=origin {
        if o!=format!("http://{}",expected) {return Err(Reject::ForbiddenOrigin);}
    }
    Ok(())
}

/// Structural HTTP admission only: successful validation does NOT authenticate,
/// authorize, discover or invoke a tool. Future gateway dispatch MUST independently
/// verify enrolled bearer identity, capability and per-invocation policy.
pub fn validate_headers_and_body(
    content_type: &str, accept: &str, header_version: &str,
    header_method: &str, header_name: Option<&str>, body: &[u8],
)->Result<Value,Reject> {
    if content_type!="application/json" || !accept.split(',').map(str::trim).any(|a|a=="application/json")
        || !accept.split(',').map(str::trim).any(|a|a=="text/event-stream") {return Err(Reject::HeaderMismatch);}
    let v=parse_request(body,MAX_MCP_BODY)?;
    if header_version!=MCP_VERSION {return Err(if header_version=="2025-11-25" { Reject::UnsupportedVersion } else {Reject::HeaderMismatch});}
    let method=v["method"].as_str().ok_or(Reject::Malformed)?;
    if method!=header_method {return Err(Reject::HeaderMismatch);}
    if method=="tools/call" {
        if header_name != v["params"]["name"].as_str(){return Err(Reject::HeaderMismatch);}
    } else if header_name.is_some() {return Err(Reject::HeaderMismatch);}
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn loopback_only() {
        assert!(require_loopback("127.0.0.1:8765".parse().unwrap()).is_ok());
        assert!(require_loopback("[::1]:8765".parse().unwrap()).is_ok());
        assert!(require_loopback("0.0.0.0:8765".parse().unwrap()).is_err());
        assert!(require_loopback("192.168.1.10:8765".parse().unwrap()).is_err());
        assert!(require_loopback("[::]:8765".parse().unwrap()).is_err());
    }
    #[test] fn host_and_origin_fail_closed() {
        let addr="127.0.0.1:8765".parse().unwrap();
        assert!(validate_host_origin("127.0.0.1:8765",None,addr).is_ok());
        assert!(validate_host_origin("127.0.0.1:8765",Some("http://127.0.0.1:8765"),addr).is_ok());
        assert_eq!(validate_host_origin("evil.test:8765",None,addr),Err(Reject::ForbiddenHost));
        assert_eq!(validate_host_origin("127.0.0.1:8765",Some("http://evil.test"),addr),Err(Reject::ForbiddenOrigin));
    }
}
