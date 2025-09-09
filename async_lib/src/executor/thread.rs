use std::{
    thread::{spawn, JoinHandle, park},
    sync::{
        atomic::Ordering
    }
};
use super::atomic_coll::Queue;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn spawn_job_thread(queue:Queue<Job>) -> JoinHandle<()> {
    spawn(move||{
        while super::RUNNING.load(Ordering::Acquire) {
            if let Some(job) = queue.pop() {
                job();
            } else {
                park()
            }

        }
    })
}

pub struct ThreadPool {
    jobs: Queue<Job>,
    threads: Vec<JoinHandle<()>>
}

impl ThreadPool {
    pub fn new() -> Self {        
        Self {
            jobs: Queue::new("Job", super::EXECUTOR_QUEUE_SIZE),
            threads: Vec::new()
        }
    }

    pub fn init(&mut self, thread_count:usize) {
        assert_ne!(self.threads.capacity(), 0, "Thread pool already initalized!");
        self.threads.reserve(thread_count);

        for _ in 0..thread_count {
            self.threads.push(spawn_job_thread(self.jobs.clone()))
        }
    }

    pub fn add_job(&self, job: impl FnOnce() + Send + 'static) {
        self.jobs.push(Box::new(job));

        for handle in &self.threads {
            handle.thread().unpark()
        }
    }
}
