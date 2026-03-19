use std::{
    pin::Pin,
    sync::{
        Arc,
        atomic::{
            AtomicU64, Ordering
        },
        LazyLock
    },
    task::{Context, Poll, Wake, Waker}
};
use super::{
    atomic::*,
    thread::{THREAD_MANAGER, ThreadProcess},
    RUNNING,
    DEFAULT_QUEUE_SIZE
};

pub(crate) static TASK_MANAGER   :LazyLock<TaskHandler> = LazyLock::new(||TaskHandler::new(DEFAULT_QUEUE_SIZE));


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(pub(crate) u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

//SAFETY: u64 is safe
unsafe impl Send for TaskId {}
unsafe impl Sync for TaskId {}

#[derive(Clone)]
pub(crate) struct Task {
    pub(crate) id: TaskId,
    future: AtomicFuture<()>
}

//SAFETY: TaskId & AtomicFutre are Send/Sync safe
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
        Pin::new(&mut self.future).poll(context)
    }
}

pub(crate) struct TaskHandler {
    tasks: AtomicMap<TaskId, Task>,
    waker_cache: AtomicMap<TaskId, Waker>,
    task_queue: AtomicQueue<TaskId>,
    thread_id: AtomicOption<usize>
}

//SAFETY: All properties are Send Safe
unsafe impl Send for TaskHandler {}

impl Clone for TaskHandler {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            waker_cache: self.waker_cache.clone(),
            task_queue: self.task_queue.clone(),
            thread_id: self.thread_id.clone()
        }
    }
}

impl TaskHandler {
    pub fn new(queue_size:usize) -> Self {
        Self {
            tasks: AtomicMap::new(),
            waker_cache: AtomicMap::new(),
            task_queue: AtomicQueue::new("Task", queue_size),
            thread_id: AtomicOption::none()
        }
    }

    pub fn spawn_task(&self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(Box::pin(future));
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id);

        self.unpark();
    }

    pub fn is_empty(&self) -> bool {
        self.task_queue.is_empty()
    }

    pub fn update_queue_capacity(&self, capcity:usize) {
        self.task_queue.set_capacity(capcity);
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

    pub fn idle_if_empty(&self) {
        if self.task_queue.is_empty() && let Some(id) = self.thread_id.try_unwrap() {
            THREAD_MANAGER.park(*id);
        }
    }

    fn unpark(&self) {
        if let Some(id) = self.thread_id.try_unwrap() {
            THREAD_MANAGER.unpark(*id);
        }
    }   

    pub fn thread_main(&'static self, id:usize) -> impl ThreadProcess {
        self.thread_id.set(Some(id));

        return move || {
            println!("{id} Started");

            while RUNNING.load(Ordering::Relaxed) {
                self.idle_if_empty();
                self.run_all_tasks();
            }
        };
    }
}

pub(crate) struct TaskWaker {
    task:TaskId,
    queue: AtomicQueue<TaskId>
}

impl TaskWaker {
    pub fn new(task:TaskId, queue:AtomicQueue<TaskId>) -> Waker {
        Waker::from(Arc::new(TaskWaker{
            task, queue
        }))
    }

    fn wake_task(&self) {
        self.queue.unique_push(self.task);
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}