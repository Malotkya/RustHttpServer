use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::DerefMut,
    pin::Pin,
    io,
    task::{Context, Poll}
};
use async_lib_macros::async_trait;

use super::TargetPipe;

#[async_trait]
pub trait Sink : TargetPipe{

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T> Sink for Vec<T> {

    fn poll_ready(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self:Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T> Sink for VecDeque<T> {

    fn poll_ready(self:Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self:Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<S: Sink + Unpin> Sink for Box<S> {

    fn poll_ready(self:Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(unsafe{ self.get_unchecked_mut() }.as_mut()).poll_ready(cx)
    }

    fn poll_close(self:Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(unsafe{ self.get_unchecked_mut() }.as_mut()).poll_close(cx)
    }
}