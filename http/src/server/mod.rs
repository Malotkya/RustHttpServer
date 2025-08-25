use crate::RequestBuilder;
use std::net::TcpStream;

mod http0;
mod http1;

enum RequestError {
    ParseError(String),
    IoError(std::io::Error)
}

pub fn load_settings(_path:&'static str) -> (u16, String){
    todo!("Ability to load from settings file.")
}

pub fn build_request(stream:TcpStream, hostname:&str, port:u16) -> Result<RequestBuilder, RequestError> {
    match http1::parse_request(stream, hostname, port) {
        Ok(builder) => Ok(builder),
        Err(e) => match e {
            http1::BuildError::MissingVersion(method, uri) => {
                http0::build(port, method, uri).map_err(|e|RequestError::ParseError(format!("{}", e)))
            },
            http1::BuildError::IoError(e) => Err(RequestError::IoError(e)),
            err => Err(RequestError::ParseError(format!("{}", err)))
        }
    }
}