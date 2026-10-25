//! C-010 request-level structural admission before identity/policy dispatch.
use crate::{Reject, MCP_VERSION};
use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::fmt;

/// Strict JSON value: disallow duplicate keys recursively (serde_json::Value silently overwrites).
#[derive(Debug)]
struct StrictValue(Value);

impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = StrictValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("I-JSON value without duplicate keys") }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value,E> { Ok(StrictValue(Value::Bool(v))) }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value,E> { Ok(StrictValue(Value::from(v))) }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value,E> { Ok(StrictValue(Value::from(v))) }
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value,E> {
                serde_json::Number::from_f64(v).map(|n| StrictValue(Value::Number(n))).ok_or_else(||E::custom("nonfinite number"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value,E> { Ok(StrictValue(Value::String(v.to_owned()))) }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value,E> { Ok(StrictValue(Value::String(v))) }
            fn visit_none<E: de::Error>(self) -> Result<Self::Value,E> { Ok(StrictValue(Value::Null)) }
            fn visit_unit<E: de::Error>(self) -> Result<Self::Value,E> { Ok(StrictValue(Value::Null)) }
            fn visit_some<D: Deserializer<'de>>(self, d:D) -> Result<Self::Value,D::Error> { StrictValue::deserialize(d) }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a:A) -> Result<Self::Value,A::Error> {
                let mut vals=Vec::new();
                while let Some(v)=a.next_element::<StrictValue>()? { vals.push(v.0); }
                Ok(StrictValue(Value::Array(vals)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a:A) -> Result<Self::Value,A::Error> {
                let mut vals=Map::new();
                while let Some((k,v))=a.next_entry::<String,StrictValue>()? {
                    if vals.contains_key(&k) { return Err(de::Error::custom("duplicate JSON object key")); }
                    vals.insert(k,v.0);
                }
                Ok(StrictValue(Value::Object(vals)))
            }
        }
        deserializer.deserialize_any(V)
    }
}

/// Parse a single bounded JSON-RPC request. No implicit initialize/legacy or batch support.
pub fn parse_request(body: &[u8], max_bytes: usize) -> Result<Value,Reject> {
    if body.is_empty() || body.len()>max_bytes { return Err(if body.len()>max_bytes {Reject::TooLarge} else {Reject::Malformed}); }
    let mut decoder=serde_json::Deserializer::from_slice(body);
    let parsed=StrictValue::deserialize(&mut decoder).map_err(|e| {
        if e.to_string().contains("duplicate JSON object key") { Reject::DuplicateKey } else { Reject::Malformed }
    })?;
    decoder.end().map_err(|_|Reject::Malformed)?;
    let v=parsed.0;
    let obj=v.as_object().ok_or(Reject::Malformed)?;
    if obj.get("jsonrpc").and_then(Value::as_str)!=Some("2.0") { return Err(Reject::Malformed); }
    match obj.get("id") {
        Some(Value::Number(n)) if n.is_i64() || n.is_u64()=>{},
        Some(Value::String(_))=>{},
        _=>return Err(Reject::Malformed),
    }
    let method=obj.get("method").and_then(Value::as_str).ok_or(Reject::Malformed)?;
    let params=obj.get("params").and_then(Value::as_object).ok_or(Reject::Malformed)?;
    let meta=params.get("_meta").and_then(Value::as_object).ok_or(Reject::Malformed)?;
    let ver=meta.get("io.modelcontextprotocol/protocolVersion").and_then(Value::as_str).ok_or(Reject::Malformed)?;
    if ver!=MCP_VERSION {return Err(Reject::UnsupportedVersion);}
    let info=meta.get("io.modelcontextprotocol/clientInfo").and_then(Value::as_object).ok_or(Reject::Malformed)?;
    if !matches!(info.get("name"),Some(Value::String(s)) if !s.is_empty())
       || !matches!(info.get("version"),Some(Value::String(s)) if !s.is_empty())
       || !matches!(meta.get("io.modelcontextprotocol/clientCapabilities"),Some(Value::Object(_))) {
        return Err(Reject::Malformed);
    }
    match method {
        "server/discover" | "tools/list" => {},
        "tools/call" => {
            if !matches!(params.get("name"),Some(Value::String(s)) if !s.is_empty())
              || !matches!(params.get("arguments"),Some(Value::Object(_))) {return Err(Reject::Malformed);}
        },
        _=>return Err(Reject::UnsupportedMethod),
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn body(method: &str) -> Vec<u8> {
        format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{}","params":{{"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientInfo":{{"name":"a","version":"1"}},"io.modelcontextprotocol/clientCapabilities":{{}}}}}}}}"#,method).into_bytes()
    }
    #[test] fn valid_tools_list_structure() { assert!(parse_request(&body("tools/list"), 4096).is_ok()); }
    #[test] fn legacy_method_denied() { assert_eq!(parse_request(&body("initialize"),4096),Err(Reject::UnsupportedMethod)); }
    #[test] fn old_version_denied() {
        let s=String::from_utf8(body("tools/list")).unwrap().replace("2026-07-28","2025-11-25");
        assert_eq!(parse_request(s.as_bytes(),4096),Err(Reject::UnsupportedVersion));
    }
    #[test] fn duplicate_root_key_denied() {
        let s=String::from_utf8(body("tools/list")).unwrap().replace(r#""id":1"#,r#""id":1,"id":2"#);
        assert_eq!(parse_request(s.as_bytes(),4096),Err(Reject::DuplicateKey));
    }
    #[test] fn duplicate_nested_key_denied() {
        let s=String::from_utf8(body("tools/list")).unwrap().replace(r#""name":"a""#,r#""name":"a","name":"b""#);
        assert_eq!(parse_request(s.as_bytes(),4096),Err(Reject::DuplicateKey));
    }
    #[test] fn batches_denied() {
        let s=format!("[{}]",String::from_utf8(body("tools/list")).unwrap());
        assert_eq!(parse_request(s.as_bytes(),4096),Err(Reject::Malformed));
    }
    #[test] fn oversize_denied() { assert_eq!(parse_request(&body("tools/list"),10),Err(Reject::TooLarge)); }
    #[test] fn trailing_garbage_denied() {
        let mut b=body("tools/list"); b.extend_from_slice(b" {}");
        assert_eq!(parse_request(&b,4096),Err(Reject::Malformed));
    }
}
