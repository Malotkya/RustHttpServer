use super::{AtomicQueue, AtomicOption, ThreadJob, ThreadPoolConnection};
use std::thread;
use std::sync::Arc;

pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Clone)]
pub(crate) struct JobHandler {
    pub(crate) queue: AtomicQueue<Job>,
    pub(crate) conn: AtomicOption<ThreadPoolConnection>
}

impl JobHandler {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: AtomicQueue::new("Job", capacity),
            conn: AtomicOption::new()
        }
    }

    pub fn add(&self, job: impl FnOnce() + Send + 'static) {
        if self.conn.is_none() {
            panic!("Thread pool was never initalized!")
        }

        self.queue.push(Box::new(job));
    }

    pub fn single_thread(&'static self) -> Arc<SingleThreadJob> {
        Arc::new(
            SingleThreadJob {
                handler:self,
                listener: None
            }
        )
    }

    pub fn single_thread_listener(&'static self, listener: impl Fn() + Send + Sync + 'static) -> Arc<SingleThreadJob> {
        Arc::new(
            SingleThreadJob {
                handler: self,
                listener: Some(Arc::new(Box::new(listener)))
            }
        )
    }
}

impl ThreadJob for JobHandler {
    fn connect(&self, conn: ThreadPoolConnection) {
        self.conn.set(Some(conn));
    }

    fn next(&self) {
        if let Some(job) = self.queue.pop() {
            job();
        } else {
            thread::park();
        }
    }
}

pub(crate) struct SingleThreadJob {
    handler: &'static JobHandler,
    listener: Option<Arc<Box<dyn Fn() + Send + Sync + 'static>>>
}

impl Clone for SingleThreadJob {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler,
            listener: self.listener.clone()
        }
    }
}

impl ThreadJob for SingleThreadJob {
    fn connect(&self, conn: ThreadPoolConnection) {
        self.handler.conn.set(Some(conn))
    }

    fn next(&self) {
        if self.listener.is_some() {
            (self.listener.as_ref().unwrap())()
        }

        if let Some(job) = self.handler.queue.pop() {
            job();
        }
    }
}