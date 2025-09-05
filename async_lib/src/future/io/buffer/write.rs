use crate::future::io::AsyncWrite;
use std::io;

pub struct AsyncBufWritter<W: AsyncWrite> {
    inner: W,
    buf: Vec<u8>,
    panicked: bool
}

impl<W: AsyncWrite> AsyncBufWritter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            buf: Vec::with_capacity(super::DEFAULT_BUFFER_SIZE),
            panicked: false,
            inner
        }
    }

    async fn flush_buf(&mut self) -> io::Result<()> {
        let mut guard = BufGuard::new(&mut self.buf);
        while !guard.done() {
            self.panicked = true;
            let r = self.inner.write(guard.remaining()).await;
            self.panicked = false;

            match r {
                Ok(0) => return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write the buffered data"
                )),
                Ok(n) => guard.consume(n),
                Err(e) => return Err(e)
            }
        }

       Ok(())
    }

    async fn write_to_buf(&mut self, buf: &[u8]) -> usize {
        let available = self.spare_capacity();
        let amt_to_buffer = available.min(buf.len());

        unsafe {
            self.write_to_buffer_unchecked(&buf[..amt_to_buffer]).await;
        }

        amt_to_buffer
    }

    async unsafe fn write_to_buffer_unchecked(&mut self, buf: &[u8]) {
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
}

impl Drop for BufGuard<'_> {
    fn drop(&mut self) {
        if self.written > 0 {
            self.buffer.drain(..self.written);
        }
    }
}