use super::*;
use task::{Task, TaskId};
use queue::Queue;
use std::{
    collections::BTreeMap,
    sync::Arc,
    task::{Context, Poll, Wake, Waker}
};

const TASK_CAPACITY: usize = 100;

pub struct Executor<'this> {
    tasks:BTreeMap<TaskId, Task<'this>>,
    task_queue: Queue,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl<'t> Executor<'t> {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Queue::new(TASK_CAPACITY),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task:Task<'t>) {
        let task_id = task.id;
        self.tasks.insert(task_id, task);
        self.task_queue.push(task_id).unwrap();
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
                Poll::Ready(r) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);

                    if r.is_err() {
                        println!("{}", r.err().unwrap());
                    }
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

struct TaskWaker {
    task_id: TaskId,
    task_queue: Queue
}

impl TaskWaker {
    fn new(task_id:TaskId, task_queue:Queue) -> Waker {
        Waker::from(Arc::new(TaskWaker{
            task_id, task_queue
        }))
    }
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).unwrap()
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


#[cfg(test)]
mod test {
    use super::*;

    //Async function 1
    async fn name() -> &'static str {
        "World"
    }

    //Async function 2
    async fn print_message() -> std::io::Result<()> {
        println!(
            "Hello {}!",
            name().await
        );
        Ok(())
    }

    #[test]
    fn test_exec_spawn() {
        let mut exec = executor::Executor::new();
        exec.spawn(tasks::Task::new(print_message()));

        while exec.has_some() {
            exec.run_ready_tasks();
        }
    }
}