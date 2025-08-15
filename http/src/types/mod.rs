mod url;
pub use url::{Url, Search};

pub type Result<T, E: std::fmt::Display> = std::result::Result<T, E>;

pub struct Version {
    pub major: u8,
    pub minor: u8
}