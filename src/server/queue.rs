use std::{
    collections::VecDeque,
    sync::{Arc, Mutex}
};
use super::task::TaskId;

#[derive(Debug, Clone)]
pub struct Queue(Arc<Mutex<VecDeque<TaskId>>>, usize);

impl Queue {
    pub fn new(size:usize) -> Self {
        Self (
            Arc::new(
                Mutex::new(VecDeque::with_capacity(size))
            ),
            size
        )
    }

    pub fn is_empty(&self) -> bool {
        let queue  = self.0.lock().unwrap();
        queue.is_empty()
    }

    pub fn push(&self, value: TaskId) -> Result<(), &'static str> {
        let mut queue = self.0.lock().unwrap();
        if queue.len() >= self.1 {
            Err("Queue is at max capacity!")
        } else {
            queue.push_back(value);
            Ok(())
        }
    }

    pub fn pop(&self) -> Option<TaskId> {
        let mut queue = self.0.lock().unwrap();
        queue.pop_front()
    }
}