use crate::{http0, http1};
use crate::{RequestBuilder, Response, HttpError, HttpErrorKind, Version};
use std::{
    io::Read,
    net::{TcpListener, TcpStream}
};
pub use executor::Executor;
use task::Task;

mod executor;
mod task;
mod queue;

pub trait ServerParts {
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;
    fn hostname(&self) -> &str;
    fn port(&self) -> &u16;
}

pub struct Server<PARTS: ServerParts> {
    pub(crate) parts: PARTS,
    listener: TcpListener,
}

impl<P:ServerParts> ToString for Server<P> {
    fn to_string(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }
}

impl<P: ServerParts> Server<P> {
    pub fn connect(parts:P) -> std::io::Result<Self> {
        let listener = TcpListener::bind(
            format!("{}:{}", parts.hostname(), *parts.port())
        )?;
        listener.set_nonblocking(true)?;
        Ok(
            Self{
                parts, listener,
            }
        )
    }

    pub fn port(&self) -> &u16 {
        self.parts.port()
    }

    pub fn hostname(&self) -> &str {
        self.parts.hostname()
    }

    pub fn next<'s>(&'s self) -> std::io::Result<Option<Task<'s>>> {
        match self.listener.accept() {
            Ok(connection) => Ok(
                Some(Task::new(
                    handle_connection(&self.parts, connection.0)
                ))
            ),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }
}

enum ConnectionError {
    ParseError(String),
    IoError(std::io::Error)
}

async fn handle_connection<P:ServerParts>(parts:&P, mut stream:TcpStream) -> task::Result {
    let mut response = match stream.try_clone() {
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

    let mut req_builder: RequestBuilder<TcpStream> = match build_request(stream, parts.hostname(), *parts.port()) {
        Ok(req) => req,
        Err(ConnectionError::IoError(e)) => {
            write_connection_response(
                &mut response,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        &format!("{}", e)
                    )
                )
            )?;
            return Err(e)
        },
        Err(ConnectionError::ParseError(str)) => {
            let message = Response::from_error(
                HttpError::new(
                    HttpErrorKind::BadRequest,
                    &str
                )
            );
            println!("??? * {:?}", message);
            write_connection_response(
                &mut response,
                message
            )?;
            return Ok(())
        }
    };

    let resp = parts.handle_request(&mut req_builder).await;
    log(&req_builder, &resp);

    write_response(
        &mut response,
        resp,
        req_builder.version
    )?;

    Ok(())
}

fn build_request<S: Read>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
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

fn write_connection_response(stream:&mut TcpStream, response: Response) -> task::Result {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    })
}

fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> task::Result {
    match version.major {
        0 => http0::write(response, stream),
        _ => http1::write_response(response, version, stream)
    }
}

fn log(req: &RequestBuilder<impl Read>, resp: &Response) {
    println!("{:?} {:?}", req, resp);
}

pub fn load_settings(_path:&'static str) -> (u16, String){
    todo!("Ability to load from settings file.") 
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