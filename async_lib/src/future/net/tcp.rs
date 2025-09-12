use std::{
    net,
    time::Duration,
    task::{Context, Poll},
    async_iter::AsyncIterator,
    pin::Pin
};
use crate::io;
use async_lib_macros::{deref_inner_async, async_fn};

const STOP_BLOCK_ATTEMPT:u8 = 10;
const READ_TIMEOUT:Duration = Duration::from_millis(500);
const WRITE_TIMEOUT:Duration = Duration::from_secs(1);

pub struct TcpListener {
    io: net::TcpListener,
    nonblocking: io::Result<bool>
}

impl TcpListener {
    pub fn bind<A: net::ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Self::from(
            net::TcpListener::bind(addr)?
        )
    }

    fn from(io: net::TcpListener) -> io::Result<Self> {
        let mut nonblocking = Ok(false);

        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_nonblocking(true) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                nonblocking = Err(e);
                break;
            }
        }
        
        Ok(Self{
            io,
            nonblocking
        })
    }

    pub fn local_addr(&self) -> io::Result<super::SocketAddr> {
        self.io.local_addr()
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) -> io::Result<()>{
        match &self.nonblocking {
            Ok(_) => {
                self.nonblocking = Ok(nonblocking);
                Ok(())
            },
            Err(e) => Err(
                io::Error::new(
                    e.kind(),
                    e.to_string()
                )
            )
        }
    }

    #[async_fn]
    pub fn poll_accept(self: Pin<&mut Self>, cx:&Context<'_>) -> Poll<io::Result<(TcpStream, super::SocketAddr)>> {
        match self.io.accept() {
            Ok((inner, addr)) => Poll::Ready(
                TcpStream::from(inner).map(|stream|{
                    (stream, addr)
                })
            ),
            Err(e) => if *(self.nonblocking.as_ref().unwrap_or(&false))
                                && e.kind() == io::ErrorKind::WouldBlock {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(Err(e))
            }
        }
    }

    pub fn sync_accept(&self) -> io::Result<(TcpStream, super::SocketAddr)> {
        match self.io.accept() {
            Ok((inner, addr)) => Ok((
                TcpStream::from(inner)?,
                addr
            )),
            Err(e) => Err(e)
        }
    }

    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming { listener: self }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self{
            io: self.io.try_clone()?,
            nonblocking: super::clone_io_result(&self.nonblocking)
        })
    }
}

pub struct Incoming<'a> {
    listener: &'a mut TcpListener
}

impl<'a> AsyncIterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<io::Result<TcpStream>>> {
        let pin = Pin::new(&mut *self.listener);
        if let Poll::Ready(result) = pin.poll_accept(cx) {
            Poll::Ready(Some(
                result.map(|conn|conn.0)
            ))
        } else {
            Poll::Pending
        }
    }
}

#[deref_inner_async(Read, Write)]
pub struct TcpStream {
    io: net::TcpStream
}

impl TcpStream {
    fn from(io: std::net::TcpStream) -> io::Result<Self> {
        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_nonblocking(true) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                return Err(e)
            }
        }

        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_nodelay(true) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                return Err(e)
            }
        }

        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_read_timeout(Some(READ_TIMEOUT)) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                return Err(e)
            }
        }

        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_write_timeout(Some(WRITE_TIMEOUT)) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                return Err(e)
            }
        }

        Ok(Self{io})
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self{
            io: self.io.try_clone()?
        })
    }
}