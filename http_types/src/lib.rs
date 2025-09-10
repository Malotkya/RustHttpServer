#![feature(str_from_raw_parts)]

mod error;
pub use error::{HttpError, HttpErrorKind};
mod headers;
pub use headers::*;
mod json;
pub use json::*;
mod method;
pub use method::*;
mod path;
pub use path::Path;
pub mod request;
pub use request::*;
mod response;
pub use response::*;
mod status;
pub use status::HttpStatus;
mod url;
pub use url::*;

pub type Result = std::result::Result<Response, HttpError>;

pub trait Router {
    fn handle(&self, req:&mut RequestBuilder<std::net::TcpStream>) -> impl Future<Output = std::result::Result<Option<Response>, HttpError>>;
}

pub struct Version {
    pub major: u8,
    pub minor: u8
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("HTTP/{}.{}", self.major, self.minor)
    }
}

