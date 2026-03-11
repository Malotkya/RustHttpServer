use http_core::{RequestBuilder, Response};
use async_lib::{
    net::{TcpStream, TcpListener},
    executor::*
};
use std::{
    pin::Pin,
    sync::{
        mpsc,
        Arc
    },
    rc::Rc
};

mod helpers;

pub trait Server: 'static + Sized + Clone {
    fn hostname(&self) -> &str;
    fn port(&self) -> u16;
    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;

    fn start(&self, thread_count: usize /*, callback:Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static>>*/) -> std::io::Result<()> {
        let (listener, reciever) = generate_listeners(self)?;

        init_thread_pool_with_listener(thread_count, listener);
        start_with_callback(reciever);

        println!("Good bye!");
        Ok(())
    }

    fn address(&self) -> String {
        format!("{}:{}", self.hostname(), self.port())
    }
}

fn generate_listeners<S:Server>(server:&S) -> std::io::Result<(impl Fn() + Send + Sync + 'static, impl Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static)> {
    let (conn_send, conn_recv) = mpsc::channel::<TcpStream>();
    let conn_send = Arc::new(conn_send);
    let conn_recv = Rc::new(conn_recv);

    let listener = TcpListener::bind(server.address())?;
    let data = server.clone();

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
                        if let Err(e) = helpers::handle_connection(data, stream).await {
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