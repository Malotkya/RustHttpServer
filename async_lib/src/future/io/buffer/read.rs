use crate::future::io::AsyncRead;
use std::io;

pub struct AsyncBufReader<R: AsyncRead> {
    buf: super::AsyncBuffer,
    inner: R
}

impl<R: AsyncRead> AsyncBufReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            buf: super::AsyncBuffer::default(),
            inner
        }
    }

    async fn read(&mut self, buf:&mut [u8]) -> io::Result<usize> {
        self.buf.read_more(&mut self.inner).await?;
        self.buf.read(buf).await
    }

    async fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.buf.read_more(&mut self.inner).await?;
        self.buf.read_vectored(bufs).await
    }

    async fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.buf.read_more(&mut self.inner).await?;
        Ok(self.buf.buffer())
    }

    fn consume(&mut self, amt: usize) {
        self.buf.consume(amt)
    }
}