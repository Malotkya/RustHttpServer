use std::{
    task::{Context, Poll},
    pin::Pin,
};

mod promise;
mod io;
mod stream;
mod sink;
mod net;

trait SliceCallback = FnMut(&mut Context<'_>) -> Poll<(usize, usize)>;
trait SliceOptionCallback = FnMut(&mut Context<'_>) -> Poll<Option<(usize, usize)>>;
trait SliceResultCallback<E> = FnMut(&mut Context<'_>) -> Poll<Result<(usize, usize), E>>;

pub(crate) struct PollSlice<'t>{
    pub func: Box<dyn SliceCallback + 't>
}

impl<'t> Future for PollSlice<'t> {
    type Output = &'t [u8];

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'t [u8]> {
        (self.func)(cx).map(|(ptr, len)|{
            unsafe {
                std::slice::from_raw_parts(
                    ptr as *const u8, 
                    len
                )
            }
        })
    }

}

pub(crate) struct PollOptionSlice<'t> {
    pub func: Box<dyn SliceOptionCallback + 't>
}

impl<'t> Future for PollOptionSlice<'t> {
    type Output = Option<&'t [u8]>;

    fn poll<'a>(mut self: Pin<&'a mut Self>, cx: &'a mut Context<'_>)  -> Poll<Option<&'t [u8]>> {
        (self.func)(cx).map(|option|option.map(|(ptr, len)|{
            unsafe {
                std::slice::from_raw_parts(
                    ptr as *const u8, 
                    len
                )
            }
        }))
    }
}

pub(crate) struct PollResultSlice<'t, E> {
    pub func: Box<dyn SliceResultCallback<E> + 't>
}

impl<'t, E> Future for PollResultSlice<'t, E> {
    type Output = Result<&'t [u8], E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)  -> Poll<Result<&'t [u8], E>> {
        (self.func)(cx).map(|option|option.map(|(ptr, len)|{
            unsafe {
                std::slice::from_raw_parts(
                    ptr as *const u8, 
                    len
                )
            }
        }))
    }

}