use crate::{ ServerParts, Server, RequestBuilder, Response };
use super::{
    task::Task,
    executor::Executor
};
use std::{
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering}, mpsc, Arc
    }, 
    thread
};

mod atomics;



pub trait AsyncParts {
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;
}

pub struct AsyncServer<PARTS: ServerParts+AsyncParts> {
    pub(crate) parts: PARTS,
}

impl<P:ServerParts+AsyncParts> ToString for AsyncServer<P> {
    fn to_string(&self) -> String {
        format!("{}:{}", self.parts.hostname(), *self.parts.port())
    }
}

impl<P: ServerParts+AsyncParts> AsyncServer<P> {
    pub fn new(parts:P) -> Self {
        Self{
            parts,
        }
    }

    pub fn port(&self) -> &u16 {
        self.parts.port()
    }

    pub fn hostname(&self) -> &str {
        self.parts.hostname()
    }

    pub fn spawn_thread(&self) -> std::io::Result<atomics::ListenerHandle> {
        let (conn, conn_recv) = mpsc::channel::<TcpStream>();
        let address = Arc::new(self.to_string());
        let close = Arc::new(AtomicBool::new(false));
        let (ready_1, ready_2) = atomics::Ready::new();

        let handle = atomics::ListenerHandle(
            close.clone(),
            thread::spawn(move||{
                let listener = match TcpListener::bind(address.as_str()){
                    Ok(tcp) => tcp,
                    Err(e) => {
                        ready_1.set_error(e);
                        return;
                    }
                };
                
                if let Err(e) = listener.set_nonblocking(true) {
                    ready_1.set_error(e);
                    return;
                } else {
                    ready_1.set_success();
                }
                println!("Listening at: http://{}", address.as_str());

                loop {
                    while listener.select
                    for connection in listener.incoming() {

                    }

                    if close.load(Ordering::Relaxed) {
                        break;
                    }
                }

                println!("Stoping listener thread.");
            }),
            conn_recv
        );

        ready_2.wait().map(|_|handle)
    }
}



impl<P:ServerParts+AsyncParts> Server for AsyncServer<P> {
    fn start() -> std::io::Result<()> {
        let server = Self::new(P::new());
        let mut exec: Executor = Executor::new();
        let hook = server.spawn_thread()?;

        loop {
            match hook.conn() {
                Ok(stream) => {
                    exec.spawn(Task::new(
                        handle_connection(&server.parts, stream)
                    ));
                },
                Err(e) => {
                    if e == atomics::ConnectionError::Disconnected {
                        println!("Lost connection with listener thread!\nAtempting to shutdown gracefully.");
                        hook.wait_close().expect("Unable to wait for thread to close!");
                        break;
                    }
                }
            }

            exec.run_ready_tasks();
            //exec.sleep_if_idle();
        }

        println!("Good bye!");
        Ok(())
    }
}

async fn handle_connection<P:ServerParts+AsyncParts>(parts:&P, stream:TcpStream) -> super::task::Result {
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