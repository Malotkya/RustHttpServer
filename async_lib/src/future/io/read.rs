use std::{
    io,
    ops::DerefMut,
    pin::{Pin, pin},
    task::{Context, Poll}
};
use async_lib_macros::async_trait;

#[async_trait]
pub trait AsyncRead: Sized + Unpin{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8] ) -> Poll<io::Result<usize>>;
    fn poll_read_vectored(self:Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [super::IoSliceMut<'_>] ) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }

        self.poll_read(cx, &mut [])
    }
}

impl<T: AsyncRead> AsyncRead for &mut T {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read(cx, buf)
    }

    fn poll_read_vectored(mut self:Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [super::IoSliceMut<'_>] ) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read_vectored(cx, bufs)
    }
}

impl<T: AsyncRead> AsyncRead for Box<T> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read(cx, buf)
    }

    fn poll_read_vectored(mut self:Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [super::IoSliceMut<'_>] ) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read_vectored(cx, bufs)
    }
}

impl<P: DerefMut + Unpin> AsyncRead for Pin<P> where P::Target: AsyncRead {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_read(cx, buf)
    }

    fn poll_read_vectored(self:Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [super::IoSliceMut<'_>] ) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_read_vectored(cx, bufs)
    }
}

impl AsyncRead for &[u8] {
    fn poll_read(mut self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
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

impl<T: AsRef<[u8]> + Unpin> AsyncRead for io::Cursor<T> {
    fn poll_read(mut self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
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

pub trait AsyncBufRead: AsyncRead {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>>;

    fn fill_buf(&mut self) -> impl Future <Output = io::Result<&[u8]>> {
        let mut pin = Pin::new(self);
        let future = std::future::poll_fn(move |cx|{
            pin.as_mut().poll_fill_buf(cx).map(|result|
                result.map(|ptr|(ptr.as_ptr() as usize, ptr.len()))
            )
        });

        async {
            future.await.map(|(ptr, len)|
                unsafe{ std::slice::from_raw_parts(ptr as *const u8, len)}
            )
        }
    }

    fn consume(&mut self, amt: usize);
}

impl<T: AsyncBufRead + Unpin> AsyncBufRead for Box<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Pin::new(&mut **self.get_mut()).poll_fill_buf(cx)
    }

    fn consume(&mut self, amt: usize) {
        self.as_mut().consume(amt)
    }
}

impl<T: AsyncBufRead + Unpin> AsyncBufRead for &mut T {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Pin::new(&mut **self.get_mut()).poll_fill_buf(cx)
    }

    fn consume(&mut self, amt: usize) {
        (*self).consume(amt)
    }
}

impl<P:DerefMut + Unpin> AsyncBufRead for Pin<P> where P::Target: AsyncBufRead {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_mut().as_mut().poll_fill_buf(cx)
    }

    fn consume(&mut self, amt: usize) {
        (&mut **self).consume(amt);
    }
}

impl AsyncBufRead for &[u8] {
    fn poll_fill_buf(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(Ok(*self))
    }

    fn consume(&mut self, amt: usize) {
        *self = &self[amt..];
    }
}

/*impl<T: AsRef<[u8]> + Unpin > AsyncBufRead for io::Cursor<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(io::BufRead::fill_buf(self.get_mut()))
    }

    fn consume(&mut self, amt: usize) {
        io::BufRead::consume(self.get_mut(), amt);
    }
}*/
