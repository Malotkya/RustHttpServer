use std::{
    io,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll}
};
use async_lib_macros::async_trait;

#[async_trait]
pub trait PollWrite {
    fn poll_write(&mut self, cx: &mut Context<'_>, buf: &[u8] ) -> Poll<io::Result<usize>>;
    fn poll_write_vectored(&mut self, cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>] ) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_write(cx, b);
            }
        }

        self.poll_write(cx, &[])
    }

    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>>;
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>>;
}

impl<T: ?Sized + PollWrite + Unpin> PollWrite for Box<T> {
    fn poll_write(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_write(cx, buf)
    }

    fn poll_write_vectored(&mut self, cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_write_vectored(cx, bufs)
    }


    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut **self).poll_flush(cx)
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut **self).poll_close(cx)
    }
}

/*impl<P: DerefMut> PollWrite for Pin<P>
where P::Target: PollWrite {
    fn poll_write(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_write(cx, buf)
    }

    fn poll_write_vectored(&mut self, cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_write_vectored(cx, bufs)
    }

    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_flush(cx)
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_close(cx)
    }
}*/

impl PollWrite for Vec<u8> {
    fn poll_write(&mut self, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_write_vectored(&mut self, _cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write_vectored(&mut *self, bufs))
    }

    fn poll_flush(&mut self, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(&mut self, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl PollWrite for io::Cursor<&mut [u8]> {
    fn poll_write(&mut self, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write(&mut *self, buf))
    }

    fn poll_write_vectored(&mut self, _cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write_vectored(&mut *self, bufs))
    }

    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(io::Write::flush(&mut *self))
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }
}

impl PollWrite for io::Cursor<Vec<u8>> {
    fn poll_write(&mut self, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write(&mut *self, buf))
    }

    fn poll_write_vectored(&mut self, _cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write_vectored(&mut *self, bufs))
    }

    fn poll_flush(&mut self, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(io::Write::flush(&mut *self))
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }
}

impl PollWrite for io::Cursor<Box<[u8]>> {
    fn poll_write(&mut self, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write(&mut *self, buf))
    }

    fn poll_write_vectored(&mut self, _cx: &mut Context<'_>, bufs: &[super::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        Poll::Ready(io::Write::write_vectored(&mut *self, bufs))
    }

    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(io::Write::flush(&mut *self))
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }
}