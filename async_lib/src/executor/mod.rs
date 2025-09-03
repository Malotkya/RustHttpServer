use std::{
    collections::BTreeMap,
    sync::Mutex,
    task::{Context, Poll, Waker}
};
use tasks::*;
use waker::*; 

mod tasks;
mod waker;

static TASKS: Mutex<BTreeMap<TaskId, Task>> = Mutex::new(BTreeMap::new());
static WAKER_CACHE: Mutex<BTreeMap<TaskId, Waker>> = Mutex::new(BTreeMap::new());

pub fn spawn_task(future: impl Future<Output = ()> + Send + 'static) {
    let task = Task::new(future);
    let task_id = task.id;

    let mut tasks = TASKS.lock().unwrap();
    tasks.insert(task_id, task);

    let mut queue = waker::TASK_QUEUE.lock().unwrap();
    if queue.len() >= waker::get_queue_size() {
        panic!("Task queue is full!");
    }
    queue.push_back(task_id);
}

pub fn init_executor(queue_size:usize) {
    waker::set_queue_size(queue_size);
}

pub fn run_ready_tasks() {
    let mut queue = waker::TASK_QUEUE.lock().unwrap();
    let mut tasks = TASKS.lock().unwrap();

    while let Some(task_id) = queue.pop_back() {
        let task = match tasks.get_mut(&task_id) {
            Some(task) => task,
            None => continue
        };

        let mut cache = WAKER_CACHE.lock().unwrap();
        let waker = cache.entry(task_id)
            .or_insert_with(|| TaskWaker::new(task_id));
        let mut context = Context::from_waker(waker);

        match task.poll(&mut context) {
            Poll::Ready(_) => {
                tasks.remove(&task_id);
                cache.remove(&task_id);
            },
            Poll::Pending => {}
        }
    }
}