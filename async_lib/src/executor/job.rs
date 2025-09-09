use super::{AtomicQueue, AtomicOption, ThreadJob, ThreadPoolConnection};
use std::thread;

pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Clone)]
pub(crate) struct JobHandler {
    queue: AtomicQueue<Job>,
    conn: AtomicOption<ThreadPoolConnection>
}

impl JobHandler {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: AtomicQueue::new("Job", capacity),
            conn: AtomicOption::new()
        }
    }

    pub fn add(&self, job: impl FnOnce() + Send + 'static) {
        self.queue.push(Box::new(job));
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