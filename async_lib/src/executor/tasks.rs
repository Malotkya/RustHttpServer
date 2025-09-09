use std::{
    sync::{
        atomic::{
            AtomicU64, Ordering
        }
    },
    task::{Context, Poll, Waker},
    thread
};
use super::{
    waker::TaskWaker,
    AtomicQueue, AtomicMap, AtomicFuture, ThreadPoolConnection, AtomicOption, ThreadJob
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
pub(crate) struct Task<'a> {
    pub(crate) id: TaskId,
    future: AtomicFuture<'a>
}

unsafe impl<'a> Send for Task<'a> {}

impl<'a> Task<'a> {
    pub(crate) fn new(future: impl Future<Output = ()> +'a) -> Self {
        Self {
            id: TaskId::new(),
            future: AtomicFuture::new(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        self.future.poll(context)
    }
}

pub(crate) struct TaskHandler<'a> {
    tasks: AtomicMap<TaskId, Task<'a>>,
    waker_cache: AtomicMap<TaskId, Waker>,
    task_queue: AtomicQueue<TaskId>,
    conn: AtomicOption<ThreadPoolConnection>
}

unsafe impl Send for TaskHandler<'static> {}

impl<'a> Clone for TaskHandler<'a> {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            waker_cache: self.waker_cache.clone(),
            task_queue: self.task_queue.clone(),
            conn: self.conn.clone()
        }
    }
}

impl<'a> TaskHandler<'a> {
    pub fn new(queue_size:usize) -> Self {
        Self {
            tasks: AtomicMap::new(),
            waker_cache: AtomicMap::new(),
            task_queue: AtomicQueue::new("Task", queue_size),
            conn: AtomicOption::new()
        }
    }

    pub fn spawn_task(&self, future: impl Future<Output = ()> + 'a) {
        let task = Task::new(Box::pin(future));
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id);

        if let Some(conn) = self.conn.try_unwrap() {
            conn.unpark_self();
        }
    }

    pub fn run_ready_tasks(&self) {
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
            thread::park();
        }
    }
}


impl ThreadJob for TaskHandler<'static> {
    fn connect(&self, conn:ThreadPoolConnection) {
        self.conn.set(Some(conn));
    }

    fn next(&self) {
        if self.task_queue.is_empty() {
            self.sleep_if_idle()
        }

        self.run_ready_tasks();
    }
}