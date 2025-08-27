
use crate::{http0, http1};
use crate::{ServerParts, Response, HttpError, HttpErrorKind, RequestBuilder, Version};
use std::{
    task::{Context, Poll},
    sync::atomic::{AtomicU64, Ordering},
    pin::Pin,
    net::TcpStream,
    io::Read
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

enum TaskError {
    ParseError(String),
    IoError(std::io::Error)
}

pub(crate) type Result = std::io::Result<()>;

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task<'server> {
    pub(crate) id: TaskId,
    pub(crate) future: Pin<Box<dyn Future<Output = Result> + 'server>>
}

impl<'s> Task<'s> where {

    pub(crate) fn new<P:ServerParts>(parts: &'s P, stream: TcpStream) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(handle_connection(parts, stream))
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<Result> {
        self.future.as_mut().poll(context)
    }
}

async fn handle_connection<P:ServerParts>(parts:&P, mut stream:TcpStream) -> Result {
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
        Err(TaskError::IoError(e)) => {
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
        Err(TaskError::ParseError(str)) => {
            write_connection_response(
                &mut response,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::BadRequest,
                        &str
                    )
                )
            )?;
            return Ok(())
        }
    };

    write_response(
        &mut response,
        parts.handle_request(&mut req_builder).await,
        req_builder.version
    )?;

    Ok(())
}

fn build_request<S>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, TaskError> where S: Read{
    match http1::parse_request(stream, hostname, port) {
        Ok(builder) => Ok(builder),
        Err(e) => match e {
            http1::BuildError::MissingVersion(method, uri) => {
                http0::build(port, method, uri).map_err(|e|TaskError::ParseError(format!("{}", e)))
            },
            http1::BuildError::IoError(e) => Err(TaskError::IoError(e)),
            err => Err(TaskError::ParseError(format!("{}", err)))
        }
    }
}

fn write_connection_response(stream:&mut TcpStream, response: Response) -> Result {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    })
}

fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> Result {
    match version.major {
        0 => http0::write(response, stream),
        _ => http1::write_response(response, version, stream)
    }
}