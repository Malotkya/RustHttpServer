use std::{
    sync::Arc,
    task::{Waker, Wake}
};
use super::TaskId;
use super::atomic::AtomicQueue;

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
        self.queue.push(self.task);
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