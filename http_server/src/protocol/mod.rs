use std::fmt;
use http_core::{
    method::Method,
    request::RequestBuilder,
    response::Response,
    version::Version,
    error::{HttpError, HttpErrorKind, ValidHttpError}
};
use async_lib::{
    io::{AsyncRead, Result},
    net::TcpStream
};
use types::*;

pub mod types;
mod http0;
mod http1;

pub enum BuildError {
    Http0GetMethodOnly,
    Http0AbsolutePathOnly,
    IoError(std::io::Error),
    EmptyRequest,
    ParseError(ParseStreamError),
    MissingMethod,
    InvalidMethod(String),
    InvalidVersion(String),
    MissingVersion(Method, Uri),
    MissingUri,
    InvalidUri(UriError),
    InvalidUrl(String)
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http0GetMethodOnly => write!(f, "Only absolute paths are allowed in http/0.9 requests!"),
            Self::Http0AbsolutePathOnly => write!(f, "Only Get methods are allowed in http/0.9 requests!"),
            Self::IoError(e) => write!(f, "{}", e),
            Self::EmptyRequest => write!(f, "Empty Reqeust recieved!"),
            Self::ParseError(str) => write!(f, "{}", str),
            Self::MissingMethod => write!(f, "Missing Method at start of request!"),
            Self::InvalidMethod(str) => write!(f, "{} is not a valid method!", str),
            Self::InvalidVersion(str) => write!(f, "{} is not a valid version!", str),
            Self::MissingVersion(_, _) => write!(f, "Unable to find the http version!"),
            Self::InvalidUri(e) => write!(f, "{}", e),
            Self::MissingUri => write!(f, "Uri missing from request!"),
            Self::InvalidUrl(str) => write!(f, "{}", str)
        }
    }
}

impl Into<HttpError> for BuildError {
    fn into(self) -> HttpError {
        match self {
            Self::IoError(_) => HttpErrorKind::InternalServerError.err(),
            Self::ParseError(_) => HttpErrorKind::InternalServerError.err(),
            bad_req => HttpError::new(
                HttpErrorKind::BadRequest,
                &bad_req.to_string()
            )
        }
    }
}

pub async fn build_request<S: AsyncRead>(stream:&mut S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, BuildError> {
    match http1::build_request(stream, hostname, port).await {
        Ok(builder) => Ok(builder),
        Err(e) => match e {
            BuildError::MissingVersion(method, uri) =>  http0::build_request(port, method, uri),
            err => Err(err)
        }
    }
}

pub async fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result<()> {
    match version.major {
        0 => http0::write_response(response, stream).await,
        _ => http1::write_response(response, version, stream).await
    }
}