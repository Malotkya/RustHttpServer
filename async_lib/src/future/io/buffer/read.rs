use super::{AsyncRead, AsyncBuffer, async_fn};
use crate::io::stream::{Stream, SourcePipe, Pipe};

use std::{
    io,
    task::{Context, Poll},
    pin::{Pin, pin}
};

pub struct AsyncBufReader<R: AsyncRead> {
    buf: AsyncBuffer,
    inner: R
}

impl<R: AsyncRead + Unpin> AsyncBufReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            buf: super::AsyncBuffer::default(),
            inner
        }
    }

    #[async_fn]
    pub fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buffer:&mut [u8]) -> Poll<io::Result<usize>> {
        let Self{ buf, inner} = &mut (*self);

        if pin!(buf.read_more(inner)).poll(cx).is_ready() {
            Pin::new(&mut buf.read(buffer)).poll(cx)
        } else {
            Poll::Pending
        }
    }

    #[async_fn]
    pub fn poll_read_vectored(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [io::IoSliceMut<'_>]) -> Poll<io::Result<usize>> {
        let Self{ buf, inner} = &mut (*self);

        if pin!(buf.read_more(inner)).poll(cx).is_ready() {
            pin!(buf.read_vectored(bufs)).poll(cx)
        } else {
            Poll::Pending
        }
    }

    pub fn poll_fill_buf(mut self: Pin<&mut Self>, cx:&mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let Self{ buf, inner} = &mut (*self);

        if pin!(buf.read_more(inner)).poll(cx).is_ready() {
            let slice = unsafe{ buf.mut_buffer() };
            Poll::Ready(Ok(
                unsafe{
                    std::slice::from_raw_parts(
                        slice.as_mut_ptr(),
                        slice.len()
                    )
                }
            ))
        } else {
            Poll::Pending
        }
    }

    pub fn fill_buf(&mut self) -> impl Future<Output = io::Result<&[u8]>> {
        let mut pin = Pin::new(self);
        let future = std::future::poll_fn(move |cx|{
            pin.as_mut().poll_fill_buf(cx).map(|result|result.map(|ptr|(ptr.as_ptr() as usize, ptr.len())))
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

impl<R: AsyncRead> Stream for AsyncBufReader<R> {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self{ buf, inner} = &mut (*self);
        let pin = Pin::new(&mut *buf);

        if let Poll::Ready(r) = pin.poll_read_more(cx, inner) {
            r.unwrap();
            let size = buf.size();
            if size == 0 {
                Poll::Ready(None)
            } else {
                let vec = unsafe{
                    let slice = buf.mut_buffer();
                    Vec::from_raw_parts(
                        slice.as_mut_ptr(),
                        size, 
                        size
                    )
                };
                buf.consume(size);
                Poll::Ready(Some(
                    vec
                ))
            }
        } else {
            Poll::Pending
        }
    }
}

impl<R: AsyncRead + 'static> SourcePipe for AsyncBufReader<R> {
    type Chunk = Self::Item;

    fn pipe<P: crate::io::stream::TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> crate::io::stream::Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}