use crate::{
    server::response::write_response,
    version::Version,
    response::Response
};
use async_lib::{
    io::Result,
    net::TcpStream
};

pub(crate) enum ConnectionError {
    ParseError(String),
    IoError(std::io::Error)
}

pub async fn write_connection_error_response(stream:&mut TcpStream, response: Response) -> Result<()> {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    }).await
}