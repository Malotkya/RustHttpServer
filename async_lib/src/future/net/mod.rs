use crate::{
    executor::{
        is_running,
        thread::ThreadProcess
    },
    io::{
        ErrorKind, Result
    }
};

pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
pub(crate) use super::clone_io_result;

mod tcp;
pub use tcp::*;

pub fn tcp_listener_thread(addr:String, callback: impl Fn(TcpStream) + Sync + Send + 'static) -> Result<impl ThreadProcess> {
    let mut listener = TcpListener::bind(addr.clone())?;

    if listener.set_nonblocking(true).is_err() {
        println!("Failed to set nonblocking on TcpListener!");
    }

    Ok(move ||{
        println!("Listening at {}", addr);

        while is_running() {
            match listener.sync_accept() {
                Ok(conn) => {
                    callback(conn.0);
                },
                Err(e) => if e.kind() != ErrorKind::WouldBlock {
                    println!("Connection Error: {}", e);
                } 
            }
        }
    })
}