use std::{
    sync::{
        atomic::{AtomicBool, Ordering}
    },
};
use tasks::*;
use thread::*;

mod atomic;
pub mod tasks;
pub mod thread;

pub(crate) const DEFAULT_QUEUE_SIZE: usize = 1000;
pub(crate) static RUNNING: AtomicBool = AtomicBool::new(false);



#[inline]
pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    TASK_MANAGER.spawn_task(future);
}

#[inline]
pub fn queue_process(func: impl ThreadProcess) {
    THREAD_MANAGER.queue_process(func);
}

#[inline]
pub fn queue_job<R:Send + 'static>(func: impl ThreadJob<R>) -> impl Future<Output = R> {
    THREAD_MANAGER.queue_job(func)
}

#[inline]
pub fn set_queue_capacity(capcity:usize) {
    TASK_MANAGER.update_queue_capacity(capcity);
    THREAD_MANAGER.update_queue_capacity(capcity);
}

#[inline]
pub fn is_running() -> bool {
    RUNNING.load(Ordering::Relaxed)
}

pub fn start_async_thread_pool(thread_count:usize) {
    assert_ne!(thread_count, 0, "Unable to initalize thread pool with zero threads!");
    
    THREAD_MANAGER.init(thread_count);
    RUNNING.store(true, Ordering::Relaxed);
    THREAD_MANAGER.join_all();
}

pub fn shut_down() {
    RUNNING.store(false, Ordering::Relaxed);
}