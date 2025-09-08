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

pub(crate) use executor::spawn_task;