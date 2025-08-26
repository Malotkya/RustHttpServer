
mod task;


use task::{Task, TaskId};
use std::{
    collections::{BTreeMap, VecDeque}, sync::{Arc, Mutex}, task::{Context, Poll, Wake, Waker}
};

const TASK_CAPACITY: usize = 100;

pub struct Executor {
    tasks:BTreeMap<TaskId, Task>,
    task_queue: Arc<Mutex<VecDeque<TaskId>>>,
    waker_cache: BTreeMap<TaskId, Waker>
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(
                Mutex::new(VecDeque::with_capacity(TASK_CAPACITY))
            ),
            waker_cache: BTreeMap::new()
        }
    }

    pub fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        let queue = self.task_queue.lock().unwrap();
        
        interrupts::disable();
        if queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    pub fn run(&mut self) -> ! {
        loop {

        }
    }

    pub fn spwan(&mut self, task:Task) {
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("Task id is already in use!");
        }
        let mut queue = self.task_queue.lock().unwrap();

        if queue.len() >= TASK_CAPACITY {
            panic!("Task queue is full!");
        }
        queue.push_back(task_id);
    }

    fn run_ready_task(&mut self) {
        let mut queue = self.task_queue.lock().unwrap();

        while let Some(task_id) = queue.pop_front() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue
            };

            let waker = self.waker_cache.entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                },
                Poll::Pending => {}
            }
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<Mutex<VecDeque<TaskId>>>
}

impl TaskWaker {
    fn new(task_id:TaskId, task_queue:Arc<Mutex<VecDeque<TaskId>>>) -> Waker {
        Waker::from(Arc::new(TaskWaker{
            task_id, task_queue
        }))
    }
    fn wake_task(&self) {
        let mut queue = self.task_queue.lock().unwrap();
        if queue.len() >= TASK_CAPACITY {
            panic!("Task queue is full!");
        } else {
            queue.push_back(self.task_id);
        }
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