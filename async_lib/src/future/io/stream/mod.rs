use std::{
    collections::VecDeque,
    pin::{Pin, pin},
    task::{Context, Poll}
};
use async_lib_macros::{async_fn, async_trait};

mod pipe;
pub use pipe::{Pipe, TargetPipe, SourcePipe};
mod sink;
pub use sink::Sink;

pub trait Stream {
    type Item;

    #[async_fn]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<S: ?Sized + Stream + Unpin> Stream for Box<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut **self).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}

impl<S: Stream + Unpin> Stream for std::panic::AssertUnwindSafe<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new( &mut self.get_mut().0 ).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> Stream for Vec<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(unsafe { self.get_unchecked_mut() }.pop())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl<T> Stream for VecDeque<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(unsafe { self.get_unchecked_mut() }.pop_front())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

pub trait FusedStream: Stream {
    /// Returns `true` if the stream should no longer be polled.
    fn is_terminated(&self) -> bool;
}

impl<S: ?Sized + FusedStream + Unpin> FusedStream for Box<S> {
    fn is_terminated(&self) -> bool {
        <S as FusedStream>::is_terminated(&**self)
    }
}
