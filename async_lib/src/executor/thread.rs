use std::{
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{channel, Receiver, TryRecvError},
        LazyLock
    },
    thread::{Builder, JoinHandle},
};
use super::{
    atomic::{AtomicList, AtomicQueue},
    tasks::TASK_MANAGER,
    RUNNING, DEFAULT_QUEUE_SIZE
};

pub trait ThreadJob<R> = FnOnce() -> R + Send + Sync + 'static;
pub trait ThreadProcess = ThreadJob<()>;

pub(crate) static THREAD_MANAGER :LazyLock<ThreadManager> = LazyLock::new(||ThreadManager::new(DEFAULT_QUEUE_SIZE)); 

pub(crate) fn thread_main(id:usize) {
    while RUNNING.load(Ordering::Relaxed) {
        if let Some(job) = THREAD_MANAGER.next_process() {
            job();
        } else if TASK_MANAGER.is_empty() {
            THREAD_MANAGER.park(id);
        } else {
            TASK_MANAGER.run_next_task();
        }
    }
}

#[derive(Clone)]
pub(crate) struct ThreadManager {
    pool: AtomicList<JoinHandle<()>>,
    park: AtomicList<AtomicPtr<JoinHandle<()>>>,
    queue: AtomicQueue<Box<dyn ThreadProcess>>
}

// SAFETY: ThreadManager made up of sync safe components
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

        let handle = Builder::new()
                .name("0".to_string())
                .spawn(TASK_MANAGER.thread_main(0))
                .unwrap();

        println!("Spawned 0");
        self.pool.push(handle);

        for id in 1..size {
            let handle = Builder::new()
                .name(id.to_string())
                .spawn(move||thread_main(id))
                .unwrap();

            println!("Spawned {}", id);
            self.pool.push(handle);
        }
    }

    pub fn update_queue_capacity(&self, capcity:usize) {
        self.queue.set_capacity(capcity);
    }

    fn next_process(&self) -> Option<Box<dyn ThreadProcess>> {
        self.queue.pop()
    }

    pub fn park(&self, id:usize) {
        if let Some(ptr) = self.pool.get(id) {
            self.park.push(ptr);
        }
        std::thread::park();
    }

    pub fn unpark(&self, id:usize) {
        if let Some(ptr) = self.pool.get(id) {
            //SAFTEY: Handle has 'static lifetime
            let thread = unsafe{ &*ptr.load(Ordering::Relaxed) }.thread();
            thread.unpark();
        }
    }

    pub fn queue_process(&self, func: impl ThreadProcess) {
        self.queue.push(Box::new(func));

        if let Some(handle) = self.park.pop() {
            //SAFTEY: Handle has 'static lifetime
            let thread = unsafe{ &*handle.load(Ordering::Relaxed) }.thread();
            thread.unpark();
        }
    }

    pub fn queue_job<T:Send + 'static>(&self, func: impl ThreadJob<T>) -> Job<T>{
        let (sender, receiver) = channel::<T>();
        self.queue_process(move||sender.send(func()).unwrap());
        Job(receiver)
    }

    pub fn join_all(&self) {
        while let Some(handle) = self.pool.pop() {
            let name = handle.thread().name().unwrap_or("Anonymous").to_string();

            if let Err(e) = handle.join() {
                println!("Thread {name} paniced with error:\n{:?}", e);
            } else {
                println!("Thread {name} closed gracefully!");
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

