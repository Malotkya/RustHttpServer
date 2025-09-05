use std::{
    sync::{
        Arc, Mutex,
    },
    task::{Waker, Wake},
    collections::VecDeque
};
use super::TaskId;

#[derive(Clone)]
pub(crate) struct Queue(Arc<Mutex<VecDeque<TaskId>>>);

impl Queue {
    pub fn new(capacity:usize) -> Self {
        Self(Arc::new(
            Mutex::new(
                VecDeque::with_capacity(capacity)
            )
        ))
    }

    pub fn push(&self, item:TaskId) {
        let mut queue = self.0.lock().unwrap();
        if queue.len() >= queue.capacity() {
            panic!("Task Queue is full!")
        }
        queue.push_back(item);
    }

    pub fn pop(&self) -> Option<TaskId> {
        let mut queue = self.0.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.0.lock().unwrap();
        queue.is_empty()
    }
}

pub(crate) struct TaskWaker {
    task:TaskId,
    queue: Queue
}

impl TaskWaker {
    pub fn new(task:TaskId, queue:Queue) -> Waker {
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