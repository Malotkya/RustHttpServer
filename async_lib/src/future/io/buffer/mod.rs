use std::{
    cmp,
    io,
    mem::MaybeUninit,
    task::{Poll, Context},
    pin::{Pin, pin}
};
pub(crate) use super::{AsyncRead, AsyncWrite};
pub(crate) use async_lib_macros::async_fn;

mod read;
pub use read::AsyncBufReader;
mod write;
pub use write::AsyncBufWritter;

pub(crate) const DEFAULT_BUFFER_SIZE:usize = 8 * 1024;

pub(crate) struct AsyncBuffer {
    buf: Box<[MaybeUninit<u8>]>,
    pos: usize,
    filled: usize,
}

impl AsyncBuffer {
    pub fn with_capacitry(capacity: usize) -> Self {
        Self {
            buf: Box::new_uninit_slice(capacity),
            pos: 0,
            filled: 0,
        }
    }

    pub fn default() -> Self {
        Self::with_capacitry(DEFAULT_BUFFER_SIZE)
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.buf.get_unchecked(self.pos..self.filled).assume_init_ref() }
    }

    pub unsafe fn mut_buffer(&mut self) -> &mut [u8] {
        unsafe{ self.buf.get_unchecked_mut(self.pos..self.filled).assume_init_mut()}
    }

    pub fn size(&self) -> usize {
        self.filled - self.pos
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.capacity() - self.filled
    }

    pub unsafe fn unfilled(&self) -> &[u8] {
        unsafe { self.buf.get_unchecked(self.filled+1..).assume_init_ref() }
    }

    pub unsafe fn unfilled_mut(&mut self) -> &mut [u8] {
        unsafe { self.buf.get_unchecked_mut(self.filled+1..).assume_init_mut() }
    }

    pub fn clear(&mut self) {
        self.filled = 0;
        self.pos = 0;
    }

    pub fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.filled);
    }

    pub fn unconsume(&mut self, amt: usize) {
        self.pos = self.pos.saturating_sub(amt);
    }

    pub fn backshift(&mut self) {
        self.buf.copy_within(self.pos..self.filled, 0);
        self.filled -= self.pos;
        self.pos = 0;
    }

    #[async_fn]
    pub fn poll_read_more<R: AsyncRead>(mut self:Pin<&mut Self>, cx:&mut Context<'_>, reader:&mut R) -> Poll<io::Result<usize>> {
        let buf = unsafe{ self.unfilled_mut() };
        let amt = match pin!(reader.read(buf)).poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(r) => match r {
                Ok(size) => size,
                Err(e) => return Poll::Ready(Err(e))
            }
        };
        self.filled += amt;
        Poll::Ready(Ok(amt))
    }
}

impl AsyncRead for AsyncBuffer {
    fn poll_read(self:Pin<&mut Self>, _cx: &mut std::task::Context<'_> ,buf: &mut [u8]) -> Poll<io::Result<usize> > {
        let amt = cmp::min(buf.len(), self.size());
        unsafe {
            buf.as_mut_ptr()
                .cast::<u8>()
                .copy_from_nonoverlapping(
                    self.buffer().as_ptr(),
                    amt
                );
        }

        Poll::Ready(Ok(amt))
    }
}
