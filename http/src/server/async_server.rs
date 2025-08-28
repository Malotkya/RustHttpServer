use crate::{ ServerParts, Server, RequestBuilder, Response };
use super::{
    task::Task,
    executor::Executor
};
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, TryRecvError, Sender, Receiver}, 
    thread::{spawn, JoinHandle}
};

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

    fn spawn_thread(&self, conn:Sender<TcpStream>, close:Receiver<()>) -> std::io::Result<JoinHandle<()>> {
        let (start_send, start_recv) = channel::<String>();
        let (ready_send, ready_recv) = channel::<Option<std::io::Error>>();

        let handle = spawn(move||{
            let addr = match start_recv.recv() {
                Ok(str) => str,
                Err(_) => {
                    ready_send.send(Some(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Listener thread failed to connect with main thread!"
                    ))).expect("Listener thread failed start with main thread!");
                    return;
                }
            };
            let listener = match TcpListener::bind(&addr){
                Ok(tcp) => tcp,
                Err(e) => {
                    ready_send.send(Some(e)).expect("Listener thread failed start with main thread!");
                    return;
                }
            };

            (if let Err(e) = listener.set_nonblocking(true) {
                ready_send.send(Some(e))
            } else {
                ready_send.send(None)
            }).expect("Listener thread failed start with main thread!");
            println!("Listening at: http://{}", addr);

            loop {
                match listener.accept() {
                    Ok(connection) => {
                        if conn.send(connection.0).is_err() {
                            println!("Lost connection with main thread!");
                            break;
                        }
                    },
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::WouldBlock {
                            println!("{}", e)
                        }
                    }
                }

                match close.try_recv() {
                    Ok(_) => {
                        break;
                    },
                    Err(e) => {
                        if e == TryRecvError::Disconnected {
                            println!("Lost connection with main thread!");
                            break;
                        }
                    }
                }
            }

            println!("Stoping listener thread.");
        });

        if start_send.send(self.to_string()).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Lost connection with listener thread!"
            ))
        }

        match ready_recv.recv() {
            Ok(err) => match err {
                Some(e) => Err(e),
                None => Ok(handle)
            },
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Lost connection with listener thread!"
            ))
        }
        
    }
}

impl<P:ServerParts+AsyncParts> Server for AsyncServer<P> {
    fn start() -> std::io::Result<()> {
        let server = Self::new(P::new());
        let mut exec: Executor = Executor::new();

        let (conn_send, conn) = channel::<TcpStream>();
        let (close, close_rec) = channel::<()>();

        let hook = server.spawn_thread(conn_send, close_rec)?;
        let shut_down = move||{
            if close.send(()).is_err() {
                println!("Unable to send close signal to listener thread!");
            }

            if hook.join().is_err() {
                println!("Unable to wait for listener thread to close!");
            }
        };

        loop {
            match conn.try_recv() {
                Ok(stream) => {
                    exec.spawn(Task::new(
                        handle_connection(&server.parts, stream)
                    ));
                },
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                        println!("Lost connection with listener thread!\nAtempting to shutdown gracefully.");
                        shut_down();
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