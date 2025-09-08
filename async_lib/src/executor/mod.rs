use std::{
    collections::BTreeMap,
    task::{Context, Poll, Waker},
    cell::RefCell
};
use tasks::*;
use waker::*; 

mod tasks;
mod waker;

const EXECUTOR_QUEUE_SIZE: usize = 1000;

thread_local! {
    static EXECUTOR:RefCell<Executor<'static>> = RefCell::new(Executor::new(EXECUTOR_QUEUE_SIZE))
}

pub fn spawn_task(future: impl Future<Output = ()> + 'static) {
    EXECUTOR.with(move |cell|{
        let mut exe = cell.borrow_mut();
        exe.spawn_task(future);
    })
}

pub fn executor_loop() {
    EXECUTOR.with(|cell|{
        let mut exe = cell.borrow_mut();
        //exe.sleep_if_idle();
        exe.run_ready_tasks();
    })
}

pub(crate) struct Executor<'a> {
    tasks: BTreeMap<TaskId, Task<'a>>,
    waker_cache: BTreeMap<TaskId, Waker>,
    task_queue: waker::Queue
}

impl<'a> Executor<'a> {
    pub fn new(queue_size:usize) -> Self {
        Self {
            tasks: BTreeMap::new(),
            waker_cache: BTreeMap::new(),
            task_queue: waker::Queue::new(queue_size)
        }
    }

    pub fn spawn_task(&mut self, future: impl Future<Output = ()> + 'a) {
        let task = Task::new(Box::pin(future));
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id);
    }

    pub fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue
            };

            let waker = self.waker_cache.entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);

            match task.poll(&mut context) {
                Poll::Ready(_) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                },
                Poll::Pending => {}
            }
        }
    }

    #[allow(dead_code)]
    pub fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        
        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}