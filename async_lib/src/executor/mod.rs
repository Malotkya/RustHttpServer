use std::{
    sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering},
        
    },
};

mod tasks;
use tasks::*;
mod thread;
pub(crate) use thread::*;
mod atomic;
pub(crate) use atomic::*;

pub(crate) const DEFAULT_QUEUE_SIZE: usize = 1000;

pub trait Thread<R> = FnOnce() -> R + Send + Sync + 'static;

pub(crate) static RUNNING: AtomicBool = AtomicBool::new(false);
static TASK_MANAGER   :LazyLock<TaskHandler> = LazyLock::new(||TaskHandler::new(DEFAULT_QUEUE_SIZE));
static THREAD_MANAGER :LazyLock<ThreadManager> = LazyLock::new(||ThreadManager::new(DEFAULT_QUEUE_SIZE)); 

#[inline]
pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    TASK_MANAGER.spawn_task(future);
}

#[inline]
pub fn queue_job<R:Send + 'static>(func: impl Thread<R>) -> impl Future<Output = R> {
    THREAD_MANAGER.queue_thread(func)
}

#[inline]
pub fn is_running() -> bool {
    RUNNING.load(Ordering::Relaxed)
}

pub fn init_async_thread_pool(thread_count:usize) {  
    if thread_count == 0 {
        panic!("Unable to initalize thread pool with zero threads!");
    }
    
    THREAD_MANAGER.init(thread_count);
}

pub fn start() {
    RUNNING.store(true, Ordering::Relaxed);

    while RUNNING.load(Ordering::Relaxed) {
        TASK_MANAGER.run_all_tasks();
    }

    THREAD_MANAGER.join_all();
}

pub fn shut_down() {
    RUNNING.store(false, Ordering::Relaxed);
    
}