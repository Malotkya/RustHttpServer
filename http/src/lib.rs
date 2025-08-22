#![feature(str_from_raw_parts)]
#[allow(dead_code)]

mod path;
pub use path::Path;
pub mod request;
pub use request::Request;
mod response;
pub use response::Response;
pub mod types;
pub use types::{Url, Method, Result, HttpStatus};
mod error;
pub use error::{HttpError, HttpErrorKind};
mod headers;
pub use headers::Headers;

pub trait Router {
    fn handle<'a>(&self, url:&'a str) -> Result<Option<Response>, HttpError>;
}