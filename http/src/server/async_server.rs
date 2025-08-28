use crate::{ ServerParts, Server, RequestBuilder, Response };
use super::{
    task::Task,
    executor::Executor
};
use std::net::{TcpListener, TcpStream};

pub trait AsyncParts {
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;
}

pub struct AsyncServer<PARTS: ServerParts+AsyncParts> {
    pub(crate) parts: PARTS,
    listener: TcpListener,
}

impl<P:ServerParts+AsyncParts> ToString for AsyncServer<P> {
    fn to_string(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }
}

impl<P: ServerParts+AsyncParts> AsyncServer<P> {
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

impl<P:ServerParts+AsyncParts> Server for AsyncServer<P> {
    fn start() -> std::io::Result<()> {
        let server = Self::connect(P::new())?;
        let mut exec = Executor::new();

        println!("Listening at: http://{}", server.to_string());

        loop {
            if let Some(task) = server.next()? {
                exec.spawn(task);
            }

            exec.run_ready_tasks();
            //exec.sleep_if_idle();
        }
    }
}

async fn handle_connection<P:ServerParts+AsyncParts>(parts:&P, stream:std::net::TcpStream) -> super::task::Result {
    let (req_stream, mut resp_stream) = super::build_connections(stream)?;

    if let Some(mut request) = super::build_request(req_stream, &mut resp_stream, parts.hostname(), *parts.port())? {
        let response = parts.handle_request(&mut request).await;
        crate::log(&request, &response);
        crate::write_response(
            &mut resp_stream,
            response,
            request.version
        )?;
    }

    Ok(())      
}