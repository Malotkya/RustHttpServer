use crate::{
    protocol::*,
    response::Response,
    version::Version
};
use async_lib::{
    io::Result,
    net::TcpStream
};

pub(crate) async fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result<()> {
    match version.major {
        0 => http0::write(response, stream).await,
        _ => http1::write_response(response, version, stream).await
    }
}