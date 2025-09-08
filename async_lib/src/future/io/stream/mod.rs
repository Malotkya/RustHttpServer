use std::{
    collections::VecDeque,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll}
};
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

mod pipe;
pub use pipe::{Pipe, TargetPipe, SourcePipe};
mod sink;
pub use sink::Sink;

pub trait Stream {
    type Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<S: ?Sized + Stream + Unpin> Stream for &mut S {
    type Item = S::Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        S::poll_next(&mut **self, cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}

impl<S: ?Sized + Stream + Unpin> Stream for Box<S> {
    type Item = S::Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut **self).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}

impl<S: Stream> Stream for std::panic::AssertUnwindSafe<S> {
    type Item = S::Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        self.0.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> Stream for Vec<T> {
    type Item = T;

    fn poll_next(&mut self, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.pop())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl<T> Stream for VecDeque<T> {
    type Item = T;

    fn poll_next(&mut self, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.pop_front())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

pub trait FusedStream: Stream {
    /// Returns `true` if the stream should no longer be polled.
    fn is_terminated(&self) -> bool;
}

impl<F: ?Sized + FusedStream + Unpin> FusedStream for &mut F {
    fn is_terminated(&self) -> bool {
        <F as FusedStream>::is_terminated(&**self)
    }
}

impl<S: ?Sized + FusedStream + Unpin> FusedStream for Box<S> {
    fn is_terminated(&self) -> bool {
        <S as FusedStream>::is_terminated(&**self)
    }
}
