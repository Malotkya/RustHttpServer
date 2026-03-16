use std::{
    sync::{
        atomic::{
            AtomicU64, Ordering
        }
    },
    task::{Context, Poll, Waker}
};
use super::{
    waker::TaskWaker,
    AtomicQueue, AtomicMap, AtomicFuture, ThreadPoolConnection, AtomicOption
};



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(pub(crate) u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone)]
pub(crate) struct Task {
    pub(crate) id: TaskId,
    future: AtomicFuture
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    pub(crate) fn new(future: impl Future<Output = ()> +'static) -> Self {
        Self {
            id: TaskId::new(),
            future: AtomicFuture::new(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        self.future.poll(context)
    }
}

pub(crate) struct TaskHandler {
    tasks: AtomicMap<TaskId, Task>,
    waker_cache: AtomicMap<TaskId, Waker>,
    task_queue: AtomicQueue<TaskId>,
}

unsafe impl Send for TaskHandler {}

impl Clone for TaskHandler {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            waker_cache: self.waker_cache.clone(),
            task_queue: self.task_queue.clone(),
        }
    }
}

impl TaskHandler {
    pub fn new(queue_size:usize) -> Self {
        Self {
            tasks: AtomicMap::new(),
            waker_cache: AtomicMap::new(),
            task_queue: AtomicQueue::new("Task", queue_size),
        }
    }

    pub fn spawn_task(&self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(Box::pin(future));
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id);
    }

    pub fn len(&self) -> usize {
        self.task_queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.task_queue.is_empty()
    }

    pub fn run_next_task(&self) {
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

            break;
        }
    }

    pub fn run_all_tasks(&self) {
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
}