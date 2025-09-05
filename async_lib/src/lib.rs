#![feature(trait_alias)]
#![allow(dead_code)]
use std::cell::RefCell;
use executor::Executor;

mod executor;
mod future;
pub use future::*;

const EXECUTOR_QUEUE_SIZE: usize = 1000;

thread_local! {
    static EXECUTOR:RefCell<Executor<'static>> = RefCell::new(Executor::new(EXECUTOR_QUEUE_SIZE))
}

pub(crate) fn spawn_task(future: impl Future<Output = ()> + 'static) {
    EXECUTOR.with(move |cell|{
        let mut exe = cell.borrow_mut();
        exe.spawn_task(future);
    })
}