use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Sender, Receiver, channel}
    },
    thread::{Builder, JoinHandle}
    
};

pub(crate) struct ThreadPoolConnection {
    id: usize,
    sender: Sender<usize>
}

unsafe impl Send for ThreadPoolConnection {}

impl Clone for ThreadPoolConnection {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            sender: self.sender.clone()
        }
    }
}

impl ThreadPoolConnection {
    pub fn unpark_self(&self) {
        self.sender.send(self.id).unwrap();
    }
}

pub trait ThreadJob: Send + Clone{
    fn connect(&self, conn: ThreadPoolConnection);
    fn next(&self);
}

pub struct ThreadPool{
    threads: Vec<JoinHandle<()>>,
    sender: Option<Sender<usize>>,
    receiver: Option<Receiver<usize>>
}

impl ThreadPool {
    pub const fn new() -> Self {        
        Self {
            threads: Vec::new(),
            sender: None,
            receiver: None
        }
    }

    pub fn init(&mut self, thread_count:usize) {
        assert_ne!(self.threads.capacity(), 0, "Thread Pool is Already Initalized!"); 

        let (sender, receiver) = channel::<usize>();
        self.sender = Some(sender);
        self.receiver = Some(receiver);
        self.threads = Vec::with_capacity(thread_count);
    }

    fn assert_init(&mut self) -> (&mut Vec<JoinHandle<()>>, &Sender<usize>) {
        if self.sender.is_none() {
            panic!("Thread pool is not intalized!");
        }

        let s = self.sender.as_ref().unwrap();
        (&mut self.threads, s)
    }

    pub fn init_thread<T: ThreadJob>(&mut self, name:&'static str, job:&'static mut T, running:AtomicBool) {
        let (threads, sender) = self.assert_init();
        
        let index = threads.len();
        if index >= threads.capacity() {
            panic!("Thread Pool Full!")
        }

        let builder = Builder::new().name(name.to_string());
        job.connect(ThreadPoolConnection{
            id: index,
            sender: sender.clone()
        });
        let clone = job.clone();

        threads.push(builder.spawn(move||{
            while running.load(Ordering::Acquire) {
                clone.next();
            }
        }).unwrap());
    }

    pub fn join(mut self) {
        while let Some(handle) = self.threads.pop() {
            handle.thread().unpark();

            let name = (handle.thread().name().unwrap_or("Anonymous")).to_string();
            if handle.join().is_err() {
                println!("Unable to join with thread: \"{}\"!", name);
            }
        }
    }
}
