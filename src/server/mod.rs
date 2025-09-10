use crate::{
    http0, http1,
    RequestBuilder, Response, Version, HttpError, HttpErrorKind
};
use std::{
    io::Read,
    net::TcpStream
};
pub use async_server::*;
pub use sync_server::*;

pub(crate) mod executor;
pub(crate) mod task;
pub(crate) mod queue;
mod async_server;
mod sync_server;

pub trait ServerParts {
    fn new() -> Self;
    fn hostname(&self) -> &str;
    fn port(&self) -> &u16;
}

pub trait Server {
    fn start() -> std::io::Result<()>;
}

enum ConnectionError {
    ParseError(String),
    IoError(std::io::Error)
}

fn http_build_request<S: Read>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
    match http1::parse_request(stream, hostname, port) {
        Ok(builder) => Ok(builder),
        Err(e) => match e {
            http1::BuildError::MissingVersion(method, uri) => {
                http0::build(port, method, uri).map_err(|e|ConnectionError::ParseError(format!("{}", e)))
            },
            http1::BuildError::IoError(e) => Err(ConnectionError::IoError(e)),
            err => Err(ConnectionError::ParseError(format!("{}", err)))
        }
    }
}

pub(crate) fn build_request(stream: TcpStream, resp: &mut TcpStream, hostname:&str, port:u16) -> std::io::Result<Option<RequestBuilder<TcpStream>>> {
    match http_build_request(stream, hostname, port) {
        Ok(req) => Ok(Some(req)),
        Err(ConnectionError::IoError(e)) => {
            write_connection_response(
                resp,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        &format!("{}", e)
                    )
                )
            )?;
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
            super::write_connection_response(
                resp,
                message
            )?;
            Ok(None)
        }
    }
}

pub(crate) fn build_connections(mut stream:TcpStream) -> std::io::Result<(TcpStream, TcpStream)> {
    let response = match stream.try_clone() {
        Ok(stream) => stream,
        Err(e) => {
            write_connection_response(
                &mut stream,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        "An error occured while handeling the connection!"
                    )
                )
            )?;
            return Err(e);
        }
    };

    Ok((stream, response))
}

pub fn write_connection_response(stream:&mut TcpStream, response: Response) -> task::Result {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    })
}

pub(crate) fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> task::Result {
    match version.major {
        0 => http0::write(response, stream),
        _ => http1::write_response(response, version, stream)
    }
}

pub(crate) fn log(req: &RequestBuilder<impl Read>, resp: &Response) {
    println!("{:?} {:?}", req, resp);
}

pub fn load_settings(_path:&str) -> (Option<u16>, Option<String>){
    todo!("Ability to load from settings file.") 
}

pub fn get_arguments() -> (Option<u16>, Option<String>, Option<String>) {
    let mut port = None;
    let mut hostname = None;
    let mut config_file = None;

    for input in std::env::args() {
        if let Some(index) = input.find("=") {
            let key = &input[..index];
            let value = &input[index+1..];

            match key.to_ascii_lowercase().as_str() {
                "port" => {
                    port = Some(value.parse().unwrap())
                },
                "hostname" => {
                    hostname = Some(value.to_owned())
                },
                "config" => {
                    config_file = Some(value.to_owned())
                },
                key => {
                    panic!("Unknown command line argument: {key}!")
                }
            }
        }
    }

    (port, hostname, config_file)
}