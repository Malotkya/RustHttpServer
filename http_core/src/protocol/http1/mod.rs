/// Following RFC-2616 Stanard:
/// https://datatracker.ietf.org/doc/html/rfc2616
/// 
pub use request::*;
pub use response::*;

pub mod types;
pub mod request;
pub mod response;