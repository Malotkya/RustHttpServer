use std::{
    io,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll}
};
use async_lib_macros::async_trait;

#[async_trait]
pub trait PollSeek {
    fn poll_seek(&mut self, cx: &mut Context<'_>, pos: super::SeekFrom) -> Poll<io::Result<u64>>;
}

impl<T: ?Sized + PollSeek + Unpin> PollSeek for Box<T> {
    fn poll_seek(&mut self, cx: &mut Context<'_>, pos: super::SeekFrom) -> Poll<io::Result<u64>>{
       Pin::new(&mut **self).poll_seek(cx, pos)
    }
}

/*impl<P: DerefMut> PollSeek for Pin<P>
where P::Target: PollSeek {
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context<'_>, pos: super::SeekFrom) -> Poll<io::Result<u64>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_seek(cx, pos)
    }
}

impl<T: AsRef<[u8]> + Unpin> PollSeek for io::Cursor<T> {
    fn poll_seek(mut self: Pin<&mut Self>, _cx: &mut Context<'_>, pos: super::SeekFrom) -> Poll<io::Result<u64>> {
        match io::Seek::seek(&mut *self, pos) {
            Ok(_) => Poll::Ready(Ok(self.get_mut().position())),
            Err(e) => Poll::Ready(Err(e))
        }
        
    }
}*/