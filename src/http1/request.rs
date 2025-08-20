/// http/1.1 Request Format:
/// 
/// [METHOD] %SP% [Request-URI] %SP% [HTTP-VERSION] %CRLF%
/// ([REQUEST_HEADER_NAME] ":" [REQEUST_HEADER_VALUE] %CRLF%
/// %CRLF%
/// [BODY]
/// 
use http::{request::{RequestBuilder, RequestBody}, Method, Headers, types::ToUrl};
use http::types::Result;
use std::{net::TcpStream, io::BufReader};


use super::types::*;
use crate::http0::request::build;

pub enum BuildError<'a> {
    //http0 Errors
    IoError(std::io::Error),
    EmptyRequest,
    ParseError(&'static str),
    InvalidMethod(&'a str),
    InvalidVersion(&'a str),
    MissingVersin(Method, Uri),
    InvalidUri(UriError),
    InvalidUrl(String)
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

pub fn parse_request<'a>(stream:TcpStream, hostname:&str, port:u16) -> Result<RequestBuilder<Http1RequestBody>, BuildError<'a>> {
    let parser = TcpStreamParser::new(stream);

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
                BuildError::InvalidMethod(t.as_str())
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
                BuildError::InvalidVersion(t.as_str())
            )
        }
        None => return Err(
            BuildError::MissingVersin(method, uri)
        )
    };

    let mut headers = Headers::new();
    while let Ok(Some(chunk)) = parser.parse() && chunk.has_some(){

    }

    Ok(
        RequestBuilder::new(
            uri.to_url(hostname.into(), port).map_err(|e|BuildError::InvalidUrl(e))?,
            method,
            headers,
            version,
            Http1RequestBody(BufReader::new(parser.0), false)
        )
    )
}