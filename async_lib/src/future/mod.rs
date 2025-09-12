use std::ops::DerefMut;

pub mod fs;
pub mod io;
pub mod net;
mod promise;

pub use promise::Promise;

pub(crate) fn clone_io_result<T: Clone>(result: &io::Result<T>) -> io::Result<T> {
    match result {
        Ok(t) => Ok(t.clone()),
        Err(e) => Err(
            std::io::Error::new(
                e.kind(),
                e.to_string()
            )
        )
    }
}

pub(crate) struct Done<T>(Option<T>);

impl<T> Done<T>  {
    fn new(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> Future for Done<T> {
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match unsafe{ self.get_unchecked_mut() }.deref_mut().0.take() {
            Some(v) => std::task::Poll::Ready(v),
            None => std::task::Poll::Pending
        }
        
    }
}