use std::{
    task::{Context, Poll},
    sync::atomic::{AtomicU64, Ordering},
    pin::Pin
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

pub(crate) type Result = std::io::Result<()>;

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task<'server> {
    pub(crate) id: TaskId,
    pub(crate) future: Pin<Box<dyn Future<Output = Result> + 'server>>
}

impl<'s> Task<'s> where {

    pub(crate) fn new(future: impl Future<Output = Result> + 's) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future)
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<Result> {
        self.future.as_mut().poll(context)
    }
}