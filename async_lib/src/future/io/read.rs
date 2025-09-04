use std::{
    io,
    net::{TcpListener, TcpStream},
    ops::DerefMut,
    pin::{Pin, pin},
    task::{Context, Poll}
};
use async_lib_macros::async_trait;

#[async_trait]
pub trait PollRead {
    fn poll_read(&mut self, cx: &mut Context<'_>, buf: &mut [u8] ) -> Poll<io::Result<usize>>;
    fn poll_read_vectored(&mut self, cx: &mut Context<'_>, bufs: &mut [super::IoSliceMut<'_>] ) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }

        self.poll_read(cx, &mut [])
    }
}

impl<T: ?Sized + PollRead + Unpin> PollRead for Box<T> {
    fn poll_read(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read(cx, buf)
    }
}

impl PollRead for &[u8] {
    fn poll_read(&mut self, _cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        let amt = std::cmp::min(self.len(), buf.len());
        let (a, b) = self.split_at(amt);
        unsafe {
            buf[..amt]
                .as_mut_ptr()
                .cast::<u8>()
                .copy_from_nonoverlapping(a.as_ptr(), amt);
        }
        *self = b;
        Poll::Ready(Ok(amt))
    }
}

impl<T: AsRef<[u8]> + Unpin> PollRead for io::Cursor<T> {
    fn poll_read(&mut self, _cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        let pos = self.position();
        let slice: &[u8] = (*self).get_ref().as_ref();

        // The position could technically be out of bounds, so don't panic...
        if pos > slice.len() as u64 {
            return Poll::Ready(Ok(0));
        }

        let start = pos as usize;
        let amt = std::cmp::min(slice.len() - start, buf.len());
        // Add won't overflow because of pos check above.
        let end = start + amt;
        unsafe {
            buf[..amt]
                .as_mut_ptr()
                .cast::<u8>()
                .copy_from_nonoverlapping(slice[start..end].as_ptr(), amt);
        }
        self.set_position(end as u64);

        Poll::Ready(Ok(amt))
    }
}

pub trait PollBufRead: PollRead {
    fn poll_fill_buf(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<(usize, usize)>>;
    //fn consume(&mut self, amt: usize);
}

pub trait AsyncBufRead : PollBufRead + Sized{
    fn fill_buf(&mut self) -> impl Future <Output = io::Result<&[u8]>> {
        //let pin = 

        crate::PollResultSlice{
            func: Box::new(|cx|{
                self.poll_fill_buf(cx)
            })
        }
    }

    fn consume(& mut self, amt : usize);
}

impl<T: ?Sized + PollBufRead + Unpin> PollBufRead for Box<T> {
    fn poll_fill_buf(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<(usize, usize)>> {
        self.as_mut().poll_fill_buf(cx)
    }
}

impl<T: ?Sized + AsyncBufRead + Unpin> AsyncBufRead for Box<T> {
    fn consume(&mut self, amt: usize) {
        Pin::new(&mut **self).consume(amt)
    }
}

impl PollBufRead for &[u8] {
    fn poll_fill_buf(&mut self, _cx: &mut Context<'_>) -> Poll<io::Result<(usize, usize)>> {
        Poll::Ready(Ok((self.as_ptr() as usize, self.len())))
    }
}

impl AsyncBufRead for &[u8] {
    fn consume(&mut self, amt: usize) {
        *self = &self[amt..];
    }
}

/*
impl<T: AsRef<[u8]> + Unpin> PollBufRead for io::Cursor<T> {
    fn poll_fill_buf(&mut self, _cx: &mut Context<'_>) -> Poll<io::Result<(usize, usize)>> {
        let slice = io::BufRead::fill_buf(self.get_mut());
        Poll::Ready(Ok(
            slice.as_ptr() as usize,
            slice.len()
        ))
    }
}

impl<T: AsRef<[u8]> + Unpin> AsyncBufRead for io::Cursor<T> {
    fn consume(&mut self, amt: usize) {
        io::BufRead::consume(self.get_mut(), amt);
    }
}*/