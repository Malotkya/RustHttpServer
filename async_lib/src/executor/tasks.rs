use std::{
    task::{Context, Poll},
    sync::atomic::{AtomicU64, Ordering},
    pin::Pin
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(pub(crate) u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub(crate) struct Task {
    pub(crate) id: TaskId,
    future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>
}

impl Task where {

    pub(crate) fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}