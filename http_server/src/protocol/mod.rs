use std::fmt;
use http_core::{
    method::Method,
    error::{HttpError, HttpErrorKind},
    request::RequestBuilder,
    response::Response,
    version::Version
};
use crate::connection::{
    ConnectionError,
    write_connection_error_response
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

async fn build_protocol_request<S: AsyncRead>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
    match http1::build_request(stream, hostname, port).await {
        Ok(builder) => Ok(builder),
        Err(e) => match e {
            BuildError::MissingVersion(method, uri) => {
                http0::build_request(port, method, uri).map_err(|e|ConnectionError::ParseError(format!("{}", e)))
            },
            BuildError::IoError(e) => Err(ConnectionError::IoError(e)),
            err => Err(ConnectionError::ParseError(format!("{}", err)))
        }
    }
}

pub async fn build_request(stream: TcpStream, resp: &mut TcpStream, hostname:&str, port:u16) -> Result<Option<RequestBuilder<TcpStream>>> {
    match build_protocol_request(stream, hostname, port).await {
        Ok(req) => Ok(Some(req)),
        Err(ConnectionError::IoError(e)) => {
            write_connection_error_response(
                resp,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        &format!("{}", e)
                    )
                )
            ).await?;
            Err(e)
        },
        Err(ConnectionError::ParseError(str)) => {
            let message = Response::from_error(
                HttpError::new(
                    HttpErrorKind::BadRequest,
                    &str
                )
            );
            println!("??? * {:?}", message);
            write_connection_error_response(
                resp,
                message
            ).await?;
            Ok(None)
        }
    }
}

pub async fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result<()> {
    match version.major {
        0 => http0::write_response(response, stream).await,
        _ => http1::write_response(response, version, stream).await
    }
}