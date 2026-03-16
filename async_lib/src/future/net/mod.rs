use crate::{
    executor::{
        ThreadJob,
        ThreadPoolConnection,
    },
    io::{
        ErrorKind, Result
    }
};

pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
pub(crate) use super::clone_io_result;

mod tcp;
pub use tcp::*;

pub fn tcp_listener_thread<A:std::net::ToSocketAddrs>(addr:A, callback: impl Fn(TcpStream) + Sync + Send + 'static) -> Result<impl ThreadJob> {
    let mut listener = TcpListener::bind(addr)?;

    if listener.set_nonblocking(true).is_err() {
        println!("Failed to set nonblocking on TcpListener!");
    }

    Ok(move |conn:ThreadPoolConnection|{
        while conn.running() {
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