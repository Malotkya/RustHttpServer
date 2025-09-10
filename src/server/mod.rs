use http_types::{RequestBuilder, Response, Version, HttpError, HttpErrorKind};
use crate::{
    http0, http1,
    
};
use std::{
    io::Read,
    net::{TcpStream, TcpListener},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc
    },
    thread
};

mod atomics;

pub trait ServerParts {
    fn new() -> Self;
    fn hostname(&self) -> &str;
    fn port(&self) -> &u16;
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;
}

pub struct Server<PARTS: ServerParts> {
    pub(crate) parts: PARTS,
}

impl<P: ServerParts> Server<P> {
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

    /*fn start() -> std::io::Result<()> {
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
    }*/
}

impl<P:ServerParts> ToString for Server<P> {
    fn to_string(&self) -> String {
        format!("{}:{}", self.parts.hostname(), *self.parts.port())
    }
}

enum ConnectionError {
    ParseError(String),
    IoError(std::io::Error)
}

fn http_build_request<S: Read>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, ConnectionError> {
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

pub(crate) fn build_request(stream: TcpStream, resp: &mut TcpStream, hostname:&str, port:u16) -> std::io::Result<Option<RequestBuilder<TcpStream>>> {
    match http_build_request(stream, hostname, port) {
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
            )?;
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
            super::write_connection_response(
                resp,
                message
            )?;
            Ok(None)
        }
    }
}

pub(crate) fn build_connections(mut stream:TcpStream) -> std::io::Result<(TcpStream, TcpStream)> {
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
            )?;
            return Err(e);
        }
    };

    Ok((stream, response))
}

pub fn write_connection_response(stream:&mut TcpStream, response: Response) -> task::Result {
    write_response(stream, response, Version {
        major: 1,
        minor: 1
    })
}

pub(crate) fn write_response(stream:&mut TcpStream, response:Response, version:Version) -> task::Result {
    match version.major {
        0 => http0::write(response, stream),
        _ => http1::write_response(response, version, stream)
    }
}

pub(crate) fn log(req: &RequestBuilder<impl Read>, resp: &Response) {
    println!("{:?} {:?}", req, resp);
}

pub fn load_settings(_path:&str) -> (Option<u16>, Option<String>){
    todo!("Ability to load from settings file.") 
}

pub fn get_arguments() -> (Option<u16>, Option<String>, Option<String>) {
    let mut port = None;
    let mut hostname = None;
    let mut config_file = None;

    for input in std::env::args() {
        if let Some(index) = input.find("=") {
            let key = &input[..index];
            let value = &input[index+1..];

            match key.to_ascii_lowercase().as_str() {
                "port" => {
                    port = Some(value.parse().unwrap())
                },
                "hostname" => {
                    hostname = Some(value.to_owned())
                },
                "config" => {
                    config_file = Some(value.to_owned())
                },
                key => {
                    panic!("Unknown command line argument: {key}!")
                }
            }
        }
    }

    (port, hostname, config_file)
}

async fn handle_connection<P:ServerParts>(parts:&P, stream:TcpStream) -> super::task::Result {
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