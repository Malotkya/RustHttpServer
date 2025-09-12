use std::{
    collections::VecDeque,
    convert::Infallible,
    pin::{Pin, pin},
    task::{Context, Poll},
    fmt,
};
use async_lib_macros::async_fn;
use crate::future::Done;


pub trait TargetPipe {
    type Chunk;
    type Error: fmt::Debug;

    fn poll_accept_next(self: Pin<&mut Self>, ctx: &mut Context<'_>, chunk:&Self::Chunk) -> Poll<Result<(), Self::Error>>;
    
    fn accept_next(self:&mut Self, chunk: Self::Chunk) -> impl Future<Output = Result<(),Self::Error>> {
        let mut pin = unsafe{ Pin::new_unchecked(self) };

        std::future::poll_fn(move |cx|{
            pin.as_mut().poll_accept_next(cx, &chunk)
        })
    }

    #[async_fn]
    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

impl<T:Clone> TargetPipe for Vec<T> {
    type Chunk = T;
    type Error = Infallible;

    fn poll_accept_next(self: Pin<&mut Self>, _cx: &mut Context<'_>, chunk:&Self::Chunk) -> Poll<Result<(), Self::Error>> {
        unsafe { self.get_unchecked_mut() }.push(chunk.clone());
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T:Clone> TargetPipe for VecDeque<T> {
    type Chunk = T;
    type Error = Infallible;

    fn accept_next(self:&mut Self, chunk: Self::Chunk) -> impl Future<Output = Result<(),Self::Error>> {
        self.push_back(chunk);
        Done::new(Ok(()))
    }
    
    fn poll_accept_next(self: Pin<&mut Self>, _cx: &mut Context<'_>, chunk:&Self::Chunk) -> Poll<Result<(), Self::Error>> {
        unsafe { self.get_unchecked_mut() }.push_back(chunk.clone());
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<S: TargetPipe + Unpin> TargetPipe for Box<S> {
    type Chunk = S::Chunk;
    type Error = S::Error;

    fn poll_accept_next(self: Pin<&mut Self>, cx: &mut Context<'_>, chunk: &Self::Chunk) -> Poll<Result<(),Self::Error>> {
        Pin::new(self.get_mut().as_mut()).poll_accept_next(cx, chunk)
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(unsafe{ self.get_unchecked_mut() }.as_mut()).poll_flush(ctx)
    }
}