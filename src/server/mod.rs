use http_types::{RequestBuilder, Response};
use async_lib::{
    net::{TcpStream, TcpListener},
    executor::*
};
use std::{
    pin::Pin,
    sync::{
        mpsc,
        Arc
    }
};

mod helpers;

pub trait ServerParts: 'static {
    fn new() -> Self;
    fn hostname(&self) -> &str;
    fn port(&self) -> &u16;
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;
}

pub struct Server<PARTS: ServerParts> {
    pub(crate) parts: PARTS,
}

impl<P: ServerParts> Server<P> {
    pub fn new() -> Self {
        Self{
            parts : P::new()
        }
    }

    pub fn port(&self) -> &u16 {
        self.parts.port()
    }

    pub fn hostname(&self) -> &str {
        self.parts.hostname()
    }

    pub fn gen_listeners(self: Arc<Self>) -> std::io::Result<(impl Fn() + Send + Sync + 'static, impl Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static)> {
        let (conn_send, conn_recv) = mpsc::channel::<TcpStream>();
        let conn_send = Arc::new(conn_send);
        let conn_recv = Arc::new(conn_recv);

        let listener = TcpListener::bind(self.to_string())?;
        let data = self;

        Ok((move ||{
            match listener.sync_accept() {
                Ok(conn) => {
                    conn_send.clone().send(conn.0).unwrap();
                },
                Err(e) => if e.kind() != std::io::ErrorKind::WouldBlock {
                    println!("ERROR: {}", e)
                }
            }
        },
        async_lib::async_fn!(
            clone=(data, conn_recv),
            {
                match conn_recv.try_recv() {
                    Ok(stream) => {
                        spawn_task(async move{
                            if let Err(e) = helpers::handle_connection(&data.parts, stream).await {
                                println!("Error: {}", e)
                            }
                        });
                    },
                    Err(e) => if e != mpsc::TryRecvError::Empty {
                        panic!("{}", e)
                    }
                }
            }
        )))
    }

    fn _start(thread_count: usize, callback:Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static>>) -> std::io::Result<()> {
        let server = Arc::new(Self::new());
        let (listener, reciever) = server.gen_listeners()?;

        let mut list:Vec<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static>> = Vec::with_capacity(2);
        list.push(Box::new(reciever));
        if callback.is_some() {
            list.push(callback.unwrap())
        }

        init_thread_pool_with_listener(thread_count, listener);
        start_with_callback_list(list);

        println!("Good bye!");
        Ok(())
    }

    pub fn start_with_calback(thread_count: usize, callback_loop:impl Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static) -> std::io::Result<()> {
        Self::_start(thread_count, Some(
            Box::new(callback_loop)
        ))
    }

    pub fn start(thread_count: usize) -> std::io::Result<()> {
        Self::_start(thread_count, None)
    }
}

impl<P:ServerParts> ToString for Server<P> {
    fn to_string(&self) -> String {
        format!("{}:{}", self.parts.hostname(), *self.parts.port())
    }
}
