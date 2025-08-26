use std::pin::Pin;
use std::{
    task::{Context, Poll},
    sync::atomic::{AtomicU64, Ordering}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    pub(crate) id: TaskId,
    pub(crate) future: Pin<Box<dyn Future<Output = ()>>>
}

impl Task {
    pub(crate) fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}