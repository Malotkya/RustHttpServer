use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::DerefMut,
    pin::Pin,
    io,
    task::{Context, Poll},
    fmt
};

pub trait TargetPipe {
    type Chunk;
    type Error: fmt::Debug;

    fn poll_accept_next(&mut self, ctx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>>;
    fn poll_flush(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T> TargetPipe for Vec<T> {
    type Chunk = T;
    type Error = Infallible;

    fn poll_accept_next(&mut self, _cx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        self.push(chunk);
        Poll::Ready(Ok(()))
    }

    fn poll_flush(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T> TargetPipe for VecDeque<T> {
    type Chunk = T;
    type Error = Infallible;

    fn poll_accept_next(&mut self, _ctx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        self.push_back(chunk);
        Poll::Ready(Ok(()))
    }

    fn poll_flush(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<S: ?Sized + TargetPipe + Unpin> TargetPipe for Box<S> {
    type Chunk = S::Chunk;
    type Error = S::Error;

    fn poll_accept_next(&mut self, cx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        self.as_mut().poll_accept_next(cx, chunk)
    }

    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.as_mut().poll_flush(cx)
    }
}