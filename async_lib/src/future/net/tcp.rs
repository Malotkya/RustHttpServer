use std::{
    io,
    net,
    time::Duration
};
use async_lib_macros::deref_inner_async;

const STOP_BLOCK_ATTEMPT:u8 = 10;
const READ_TIMEOUT:Duration = Duration::from_millis(500);
const WRITE_TIMEOUT:Duration = Duration::from_secs(1);

pub struct TcpListener {
    io: net::TcpListener
}

impl TcpListener {
    pub fn bind<A: net::ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let io = net::TcpListener::bind(addr)?;
        
        let mut attmpt:u8 = 0;
        while let Err(e) = io.set_nonblocking(true) {
            attmpt += 1;
            if attmpt > STOP_BLOCK_ATTEMPT {
                return Err(e)
            }
        }

        
        Ok(Self{ io })
    }

    pub fn local_addr(&self) -> io::Result<super::SocketAddr> {
        self.io.local_addr()
    }

    pub fn accept(&self) -> io::Result<Option<(TcpStream, super::SocketAddr)>> {
        match self.io.accept() {
            Ok(s) => {
                let (inner, addr) = s;
                Ok(Some((
                    TcpStream::from(inner)?,
                    addr
                )))
            },
            Err(e) => if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming { listener: self }
    }
}

pub struct Incoming<'a> {
    listener: &'a TcpListener
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;

    fn next(&mut self) -> Option<io::Result<TcpStream>> {
        match self.listener.accept() {
            Ok(o) => match o {
                Some(s) => Some(Ok(s.0)),
                None => None
            },
            Err(e) => Some(Err(e))
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
}