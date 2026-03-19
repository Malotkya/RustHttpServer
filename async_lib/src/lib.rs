#![feature(associated_type_defaults)]
#![feature(async_iterator)]
#![feature(trait_alias)]

mod event;
pub use event::*;
pub mod executor;
mod future;
pub use future::*;

#[allow(unused_imports)]
pub(crate) use executor::{spawn_task, queue_job, queue_process};

#[macro_export]
macro_rules! async_fn {
    (  
        $(clone=(
            $($clone_name:ident),+
        ),)?
        {$($body:tt)*}
    ) => {
        move || -> std::pin::Pin<Box<dyn Future<Output=()> + 'static>> {
            $(
                $( let $clone_name = $clone_name.clone();)+
            )?
            Box::pin(
                async move{$($body)*}
            )
        }
    };
}

