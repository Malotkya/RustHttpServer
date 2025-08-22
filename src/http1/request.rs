/// http/1.1 Request Format:
/// 
/// [METHOD] %SP% [Request-URI] %SP% [HTTP-VERSION] %CRLF%
/// ([REQUEST_HEADER_NAME] ":" [REQEUST_HEADER_VALUE] %CRLF%
/// %CRLF%
/// [BODY]
/// 
use std::fmt;
use http::{request::{RequestBuilder, RequestBody}, Method, Headers, types::ToUrl};
use http::types::Result;
use std::{net::TcpStream, io::BufReader};
use super::types::*;

pub enum BuildError {
    IoError(std::io::Error),
    EmptyRequest,
    ParseError(&'static str),
    InvalidMethod(String),
    InvalidVersion(String),
    MissingVersion(Method, Uri),
    InvalidUri(UriError),
    InvalidUrl(String)
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e),
            Self::EmptyRequest => write!(f, "Empty Reqeust recieved!"),
            Self::ParseError(str) => write!(f, "{}", str),
            Self::InvalidMethod(str) => write!(f, "{} is not a valid method!", str),
            Self::InvalidVersion(str) => write!(f, "{} is not a valid version!", str),
            Self::MissingVersion(_, _) => write!(f, "Unable to find the http version!"),
            Self::InvalidUri(e) => write!(f, "{}", e),
            Self::InvalidUrl(str) => write!(f, "{}", str)
        }
    }
}

struct Http1RequestBody(BufReader<TcpStream>, bool);

impl RequestBody for Http1RequestBody {
    fn body(&mut self) -> std::result::Result<&[u8], &'static str> {
        if self.1 {
            Err("Body has already been used!")
        } else {
            self.1 = true;
            Ok(self.0.buffer())
        }
    }
}

pub fn parse_request(stream:TcpStream, hostname:&str, port:u16) -> Result<RequestBuilder, BuildError> {
    let mut parser = TcpStreamParser::new(stream);

    let start_line = match parser.parse(){
        Ok(Some(line)) => line,
        Ok(None) => return Err(BuildError::EmptyRequest),
        Err(e) => match e {
            ParseStreamError::ParseError(e) =>
                return Err(BuildError::ParseError(e)),
            ParseStreamError::ReadError(e) =>
                return Err(BuildError::IoError(e))
        }
    };

    let mut it = start_line.split();
    let method = match  it.next() {
        Some(t) => match Method::from(t.as_str()) {
            Some(m) => m,
            None => return Err(
                BuildError::InvalidMethod(t.to_string())
            )
        },
        None => return Err(BuildError::ParseError("Missing Method at start of request!"))
    };

    let uri = match it.next() {
        Some(t) => match Uri::parse(&t) {
            Ok(uri) => uri,
            Err(e) => return Err(
                BuildError::InvalidUri(e)
            )
        },
        None => return Err(
            BuildError::ParseError("Uri missing from request!")
        )
    };

    let version = match it.next() {
        Some(t) => match parse_version(&t) {
            Ok(v) => v,
            Err(_) => return Err(
                BuildError::InvalidVersion(t.to_string())
            )
        }
        None => return Err(
            BuildError::MissingVersion(method, uri)
        )
    };

    let mut headers = Headers::new();
    while let Some(chunk) = parser.parse().map_err(|e|match e{
        ParseStreamError::ParseError(str) => BuildError::ParseError(str),
        ParseStreamError::ReadError(e) => BuildError::IoError(e)
    })? && chunk.has_some() {
        //chunk = Header Name: Header Value
        let mut line = chunk.as_str().split(':');

        headers.set(
            line.next().unwrap().trim(),
            line.next().unwrap_or("").trim()
        )
    }

    Ok(
        RequestBuilder::new(
            uri.to_url(hostname.into(), port)
                    .map_err(|e|BuildError::InvalidUrl(e))?,
            method,
            headers,
            version,
            Box::new(
                Http1RequestBody(BufReader::new(parser.0), false)
            )
        )
    )
}