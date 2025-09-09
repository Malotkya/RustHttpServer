use std::{
    pin::Pin, 
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicU64, Ordering
        }
    },
    task::{Context, Poll, Waker},
    thread::{park, JoinHandle, spawn},
};
use super::{
    waker::TaskWaker,
    atomic_coll::{Queue, Map}
};

#[derive(Clone)]
pub(crate) struct AtomicFuture<'a>(Arc<Mutex<Pin<Box<dyn Future<Output = ()> + 'a>>>>);

impl<'a> AtomicFuture<'a> {
    fn new(f: impl Future<Output = ()> + 'a) -> Self {
        Self(
            Arc::new(
                Mutex::new(
                    Box::pin(f)
                )
            )
        )
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        let mut task = self.0.lock().unwrap();
        task.as_mut().poll(cx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(pub(crate) u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone)]
pub(crate) struct Task<'a> {
    pub(crate) id: TaskId,
    future: AtomicFuture<'a>
}

unsafe impl<'a> Send for Task<'a> {}
//unsafe impl<'a> Sync for Task<'a> {}

impl<'a> Task<'a> {

    pub(crate) fn new(future: impl Future<Output = ()> + 'a) -> Self {
        Self {
            id: TaskId::new(),
            future: AtomicFuture::new(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        self.future.poll(context)
    }
}

pub(crate) struct Executor<'a> {
    tasks: Map<TaskId, Task<'a>>,
    waker_cache: Map<TaskId, Waker>,
    task_queue: Queue<TaskId>,
    handle: Option<JoinHandle<()>>
}

unsafe impl Send for Executor<'static> {}

impl<'a> Clone for Executor<'a> {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            waker_cache: self.waker_cache.clone(),
            task_queue: self.task_queue.clone(),
            handle: None
        }
    }
}

impl<'a> Executor<'a> {
    pub fn new(queue_size:usize) -> Self {
        Self {
            tasks: Map::new(),
            waker_cache: Map::new(),
            task_queue: Queue::new("Task", queue_size),
            handle: None
        }
    }

    pub fn spawn_task(&mut self, future: impl Future<Output = ()> + 'a) {
        let task = Task::new(Box::pin(future));
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id);

        if let Some(handle) = &self.handle {
            handle.thread().unpark()
        }
    }

    pub fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue
            };

            let waker = self.waker_cache.default_entry(
                task_id,
                || TaskWaker::new(task_id, self.task_queue.clone())
            );
            let mut context = Context::from_waker(&*waker);

            match task.lock().unwrap().poll(&mut context) {
                Poll::Ready(_) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                },
                Poll::Pending => {}
            }
        }
    }

    pub fn sleep_if_idle(&self) {
        if self.task_queue.is_empty() {
            park();
        }
    }

    pub fn start_thread(&'static mut self) {
        let mut clone = self.clone();

        self.handle = Some(spawn(move ||{
            while super::RUNNING.load(Ordering::Acquire) {
                clone.run_ready_tasks();
                clone.sleep_if_idle();
            }
        }));
    }
}