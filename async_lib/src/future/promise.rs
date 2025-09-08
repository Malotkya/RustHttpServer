use std::{
    sync::{
        mpsc::{Sender, Receiver, TryRecvError, channel}
    },
    task::{Context, Poll}
};
use crate::spawn_task;

async fn async_promise<T, E>(future: impl Future<Output = Result<T, E>> + 'static, sender:Sender<Result<T, E>>) {
   let result = future.await;
   sender.send(result).unwrap();
}

async fn callback_promise<T, E>(res: impl FnOnce(T) + 'static, rej: impl FnOnce(E) + 'static, callback: impl FnOnce( Box<dyn FnOnce(T)> , Box<dyn FnOnce(E)> ) ) {
    callback(
        Box::new(res),
        Box::new(rej)
    );
} 

async fn ready_promise<T, E>(ready: Result<T, E>, sender: Sender<Result<T, E>>) {
    sender.send(ready).unwrap();
}

pub type ResultCallback<T> = Box<dyn Fn(T)>;

pub struct Promise<T, E>(Receiver<Result<T, E>>);

impl<T:'static, E:'static> Promise<T, E> {

    pub fn new(callback: impl FnOnce(Box<dyn FnOnce(T)> , Box<dyn FnOnce(E)>) + 'static) -> Self {
        let (sender, receiver) = channel::<Result<T, E>>();
        let sender_clone = sender.clone();

        let res= move |value:T| sender.send(Ok(value)).unwrap();
        let rej = move |value: E| sender_clone.send(Err(value)).unwrap();
        spawn_task(callback_promise(res, rej, callback));
        
        
        Self(
            receiver
        )
    }

    pub fn future(future: impl Future<Output = Result<T, E>> + 'static) -> Self {
        let (sender, receiver) = channel::<Result<T, E>>();
        spawn_task(async_promise(future, sender));

        Self(
            receiver
        )
    }

    fn ready(result:Result<T, E>) -> Self {
        let (sender, receiver) = channel::<Result<T, E>>();
        spawn_task(ready_promise(result, sender));
        Self(
            receiver
        )
    }

    pub fn then<R:'static>(self, callback: impl FnOnce(Box<dyn FnOnce(R)> , Box<dyn FnOnce(E)>) + 'static) -> Promise<R, E> {
        Promise::new(callback)
    }

    pub fn then_future<F: Future<Output = Result<R, E>> + 'static, R:'static>(self, callback: impl FnOnce(T) -> F)  -> Promise<R, E>{
        match self.0.recv() {
            Ok(result) => match result {
                Ok(t) => Promise::future(callback(t)),
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

impl<T:'static, E:'static> Future for Promise<T, E> {
    type Output = Result<T, E>;

    fn poll(self: std::pin::Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<T, E>> {
        match self.0.try_recv() {
            Ok(t) => Poll::Ready(t),
            Err(e) => if e == TryRecvError::Empty {
                ctx.waker().wake_by_ref();
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
        $crate::Promise::new($value())
    };
    ($value:path=>error=$type:ty) => {
        $crate::Promise::new((async|| -> Result<(), $type>{
            let e = $value().await;
            Err(e)
        })());
    };
    ($value:path=>result=$type:ty) => {
        $crate::Promise::new((async|| -> Result<$type, ()>{
            let r = $value().await;
            Ok(r)
        })())
    };
}