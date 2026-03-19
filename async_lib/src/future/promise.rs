use std::{
    sync::{
        mpsc::{Receiver, TryRecvError, channel}
    },
    task::{Context, Poll},
    fmt
};
use crate::spawn_task;

pub struct Promise<R>(Receiver<R>);

impl<R:'static> Promise<R> {

    pub fn new(callback: impl FnOnce() -> R + 'static) -> Self {
        let (sender, receiver) = channel::<R>();
        spawn_task(async move{sender.send(callback()).unwrap();} );
        
        Self(
            receiver
        )
    }

    pub fn future(future: impl Future<Output = R> + 'static) -> Self {
        let (sender, receiver) = channel::<R>();
        spawn_task(async move{sender.send(future.await).unwrap()});

        Self(
            receiver
        )
    }

    pub fn callback(callback: impl FnOnce(Box<dyn FnOnce(R)>) + 'static) -> Self {
        let (sender, receiver) = channel::<R>();
        let res= move |value:R| sender.send(value).unwrap();

        spawn_task(async{callback(Box::new(res))});
        
        
        Self(
            receiver
        )
    }

    /*fn ready(result:R) -> Self {
        let (sender, receiver) = channel::<R>();
        spawn_task(async move{sender.send(result).unwrap();} );

        Self(
            receiver
        )
    }*/

    pub fn then<T:'static>(self, callback: impl FnOnce(R) -> T + 'static) -> Promise<T> {
        self.then_future(async move|temp| {
            callback(temp)
        })
    }

    pub fn then_future<F: Future<Output = T> + 'static, T:'static>(self, callback: impl FnOnce(R) -> F + 'static)  -> Promise<T>{
        Promise::future(async move{
            callback(self.await).await
        })
    }
}

impl<R:'static> Future for Promise<R> {
    type Output = R;

    fn poll(self: std::pin::Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<R> {
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

impl<R:fmt::Debug> fmt::Debug for Promise<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Promise").finish()
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