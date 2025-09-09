#![feature(box_vec_non_null)]
#![feature(associated_type_defaults)]
#![feature(maybe_uninit_slice)]
#![feature(trait_alias)]
#![allow(dead_code)]

mod event;
pub use event::EventEmitter;
pub mod executor;
mod future;
pub use future::*;

#[allow(unused_imports)]
pub(crate) use executor::{spawn_task, thread_await, thread_run};

#[macro_export]
macro_rules! async_fn {
    ($($body:tt)*) => {
        move || -> std::pin::Pin<Box<dyn Future<Output=()> + 'static>> {
            Box::pin(
                async move$($body)*
            )
        }
    };
}

