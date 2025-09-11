use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::{DerefMut, Deref},
    pin::{Pin, pin},
    io,
    task::{Context, Poll},
    fmt
};
use async_lib_macros::async_fn;
use crate::future::Done;


pub trait TargetPipe {
    type Chunk;
    type Error: fmt::Debug;


    /// ToDo: Figure out for this to not be backwards!! ///
    fn poll_accept_next(self: Pin<&mut Self>, ctx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>>{
        pin!( unsafe{ self.get_unchecked_mut() }.accept_next(chunk) ).poll(ctx)
    }
    
    async fn accept_next(self:&mut Self, chunk: Self::Chunk) -> Result<(),Self::Error>;

    #[async_fn]
    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T> TargetPipe for Vec<T> {
    type Chunk = T;
    type Error = Infallible;

    fn accept_next(self:&mut Self, chunk: Self::Chunk) -> impl Future<Output = Result<(),Self::Error>> {
        self.push(chunk);
        Done::new(Ok(()))
    }

    fn poll_accept_next(self: Pin<&mut Self>, _cx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        unsafe { self.get_unchecked_mut() }.push(chunk);
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T> TargetPipe for VecDeque<T> {
    type Chunk = T;
    type Error = Infallible;

    fn accept_next(self:&mut Self, chunk: Self::Chunk) -> impl Future<Output = Result<(),Self::Error>> {
        self.push_back(chunk);
        Done::new(Ok(()))
    }
    
    fn poll_accept_next(self: Pin<&mut Self>, _cx: &mut Context<'_>, chunk:Self::Chunk) -> Poll<Result<(), Self::Error>> {
        unsafe { self.get_unchecked_mut() }.push_back(chunk);
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<S: TargetPipe + Unpin> TargetPipe for Box<S> {
    type Chunk = S::Chunk;
    type Error = S::Error;

     fn accept_next(self:&mut Self, chunk: Self::Chunk) -> impl Future<Output = Result<(),Self::Error>> {
        self.as_mut().accept_next(chunk)
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(unsafe{ self.get_unchecked_mut() }.as_mut()).poll_flush(ctx)
    }
}