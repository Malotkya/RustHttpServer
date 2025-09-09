use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{Receiver, TryRecvError, channel}
    },
    cell::RefCell
};
use tasks::*;
use thread::*;

mod tasks;
mod waker;
mod thread;
mod atomic_coll;

const EXECUTOR_QUEUE_SIZE: usize = 1000;
pub(crate) static RUNNING:AtomicBool = AtomicBool::new(false);

thread_local! {
    static EXECUTOR:RefCell<Executor<'static>> = RefCell::new(Executor::new(EXECUTOR_QUEUE_SIZE));
    static THREAD_POOL:RefCell<ThreadPool> = RefCell::new(ThreadPool::new());
}

pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    EXECUTOR.with(move |cell|{
        let mut exe = cell.borrow_mut();
        exe.spawn_task(future);
    })
}

pub fn thread_await<T: Send + 'static>(func: impl Fn() -> T + Send + 'static) -> impl Future<Output = T> {
    let (sender, receiver) = channel::<T>();

    THREAD_POOL.with(move|cell|{
        let pool = cell.borrow_mut();

        pool.add_job(move||{
            let r = func();
            sender.send(r).unwrap();
        });
    });

    Actor(receiver)
}

pub fn thread_run(func: impl Fn() + Send + 'static) {
    THREAD_POOL.with(move|cell|{
        let pool = cell.borrow();

        pool.add_job(func);
    })
}

pub fn executor_loop() {
    EXECUTOR.with(|cell|{
        let mut exe = cell.borrow_mut();
        //exe.sleep_if_idle();
        exe.run_ready_tasks();
    })
}

pub struct Actor<T:Send>(pub(crate) Receiver<T>);

impl<T:Send> Future for Actor<T> {
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.0.try_recv() {
            Ok(value) => std::task::Poll::Ready(value),
            Err(e) => if e == TryRecvError::Empty {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            } else {
                panic!("Actor disconected!")
            }
        }
    }
}