use crate::future::io::{
    stream::{Sink, TargetPipe},
    PollWrite,
    Stream
};
use std::{
    io,
    task::{Context, Poll}
};

pub struct AsyncBufWritter<W: PollWrite> {
    inner: W,
    buf: Vec<u8>,
    panicked: bool
}

impl<W: PollWrite> AsyncBufWritter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            buf: Vec::with_capacity(super::DEFAULT_BUFFER_SIZE),
            panicked: false,
            inner
        }
    }

    fn poll_flush_buf(&mut self, cx:&mut Context<'_>) -> Poll<io::Result<()>> {
        let mut guard = BufGuard::new(&mut self.buf);
        while !guard.done() {
            self.panicked = true;
            let r = match self.inner.poll_write(cx, guard.remaining()) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(r) => r
            };
            self.panicked = false;

            match r {
                Ok(0) => return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write the buffered data"
                ))),
                Ok(n) => guard.consume(n),
                Err(e) => return Poll::Ready(Err(e))
            }
        }

        guard.clear();
       Poll::Ready(Ok(()))
    }

    fn flush_buf(&mut self) -> impl Future<Output=io::Result<()>> {
        std::future::poll_fn(|cx|{
            self.poll_flush_buf(cx)
        })
    }

    pub fn poll_write_to_buf(&mut self, _cx:&mut Context<'_>, buf: &[u8]) -> Poll<usize> {
        let available = self.spare_capacity();
        let amt_to_buffer = available.min(buf.len());

        unsafe {
            self.write_to_buffer_unchecked(&buf[..amt_to_buffer]);
        }

        Poll::Ready(amt_to_buffer)
    }

    fn write_to_buffer(&mut self, buf: &[u8]) -> impl Future<Output = usize> {
        std::future::poll_fn(|cx|{
            self.poll_write_to_buf(cx, buf)
        })
    }

    unsafe fn write_to_buffer_unchecked(&mut self, buf: &[u8]) {
        debug_assert!(buf.len() <= self.spare_capacity());
        let old_len = self.buf.len();
        let buf_len: usize = buf.len();
        let src = buf.as_ptr();

        unsafe {
            let dst = self.buf.as_mut_ptr().add(old_len);
            std::ptr::copy_nonoverlapping(src, dst, buf_len);
            self.buf.set_len(old_len + buf_len);
        }

    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    fn spare_capacity(&self) -> usize {
        self.buf.capacity() - self.buf.len()
    }
}

impl<W: PollWrite> TargetPipe for AsyncBufWritter<W> {
    type Chunk = Vec<u8>;
    type Error = io::Error;

    fn poll_accept_next(&mut self, ctx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        match self.poll_write_to_buf(ctx, &chunk) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(size) => if size < chunk.len() {
                Poll::Ready(Err(
                    io::Error::new(
                        io::ErrorKind::OutOfMemory,
                        "Write Buffer has run out of memory!"
                    )
                ))
            } else {
                Poll::Ready(Ok(()))
            }
        }
    }

    fn poll_flush(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_flush_buf(ctx)
    }
}

impl<W: PollWrite> Sink for AsyncBufWritter<W> {
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.panicked {
            Poll::Ready(Err(
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Sink has panicked"
                )
            ))
        } else {
            Poll::Ready(Ok(()))
        }
            
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_flush(cx)
    }
}

struct BufGuard<'a> {
    buffer: &'a mut Vec<u8>,
    written: usize
}

impl<'a> BufGuard<'a> {
    fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self { buffer, written: 0 }
    }


    fn remaining(&self) -> &[u8] {
        &self.buffer[self.written..]
    }


    fn consume(&mut self, amt: usize) {
        self.written += amt;
    }


    fn done(&self) -> bool {
        self.written >= self.buffer.len()
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }
}

impl Drop for BufGuard<'_> {
    fn drop(&mut self) {
        if self.written > 0 {
            self.buffer.drain(..self.written);
        }
    }
}