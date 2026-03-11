use crate::{
    protocol::*,
    server::error::{ConnectionError, write_connection_error_response},
    error::{HttpError, HttpErrorKind},
    request::RequestBuilder,
    response::Response,
};
use async_lib::{
    io::{AsyncRead, Result},
    net::TcpStream
};

async fn build_request_inner<S: AsyncRead>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
    match http1::parse_request(stream, hostname, port).await {
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

pub(crate) async fn build_request(stream: TcpStream, resp: &mut TcpStream, hostname:&str, port:u16) -> Result<Option<RequestBuilder<TcpStream>>> {
    match build_request_inner(stream, hostname, port).await {
        Ok(req) => Ok(Some(req)),
        Err(ConnectionError::IoError(e)) => {
            write_connection_error_response(
                resp,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        &format!("{}", e)
                    )
                )
            ).await?;
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
            write_connection_error_response(
                resp,
                message
            ).await?;
            Ok(None)
        }
    }
}