use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::DerefMut,
    pin::Pin,
    io,
    task::{Context, Poll}
};

use super::TargetPipe;

pub trait Sink : TargetPipe{

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T> Sink for Vec<T> {

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T> Sink for VecDeque<T> {

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<S: ?Sized + Sink + Unpin> Sink for Box<S> {

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.as_mut().poll_ready(cx)
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.as_mut().poll_close(cx)
    }
}