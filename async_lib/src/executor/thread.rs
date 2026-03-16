use std::{
    sync::{
        atomic::Ordering,
        Arc, Mutex
    },
    thread::{Builder, JoinHandle, park, Thread},
};
use super::{
    RUNNING,
    AtomicOption
};

pub trait ThreadJob = Fn(ThreadPoolConnection) + Send + Sync + 'static;

#[derive(Clone)]
pub struct ThreadPoolConnection {
    id: usize,
    parked: Arc<Mutex<bool>>,
    handle: AtomicOption<&'static JoinHandle<()>>
}

unsafe impl Send for ThreadPoolConnection {}

impl ThreadPoolConnection {
    pub fn unpark_self(&self) {
        if let Some(handle) = self.handle.try_unwrap() {
            if let Ok(mut parked) = self.parked.lock() {
                *parked = false;
                handle.thread().unpark();
            } 
        }
    }

    pub fn park(&self) {
        if let Ok(mut parked) = self.parked.lock() {
            *parked = true;
            park();
        } 
    }

    pub fn running(&self) -> bool {
        RUNNING.load(Ordering::Relaxed)
    }
}

struct PoolThreadConnection {
    id: usize,
    parked: Arc<Mutex<bool>>,
    handle: JoinHandle<()>
}

impl PoolThreadConnection {
    fn is_parked(&self) -> Option<bool> {
        if let Ok(parked) = self.parked.lock() {
            Some(*parked)
        } else {
            None
        }
    }

    fn thread(&self) -> &Thread {
        self.handle.thread()
    }

    fn join(self) {
        let name = (self.handle.thread().name().unwrap_or("Anonymous")).to_string();

        if self.handle.join().is_err() {
            println!("Unable to join with thread: \"{}\"!", name);
        }
    }
}

#[derive(Default)]
pub struct ThreadPool{
    threads: Vec<PoolThreadConnection>,
}

impl ThreadPool {
    pub const fn new() -> Self {        
        Self {
            threads: Vec::new()
        }
    }

    pub fn init(&mut self, thread_count:usize) {
        assert_ne!(self.threads.capacity(), 0, "Thread Pool is Already Initalized!"); 
        self.threads = Vec::with_capacity(thread_count);
    }

    pub fn init_thread(&mut self, name:&str, thread:impl ThreadJob) {
        let index = self.threads.len();

        if index >= self.threads.capacity() {
            panic!("Thread Pool Full!")
        }

        let builder = Builder::new().name(name.to_string());
        let conn = ThreadPoolConnection{
            id: index,
            parked: Arc::new(Mutex::new(false)),
            handle: AtomicOption::none()
        };
        let clone = conn.clone();

        let handle = builder.spawn(move||thread(conn)).unwrap();
        clone.handle.set(Some(unsafe {
            &*(&handle as *const JoinHandle<()>)
                as &JoinHandle<()>
        }));

        let handle = PoolThreadConnection {
            id: index,
            parked: clone.parked.clone(),
            handle
        };

        self.threads.push(handle);
    }

    pub fn unpark(&mut self, mut count:usize) {
        for handle in &self.threads {
            match handle.is_parked() {
                Some(parked) => if parked {
                    handle.thread().unpark();
                    count -= 1;
                },
                //If Unsure, unpark anyway but don't count it.
                None =>  handle.thread().unpark()
            }

            if count == 0 {
                break;
            }
        }
    }

    pub fn join(mut self) {
        while let Some(handle) = self.threads.pop() {
            handle.thread().unpark();
            handle.join()
        }
    }
}
