use std::{
    sync::{
        Arc,
        mpsc::{Sender, Receiver, TryRecvError, channel}
    },
    task::{Context, Poll}
};
use crate::executor;

async fn async_promise<'a, T, E>(future: impl Future<Output = Result<T, E>> + 'a, sender:Sender<Result<T, E>>) {
   let result = future.await;
   sender.send(result).unwrap();
}

async fn ready_promise<T, E>(ready: Result<T, E>, sender: Sender<Result<T, E>>) {
    sender.send(ready).unwrap();
}

pub struct Promise<T: Send + 'static, E:Send + 'static>(Receiver<Result<T, E>>);

impl<T: Send + 'static, E:Send + 'static> Promise<T, E> {
    pub fn new(future: impl Future<Output = Result<T, E>> + Send + 'static) -> Self {
        let (sender, receiver) = channel::<Result<T, E>>();
        executor::spawn_task(async_promise(future, sender));
        Self(
            receiver
        )
    }

    fn ready(result:Result<T, E>) -> Self {
        let (sender, receiver) = channel::<Result<T, E>>();
        executor::spawn_task(ready_promise(result, sender));
        Self(
            receiver
        )
    }

    pub fn then<F: Future<Output = Result<R, E>> + Send + 'static, R:Send + 'static>(self, callback: impl FnOnce(T) -> F)  -> Promise<R, E>{
        match self.0.recv() {
            Ok(result) => match result {
                Ok(t) => Promise::new(callback(t)),
                Err(e) => Promise::ready(Err(e))
            },
            Err(_) => panic!("Promise was disconnected from Executor!")
        }
    }

    pub fn error(self, callback: impl FnOnce(E)) {
        match self.0.recv() {
            Ok(result) => {
                if let Err(e) = result {
                    callback(e)
                }
            },
            Err(_) => panic!("Promise was disconnected from Executor!")
        }
    }
}

impl<T: Send + 'static, E:Send + 'static> Future for Promise<T, E> {
    type Output = Arc<Result<T, E>>;

    fn poll(self: std::pin::Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.try_recv() {
            Ok(t) => Poll::Ready(Arc::new(t)),
            Err(e) => if e == TryRecvError::Empty {
                Poll::Pending
            } else {
                panic!("Promise was disconnected from Executor!")
            }
        }
    }
}

#[macro_export]
macro_rules! promise {
    ($value:path) => {
        async_http::Promise::new($value())
    };
    ($value:path=>error=$type:ty) => {
        async_http::Promise::new((async|| -> Result<(), $type>{
            let e = $value().await;
            Err(e)
        })());
    };
    ($value:path=>result=$type:ty) => {
        async_http::Promise::new((async|| -> Result<$type, ()>{
            let r = $value().await;
            Ok(r)
        })())
    };
}