use std::{
    cell::RefCell,
    sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, TryRecvError}
    },
    thread::JoinHandle
};

mod tasks;
use tasks::*;
mod waker;
mod thread;
pub(crate) use thread::*;
mod atomic;
pub(crate) use atomic::*;

const DEFAULT_QUEUE_SIZE: usize = 1000;

pub(crate) static RUNNING:AtomicBool = AtomicBool::new(false);
static JOB_THREAD_QUEUE:LazyLock<AtomicQueue<Box<dyn ThreadJob>>> = LazyLock::new(||{
    AtomicQueue::new("Job", DEFAULT_QUEUE_SIZE)
});
static ASYNC_TASKS_HANDLER:LazyLock<TaskHandler> = LazyLock::new(||{
    TaskHandler::new(DEFAULT_QUEUE_SIZE)
});
static MAIN_THREAD_HANDLE:LazyLock<AtomicOption<JoinHandle<()>>> = LazyLock::new(||{
    AtomicOption::none()
});

pub trait ThreadJobReturnValue<T> = Fn(ThreadPoolConnection) -> T + Send + Sync + 'static;

thread_local! {
    static THREAD_POOL:RefCell<ThreadPool> = RefCell::new(ThreadPool::new());
}

pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    ASYNC_TASKS_HANDLER.spawn_task(future);
    unpark_main_thread();
}

pub fn spawn_thread_job(func: impl ThreadJob) {
    JOB_THREAD_QUEUE.push(Box::new(func));

    unpark_job_threads(1);
}

pub fn await_thread_job<T: Send + 'static>(func: impl ThreadJobReturnValue<T>) -> impl Future<Output = T> {
    let (sender, receiver) = channel::<T>();

    JOB_THREAD_QUEUE.push(Box::new(move|conn|{
        sender.send(func(conn)).unwrap()
    }));

    unpark_job_threads(1);

    Actor(receiver)
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

fn job_thread(conn:ThreadPoolConnection) {
    while RUNNING.load(Ordering::Relaxed) {
        //Sleep If Idle
        if JOB_THREAD_QUEUE.is_empty() && ASYNC_TASKS_HANDLER.is_empty() {
            std::thread::park();
        }

        while let Some(job) = JOB_THREAD_QUEUE.pop() {
            job(conn.clone())
        }

        ASYNC_TASKS_HANDLER.run_next_task();
    }
}

#[inline]
fn unpark_main_thread() {
    if let Some(handle) = MAIN_THREAD_HANDLE.try_unwrap() {
        handle.thread().unpark();
    }
}

#[inline]
pub(crate) fn unpark_job_threads(count:usize) {
    THREAD_POOL.with(|cell|{
        cell.borrow_mut().unpark(count)
    });
}

pub fn init_async_thread_pool(thread_count:usize) {  
    if thread_count == 0 {
        panic!("Unable to initalize thread pool with zero threads!");
    }
    
    THREAD_POOL.with(|cell|{
        let mut pool = cell.borrow_mut();
        pool.init(thread_count);

        for i in 0..thread_count {
            pool.init_thread(&i.to_string(), job_thread);
        }
    });
}

pub fn start() {
    RUNNING.store(true, Ordering::Relaxed);

    while RUNNING.load(Ordering::Relaxed) {
        //Sleep If Idle
        if ASYNC_TASKS_HANDLER.is_empty() {
            std::thread::park();

        //Ask for help with tasks
        } else {
            unpark_job_threads(ASYNC_TASKS_HANDLER.len());
        }

        //Handle as many tasks as possible.
        ASYNC_TASKS_HANDLER.run_all_tasks();
    }

    THREAD_POOL.take().join();
}

pub fn shut_down() {
    RUNNING.store(false, Ordering::Relaxed);
    
}