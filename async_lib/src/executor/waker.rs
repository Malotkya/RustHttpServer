use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering}
    },
    task::{Waker, Wake},
    collections::VecDeque
};
use super::TaskId;

static QUEUE_SIZE:AtomicUsize = AtomicUsize::new(0);
pub(crate) static TASK_QUEUE:Mutex<VecDeque<TaskId>> = Mutex::new(VecDeque::new());

pub(crate) struct TaskWaker(TaskId);

pub(crate) fn get_queue_size() -> usize {
    QUEUE_SIZE.load(Ordering::Acquire)
}

pub(crate) fn set_queue_size(value:usize){
    QUEUE_SIZE.store(value, Ordering::Relaxed);
}

impl TaskWaker {
    pub fn new(task_id:TaskId) -> Waker {
        Waker::from(Arc::new(TaskWaker(
            task_id
        )))
    }

    fn wake_task(&self) {
        let mut queue = TASK_QUEUE.lock().unwrap();
        if queue.len() >= get_queue_size() {
            panic!("Task Queue is full!");
        }

        queue.push_back(self.0);
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