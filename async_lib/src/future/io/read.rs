use std::{
    io,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll}
};
pub trait AsyncRead {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_> ) -> Poll<io::Result<usize>>;
    fn poll_read_vectored<'a>( self: Pin<&'a mut Self>, cx: &'a mut Context<'a>, bufs: &'a mut [super::IoSliceMut<'a>] ) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, &mut ReadBuf::from(b));
            }
        }

        self.poll_read(cx, &mut ReadBuf::new(&mut []))
    }
}

impl<T: ?Sized + AsyncRead + Unpin> AsyncRead for Box<T> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self).poll_read(cx, buf)
    }
}

impl<P: DerefMut> AsyncRead for Pin<P>
where P::Target: AsyncRead {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<usize>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_read(cx, buf)
    }
}

impl AsyncRead for &[u8] {
    fn poll_read(mut self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<usize>> {
        let amt = std::cmp::min(self.len(), buf.remaining());
        let (a, b) = self.split_at(amt);
        buf.put_slice(a);
        *self = b;
        Poll::Ready(Ok(a.len()))
    }
}

impl<T: AsRef<[u8]> + Unpin> AsyncRead for io::Cursor<T> {
    fn poll_read(mut self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<usize>> {
        let pos = self.position();
        let slice: &[u8] = (*self).get_ref().as_ref();

        // The position could technically be out of bounds, so don't panic...
        if pos > slice.len() as u64 {
            return Poll::Ready(Ok(0));
        }

        let start = pos as usize;
        let amt = std::cmp::min(slice.len() - start, buf.remaining());
        // Add won't overflow because of pos check above.
        let end = start + amt;
        buf.put_slice(&slice[start..end]);
        self.set_position(end as u64);

        Poll::Ready(Ok(amt))
    }
}

pub trait AsyncBufRead: AsyncRead {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>>;
    fn consume(self: Pin<&mut Self>, amt: usize);
}

impl<T: ?Sized + AsyncBufRead + Unpin> AsyncBufRead for Box<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Pin::new(&mut **self.get_mut()).poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut **self).consume(amt)
    }
}

impl<P: DerefMut> AsyncBufRead for Pin<P>
where P::Target: AsyncBufRead {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        unsafe { self.get_unchecked_mut() }.as_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        unsafe { self.get_unchecked_mut() }.as_mut().consume(amt);
    }
}

impl AsyncBufRead for &[u8] {
    fn poll_fill_buf(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(Ok(*self))
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        *self = &self[amt..];
    }
}

impl<T: AsRef<[u8]> + Unpin> AsyncBufRead for io::Cursor<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(io::BufRead::fill_buf(self.get_mut()))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        io::BufRead::consume(self.get_mut(), amt);
    }
}

pub struct ReadBuf<'a> {
    buf: &'a mut [u8],
    filled: usize,
    initialized: usize,
}

impl<'a> ReadBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            initialized: buf.len(),
            buf,
            filled: 0
        }
    }

    pub fn from(slice: &'a mut super::IoSliceMut<'a>) -> Self {
        let buf = slice.as_mut_array().unwrap_or(&mut []);
        Self {
            initialized: buf.len(),
            buf,
            filled: 0
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.capacity() - self.filled
    }
    

    pub fn filled(&self) -> &[u8] {
        &self.buf[..self.filled]
    }

    pub fn filled_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.filled]
    }

    pub fn take(&mut self, n: usize) -> ReadBuf<'_> {
        let max = std::cmp::min(self.remaining(), n);
        // Safety: We don't set any of the `unfilled_mut` with `MaybeUninit::uninit`.
        ReadBuf::new(&mut self.filled_mut()[..max])
    }

    pub fn initialized(&self) -> &[u8] {
        &self.buf[..self.initialized]
    }

    pub fn initialized_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.initialized]
    }

    pub fn inner_mut(&mut self) -> &mut [u8] {
        self.buf
    }

    pub fn unfilled_mut(&mut self) -> &mut [u8] {
        &mut self.buf[self.filled..]
    }

    pub fn initialize_unfilled(&mut self) -> &mut [u8] {
        self.initialize_unfilled_to(self.remaining())
    }

    pub fn initialize_unfilled_to(&mut self, n: usize) -> &mut [u8] {
        assert!(self.remaining() >= n, "n overflows remaining");

        let end = self.filled + n;
        if self.initialized < end {
            unsafe {
                self.buf[self.initialized..end]
                    .as_mut_ptr()
                    .write_bytes(0, end - self.initialized);
            }
            self.initialized = end;
        }

        &mut self.buf[self.filled..end]
    }

    pub fn clear(&mut self) {
        self.filled = 0;
    }

    pub fn advance(&mut self, n: usize) {
        let new = self.filled.checked_add(n).expect("filled overflow");
        self.set_filled(new);
    }

    pub fn set_filled(&mut self, n: usize) {
        assert!(
            n <= self.initialized,
            "filled must not become larger than initialized"
        );
        self.filled = n;
    }
    
    pub unsafe fn assume_init(&mut self, n: usize) {
        let new = self.filled + n;
        if new > self.initialized {
            self.initialized = new;
        }
    }

    pub fn put_slice(&mut self, buf: &[u8]) {
        assert!(
            self.remaining() >= buf.len(),
            "buf.len() must fit in remaining(); buf.len() = {}, remaining() = {}",
            buf.len(),
            self.remaining()
        );

        let amt = buf.len();
        // Cannot overflow, asserted above
        let end = self.filled + amt;

        // Safety: the length is asserted above
        unsafe {
            self.buf[self.filled..end]
                .as_mut_ptr()
                .cast::<u8>()
                .copy_from_nonoverlapping(buf.as_ptr(), amt);
        }

        if self.initialized < end {
            self.initialized = end;
        }
        self.filled = end;
    }
}