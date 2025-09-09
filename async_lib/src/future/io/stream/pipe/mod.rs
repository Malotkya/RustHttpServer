use std::{
    ops::DerefMut,
    io,
    pin::{pin, Pin},
    task::{Context, Poll},
    sync::{Arc, Mutex},
    str::FromStr
};
use crate::EventEmitter;
use super::{Stream, FusedStream, Sink};

mod source;
pub use source::SourcePipe;
mod target;
pub use target::TargetPipe;

pub struct Pipe<
    Target: TargetPipe,
    Source: SourcePipe<Chunk: Into<Target::Chunk>>
> {
    source: *mut Source,
    target: *mut Target,
    emitter: EventEmitter,
    done: bool
}

pub(crate) struct PipeTask<
    Target: TargetPipe,
    Source: SourcePipe<Chunk: Into<Target::Chunk>>
>(*mut Pipe<Target, Source>);

unsafe impl<
    Target: TargetPipe,
    Source: SourcePipe<Chunk: Into<Target::Chunk>>
> Send for PipeTask<Target, Source> {}

unsafe impl<
    Target: TargetPipe,
    Source: SourcePipe<Chunk: Into<Target::Chunk>>
> Sync for PipeTask<Target, Source> {}

impl<
    Target: TargetPipe<Chunk: From<<Source as Stream>::Item>> + SourcePipe + 'static,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>> + 'static
> Stream for Pipe<Target, Source> {
    type Item = <Target as Stream>::Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            self.target().poll_next(cx)
        } else {
            Poll::Pending
        }
    }
}

impl<
    Target: TargetPipe<Chunk: From<<Source as Stream>::Item>> + SourcePipe + 'static,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>> + 'static,
> SourcePipe for Pipe<Target, Source> {

    fn pipe<T: TargetPipe<Chunk: From<<Target as Stream>::Item>> + 'static>(&mut self, pipe: &mut T) -> Pipe<T, Self> {
        Pipe::new(self, pipe)
    }
}

impl<
    Target: TargetPipe<Chunk: From<<Source as Stream>::Item>> + 'static,
    Source: SourcePipe<Chunk: Into<Target::Chunk>> + 'static
> Pipe<Target, Source> {

    pub fn new(source: &mut Source, target: &mut Target) -> Self {
        let mut pipe = Self {
            source, target,
            emitter: EventEmitter::new(),
            done: false
        };

        let mut task = PipeTask(
            &mut pipe as *mut Self
        );

        crate::spawn_task(std::future::poll_fn(move|cx|{
            task.poll(cx)
        }));
        
        pipe
    }

    pub fn source(&mut self) -> &mut Source {
        unsafe{ &mut (*self.source) }
    }

    pub fn target(&mut self) -> &mut Target {
        unsafe{ &mut (*self.target) }
    }

    pub fn on<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
        self.emitter.on(event, callback)
    }

    pub fn once<A: FromStr + 'static, E: Fn(A) + Sync + Send + 'static>(&mut self, event: &str, callback:E) -> String {
       self.emitter.once(event, callback)
    }

    pub fn remove_listener(&mut self, id: &str) -> bool {
        self.emitter.remove_listener(id)
    }

    pub fn emit<T: ToString>(&mut self, event: &str, value: T) {
        self.emitter.emit(event, value)
    }
}

impl <
    Target: TargetPipe<Chunk: From<<Source as Stream>::Item>> + SourcePipe + 'static,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>> + 'static,
> FusedStream for Pipe<Target, Source> {

    fn is_terminated(&self) -> bool {
        self.done
    }
}

impl<
    Target: TargetPipe<Chunk: From<<Source as Stream>::Item>> + 'static,
    Source: SourcePipe<Chunk: Into<Target::Chunk>> + 'static
> Future for Pipe<Target, Source> {
    type Output = Result<(), Target::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        if self.done {
            return Poll::Ready(Ok(()))
        }
        
        let into_chunk = match self.source().poll_next(cx) {
            Poll::Ready(o) => o,
            Poll::Pending => return Poll::Pending
        };

        let poll_continue: Poll<Result<bool, Target::Error>> = match into_chunk {
            Some(value) => self.target().poll_accept_next(cx, value.into()).map(|r|r.map(|_|false)),
            None => self.target().poll_flush(cx).map(|r|r.map(|_|true))
        };

        match poll_continue {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => {
                self.done = true;
                //self.emit("Error", format!("{:?}", e));
                Poll::Ready(Err(e))
            },
            Poll::Ready(Ok(false)) => Poll::Pending,
            Poll::Ready(Ok(true)) => {
                self.done = true;
                //self.emit("Done", "");
                Poll::Ready(Ok(()))
            }
        }
    }
}

impl<
    T: TargetPipe<Chunk: From<<S as Stream>::Item>> + 'static,
    S: SourcePipe<Chunk: Into<T::Chunk>> + 'static
> PipeTask<T, S> {
    fn poll(&mut self, cx: &mut Context<'_>) -> std::task::Poll<()> {
        let mut pipe = unsafe{ &mut (*self.0) };
        
        match Pipe::poll(Pin::new(&mut pipe), cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(_)) => {
                pipe.emit("Done", "");
                Poll::Ready(())
            },
            Poll::Ready(Err(e)) => {
                pipe.emit("Error", format!("{:?}", e));
                Poll::Ready(())
            }
        }
    }
}