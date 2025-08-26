use crate::{RequestBuilder, Response, Version};
use std::{io::Read, net::TcpStream};

mod http0;
mod http1;

pub enum RequestError {
    ParseError(String),
    IoError(std::io::Error)
}

pub fn load_settings(_path:&'static str) -> (u16, String){
    todo!("Ability to load from settings file.")
}

pub fn build_request<S>(stream:S, hostname:&str, port:u16) -> Result<RequestBuilder<S>, RequestError> where S: Read{
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

pub fn write_connection_response(stream:&mut TcpStream, response: Response) -> Result<(), std::io::Error> {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    })
}

pub fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result<(), std::io::Error> {
    match version.major {
        0 => http0::write(response, stream),
        _ => http1::write_response(response, version, stream)
    }
}

pub fn get_arguments() -> (Option<u16>, Option<String>) {
    let mut port = None;
    let mut hostname = None;

    for input in std::env::args() {
        let input: Vec<_> = input.split("=").collect();

        match input[0].to_ascii_lowercase().as_str() {
            "port" => {
                port = Some(input.get(1).unwrap().parse().unwrap())
            },
            "hostname" => {
                hostname = Some(input.get(1).unwrap().to_string())
            },
            key => {
                panic!("Unknown command line argument: {key}!")
            }
        }
    }

    (port, hostname)
}