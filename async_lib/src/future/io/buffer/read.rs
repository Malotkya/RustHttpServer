use crate::future::io::PollRead;
use crate::io::stream::{Stream, SourcePipe, Pipe};
use std::{
    io,
    task::{Context, Poll}
};

pub struct AsyncBufReader<R: PollRead> {
    buf: super::AsyncBuffer,
    inner: R
}

impl<R: PollRead> AsyncBufReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            buf: super::AsyncBuffer::default(),
            inner
        }
    }

    pub fn poll_read(&mut self, cx: &mut Context<'_>, buf:&mut [u8]) -> Poll<io::Result<usize>> {
        if self.buf.poll_read_more(cx, &mut self.inner).is_ready() {
            self.buf.poll_read(cx, buf)
        } else {
            Poll::Pending
        }
    }

    pub fn read(&mut self, buf:&mut [u8]) -> impl Future<Output = io::Result<usize>> {
        std::future::poll_fn(move |cx|{
            self.poll_read(cx, buf)
        })
    }

    pub fn poll_read_vectored(&mut self, cx: &mut Context<'_>, bufs: &mut [io::IoSliceMut<'_>]) -> Poll<io::Result<usize>> {
        if self.buf.poll_read_more(cx, &mut self.inner).is_ready() {
            self.buf.poll_read_vectored(cx, bufs)
        } else {
            Poll::Pending
        }
    }

    pub fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> impl Future<Output = io::Result<usize>> {
        std::future::poll_fn(move |cx|{
            self.poll_read_vectored(cx, bufs)
        })
    }

    pub fn poll_fill_buf(&mut self, cx:&mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        if self.buf.poll_read_more(cx, &mut self.inner).is_ready() {
            Poll::Ready(Ok(self.buf.buffer()))
        } else {
            Poll::Pending
        }
    }

    pub fn fill_buf(&mut self) -> impl Future<Output = io::Result<&[u8]>> {
        let future = std::future::poll_fn(|cx|{
            self.poll_fill_buf(cx).map(|result|result.map(|ptr|(ptr.as_ptr() as usize, ptr.len())))
        });

        let combine = async||{
            future.await.map(|(ptr, len)| unsafe{ std::slice::from_raw_parts(ptr as *const u8, len)})
        };

        combine()
    }

    pub fn consume(&mut self, amt: usize) {
        self.buf.consume(amt)
    }

    pub fn buffer(&self) -> &[u8] {
        self.buf.buffer()
    }
}

impl<R: PollRead> Stream for AsyncBufReader<R> {
    type Item = Vec<u8>;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {

        if let Poll::Ready(r) = self.buf.poll_read_more(cx, &mut self.inner) {
            r.unwrap();
            let size = self.buf.size();
            if size == 0 {
                Poll::Ready(None)
            } else {
                let vec = unsafe{
                    let slice = self.buf.mut_buffer();
                    Vec::from_parts(
                        std::ptr::NonNull::new_unchecked(slice.as_mut_ptr()),
                        size, 
                        size
                    )
                };
                self.buf.consume(size);
                Poll::Ready(Some(
                    vec
                ))
            }
        } else {
            Poll::Pending
        }
    }
}

impl<R: PollRead + 'static> SourcePipe for AsyncBufReader<R> {
    type Chunk = Self::Item;

    fn pipe<P: crate::io::stream::TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> crate::io::stream::Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}