use std::{
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{channel, Receiver, TryRecvError}
    },
    thread::{Builder, JoinHandle, park},
};
use super::{
    AtomicList, AtomicQueue,
    RUNNING,
};

pub trait Thread<R> = FnOnce() -> R + Send + Sync + 'static;
trait QueuedJob = FnOnce() + Send + Sync;

#[derive(Clone)]
pub(crate) struct ThreadManager {
    pool: AtomicList<JoinHandle<()>>,
    park: AtomicList<AtomicPtr<JoinHandle<()>>>,
    queue: AtomicQueue<Box<dyn QueuedJob>>
}

unsafe impl Send for ThreadManager {}

impl ThreadManager {
    pub fn new(queue_size:usize) -> Self {
        Self {
            pool: AtomicList::new("Thread Pool", 0),
            park: AtomicList::new("Thread Park", 0),
            queue: AtomicQueue::new("Thread Job", queue_size)
        }
    }

    pub fn init(&self, size:usize) {
        assert_eq!(self.pool.get_capacity(), 0, "Thread Pool is Already Initalized!");

        self.pool.set_capacity(size);
        self.park.set_capacity(size);

        for id in 0..size {
            let clone = self.clone();
            let handle = Builder::new()
                .name(id.to_string())
                .spawn(move||{
                    while RUNNING.load(Ordering::Relaxed) {
                        match clone.next_job() {
                            Some(job) => job(),
                            None => clone.park(id)
                        }
                    }
                })
                .unwrap();

            self.pool.push(handle);
        }
    }

    pub fn update_queue_capacity(&self, capcity:usize) {
        self.queue.set_capacity(capcity);
    }

    fn next_job(&self) -> Option<Box<dyn QueuedJob>> {
        self.queue.pop()
    }

    fn park(&self, id:usize) {
        let ptr = self.pool.get(id).unwrap();
        self.park.push(ptr);
        park()
    }

    pub fn queue_thread<T:Send + 'static>(&self, func: impl Thread<T>) -> Job<T>{
        let (sender, receiver) = channel::<T>();

        self.queue.push(Box::new(move||sender.send(func()).unwrap()));

        if let Some(handle) = self.park.pop() {
            //SAFTEY: Handle has 'static lifetime
            unsafe{ (**handle.as_ptr()).thread().unpark() }
        }

        Job(receiver)
    }

    pub fn join_all(&self) {
        while let Some(handle) = self.pool.pop() {
            let name = handle.thread().name().unwrap_or("Anonymous").to_string();

            if let Err(e) = handle.join() {
                println!("Thread {name} paniced with error:\n{:?}", e)
            }
        }
    }
}

pub struct Job<T:Send>(pub(crate) Receiver<T>);

impl<T:Send> Future for Job<T> {
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

