/// http/1.1 Request Format:
/// 
/// [METHOD] %SP% [Request-URI] %SP% [HTTP-VERSION] %CRLF%
/// ([REQUEST_HEADER_NAME] ":" [REQEUST_HEADER_VALUE] %CRLF%
/// %CRLF%
/// [BODY]
/// 
use http::{request::RequestBuilder, Method};
use http::types::Result;
use std::net::TcpStream;
use std::io::BufReader;

use super::types::*;
use crate::http0::request::build;

pub enum BuildError {
    IoError(std::io::Error),
    EmptyRequest,
    ParseError(&'static str),
    InvalidMethod(Option<String>),
    OnlyGetMethod(Method)
}

pub fn parse_request(stream:TcpStream) -> Result<RequestBuilder<TcpStream>, BuildError> {
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

    let list = split(start_line);
    let method = match list.get(0) {
        Some(t) => match Method::from(t.as_str()) {
            Some(m) => m,
            None => return Err(
                BuildError::InvalidMethod(Some(
                    String::from(t.as_str()
                )))
            )
        },
        None => return Err(BuildError::InvalidMethod(None))
    };

    let version = match list.get(2) {
        Some(t) => match parse_version(t) {
            Ok(v) => v,
            Err(_) =>
        }
        None => {
            return build(method, match list.get(1){
                Some(t) => t.as_str(),
                None => "/"
            });
        }
    }
}