use http_types::{Response, Version, HttpError, HttpErrorKind, RequestBuilder};
use async_lib::{
    io::{AsyncRead, Result},
    net::TcpStream
};
use super::ServerParts;
use crate::{
    http0, http1,
};

enum ConnectionError {
    ParseError(String),
    IoError(std::io::Error)
}

async fn http_build_request<S: AsyncRead>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
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
    match http_build_request(stream, hostname, port).await {
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
            write_connection_response(
                resp,
                message
            ).await?;
            Ok(None)
        }
    }
}

pub(crate) async fn build_connections(mut stream:TcpStream) -> std::io::Result<(TcpStream, TcpStream)> {
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
            ).await?;
            return Err(e);
        }
    };

    Ok((stream, response))
}

pub async fn write_connection_response(stream:&mut TcpStream, response: Response) -> Result<()> {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    }).await
}

pub(crate) async fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result<()> {
    match version.major {
        0 => http0::write(response, stream).await,
        _ => http1::write_response(response, version, stream).await
    }
}

pub(crate) async fn handle_connection<P: ServerParts>(parts: &P, stream:TcpStream) -> Result<()> {
    let (req_stream, mut resp_stream) = build_connections(stream).await?;

    if let Some(mut request) = build_request(req_stream, &mut resp_stream, parts.hostname(), *parts.port()).await? {
        let response = parts.handle_request(&mut request).await;
        crate::log(&request, &response);
        write_response(
            &mut resp_stream,
            response,
            request.version
        ).await?;
    }

    Ok(())
}