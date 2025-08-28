use crate::{ ServerParts, Server, RequestBuilder, Response };
use std::net::{TcpListener, TcpStream};

pub trait SyncParts {
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> Response;
}

pub struct SyncServer<PARTS: ServerParts+SyncParts> {
    pub(crate) parts: PARTS,
    listener: TcpListener,
}

impl<P:ServerParts+SyncParts> ToString for SyncServer<P> {
    fn to_string(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }
}

impl<P: ServerParts+SyncParts> SyncServer<P> {
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

    pub fn next<'s>(&'s self) -> std::io::Result<()> {
        match self.listener.accept() {
            Ok(connection) => {
                handle_connection(&self.parts, connection.0)?;
                Ok(())
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}

impl<P:ServerParts+SyncParts> Server for SyncServer<P> {
    fn start() -> std::io::Result<()> {
        let server = Self::connect(P::new())?;

        println!("Listening at: http://{}", server.to_string());

        loop {
            server.next()?;
        }
    }
}

fn handle_connection<P:ServerParts+SyncParts>(parts:&P, stream:std::net::TcpStream) -> super::task::Result {
    let (req_stream, mut resp_stream) = super::build_connections(stream)?;

    if let Some(mut request) = super::build_request(req_stream, &mut resp_stream, parts.hostname(), *parts.port())? {
        let response = parts.handle_request(&mut request);
        crate::log(&request, &response);
        crate::write_response(
            &mut resp_stream,
            response,
            request.version
        )?;
    }

    Ok(())      
}