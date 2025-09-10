/// http/1.1 Request Format:
/// 
/// [METHOD] %SP% [Request-URI] %SP% [HTTP-VERSION] %CRLF%
/// ([REQUEST_HEADER_NAME] ":" [REQEUST_HEADER_VALUE] %CRLF%
/// %CRLF%
/// [BODY]
/// 
use std::fmt;
use http_types::{RequestBuilder, Method, Headers, ToUrl};
use async_lib::io::AsyncRead;
use super::types::*;

pub enum BuildError {
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

pub async fn parse_request<S>(stream:S, hostname:&str, port:u16) -> Result<RequestBuilder<S>, BuildError>
    where S: AsyncRead {

    let mut parser = StreamParser::new(stream);

    let start_line = match parser.parse().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err(BuildError::EmptyRequest),
        Err(e) => match e {
            ParseStreamError::ReadError(e) =>
                return Err(BuildError::IoError(e)),
            parse_err => {
                return Err(BuildError::ParseError(parse_err));
            }
         }
    };

    let mut it = start_line.split();
    let method = match  it.next() {
        Some(t) => match Method::from(t.as_str()) {
            Some(m) => m,
            None => return Err(
                BuildError::InvalidMethod(t.decode())
            )
        },
        None => return Err(BuildError::MissingMethod)
    };

    let uri = match it.next() {
        Some(t) => match Uri::parse(&t) {
            Ok(uri) => uri,
            Err(e) => return Err(
                BuildError::InvalidUri(e)
            )
        },
        None => return Err(
            BuildError::MissingUri
        )
    };

    let version = match it.next() {
        Some(t) => match parse_version(&t) {
            Ok(v) => v,
            Err(_) => return Err(
                BuildError::InvalidVersion(t.decode())
            )
        }
        None => return Err(
            BuildError::MissingVersion(method, uri)
        )
    };

    let mut headers = Headers::new();
    while let Some(chunk) = parser.parse().await.map_err(|e|match e{
        ParseStreamError::ReadError(e) => BuildError::IoError(e),
        parse_err => BuildError::ParseError(parse_err)
    })? && chunk.has_some() {
        //chunk = Header Name: Header Value
        let mut line = chunk.as_str().split(':');

        headers.set(
            line.next().unwrap().trim(),
            line.next().unwrap_or("").trim()
        );
    }

    Ok(
        RequestBuilder::new(
            uri.to_url(hostname.into(), port)
                    .map_err(|e|BuildError::InvalidUrl(e))?,
            method,
            headers,
            version,
            Some(parser.take_reader().unwrap())
        )
    )
}