use std::{
    cell::RefCell, pin::Pin, sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, TryRecvError}
    },
    task::{Context, Poll}
};
use lazy_static::__Deref;

mod job;
use job::*;
mod tasks;
use tasks::*;
mod waker;
mod thread;
pub(crate) use thread::*;
mod atomic;
pub(crate) use atomic::*;

const DEFAULT_QUEUE_SIZE: usize = 1000;

pub(crate) static RUNNING:AtomicBool = AtomicBool::new(false);
lazy_static::lazy_static!(
    static ref JOBS:JobHandler = JobHandler::new(DEFAULT_QUEUE_SIZE);
);

thread_local! {
    static THREAD_POOL:RefCell<ThreadPool> = RefCell::new(ThreadPool::new());
    static TASKS:RefCell<TaskHandler<'static>> = RefCell::new(TaskHandler::new(DEFAULT_QUEUE_SIZE));
}

pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    TASKS.with(|cell|{
        let tasks = cell.borrow();
        tasks.spawn_task(future);
    })
}

pub fn thread_await<T: Send + 'static>(func: impl Fn() -> T + Send + Sync + 'static) -> impl Future<Output = T> {
    let (sender, receiver) = channel::<T>();

    JOBS.add(move||{
        let r = func();
        sender.send(r).unwrap();
    });

    Actor(receiver)
}

pub fn thread_run(func: impl Fn() + Send + Sync + 'static) {
    JOBS.add(func);
}

struct Actor<T:Send>(pub(crate) Receiver<T>);

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

struct UserTask{
    function: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>  + 'static>,
    future: Option<Pin<Box<dyn Future<Output = ()> + 'static>>>
}

impl UserTask {
    pub fn new(func: impl Fn() -> Pin<Box<dyn Future<Output = ()>>>  + 'static) -> Self {
        Self {
            function: Box::new(func),
            future: None
        }
    }
}

impl Future for UserTask {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.future.as_mut() {
            Some(future) => if future.as_mut().poll(cx).is_ready() {
                self.future = None;
            },
            None => {
                let mut future = (self.function.as_ref())();
                if future.as_mut().poll(cx).is_pending() {
                    self.future = Some(future);
                }
            }
        }
        
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn init_thread_pool(thread_count:usize, listener: impl Fn() + Send + Sync + 'static) {
    if thread_count == 0 {
        panic!("Unable to initalize thread pool with zero threads!");
    }
    
    THREAD_POOL.with(|cell|{
        let mut pool = cell.borrow_mut();
        pool.init(thread_count);

        match thread_count {
            1 => {
                println!("Two threads is recomended");
                pool.init_thread(
                    "Listener & IO", 
                    JOBS.single_thread(listener), 
                );
            },
            _ => {
                pool.init_thread("Listener", ListenerThreadJob::new(listener));

                let mut index:usize = 1;
                while index < thread_count {
                    pool.init_thread(
                        &format!("IO: {}", index),
                        JOBS.deref()
                    );

                    index += 1;
                }
            }
        }
    });
}

pub fn start_with_callback(callback: impl Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static) {
    TASKS.with(|cell|{
        let tasks = cell.borrow();

        tasks.spawn_task(UserTask::new(callback));
        start();
    });
}

pub fn start() {
    RUNNING.store(true, Ordering::Relaxed);

    TASKS.with(|cell|{
        let tasks = cell.borrow();

        while RUNNING.load(Ordering::Acquire) {
            tasks.sleep_if_idle();
            tasks.run_ready_tasks();
        }
    })
}

pub fn shut_down() {
    RUNNING.store(false, Ordering::Relaxed);
    THREAD_POOL.take().join();
}