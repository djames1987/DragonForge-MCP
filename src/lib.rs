//! DragonForge MCP Phase 1 foundational request-validation primitives.
//! This crate does NOT expose a listener, credential issuance, SQLite writer or tool execution.
//! Future integration must implement C-001/C-005/C-010 before enabling any MCP service.

pub mod protocol;
pub mod transport;
pub const MCP_VERSION: &str = "2026-07-28";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reject {
    Malformed,
    DuplicateKey,
    UnsupportedVersion,
    HeaderMismatch,
    ForbiddenHost,
    ForbiddenOrigin,
    MethodNotAllowed,
    Unauthorized,
    UnsupportedMethod,
    TooLarge,
}

impl Reject {
    pub fn safe_code(&self) -> &'static str {
        match self {
            Reject::Malformed | Reject::DuplicateKey => "INVALID_REQUEST",
            Reject::UnsupportedVersion => "UNSUPPORTED_PROTOCOL_VERSION",
            Reject::HeaderMismatch => "HEADER_MISMATCH",
            Reject::ForbiddenHost => "HOST_DENIED",
            Reject::ForbiddenOrigin => "ORIGIN_DENIED",
            Reject::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            Reject::Unauthorized => "AUTH_INVALID",
            Reject::UnsupportedMethod => "METHOD_NOT_FOUND",
            Reject::TooLarge => "REQUEST_TOO_LARGE",
        }
    }
}
