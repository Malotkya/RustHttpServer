use std::{
    io,
    task::{Poll, Context},
    pin::{Pin, pin}
};

use super::{Stream, FusedStream, Sink};

mod source;
pub use source::SourcePipe;
mod target;
pub use target::TargetPipe;

pub struct Pipe<
    'a,
    Target: TargetPipe,
    Source: SourcePipe<Chunk: Into<Target::Chunk>>
> {
    source: &'a mut Source,
    target: &'a mut Target,
    done: bool
}

impl<
    'a,
    Target: TargetPipe + SourcePipe,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>>
> Stream for Pipe<'a, Target, Source> {
    type Item = <Target as Stream>::Item;

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            self.target.poll_next(cx)
        } else {
            Poll::Pending
        }
    }
}

impl<
    't,
    Target: TargetPipe + SourcePipe,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>>
> SourcePipe for Pipe<'t, Target, Source> {

    fn poll<'a, P: TargetPipe<Chunk: From<<Target as Stream>::Item>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self> {
        Pipe::new(self, pipe)
    }
}

impl<
    'a,
    T: TargetPipe,
    S: SourcePipe<Chunk: Into<T::Chunk>>
> Pipe<'a, T, S> {

    pub fn new(source: &'a mut S, target: &'a mut T) -> Self {
        Self {
            source, target,
            done: false
        }
    }
}

impl <
    'a,
    Target: TargetPipe + SourcePipe,
    Source: SourcePipe<Chunk: Into<<Target as TargetPipe>::Chunk>>
> FusedStream for Pipe<'a, Target, Source> {

    fn is_terminated(&self) -> bool {
        self.done
    }
}

impl<
    'a,
    T: TargetPipe<Chunk: From<<S as Stream>::Item>>,
    S: SourcePipe<Chunk: Into<T::Chunk>>
> Future for Pipe<'a, T, S> {
    type Output = Result<(), T::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        if self.done {
            return Poll::Ready(Ok(()))
        }
        
        let into_chunk = match self.source.poll_next(cx) {
            Poll::Ready(o) => o,
            Poll::Pending => return Poll::Pending
        };

        let poll_continue: Poll<Result<bool, T::Error>> = match into_chunk {
            Some(value) => self.target.poll_accept_next(cx, value.into()).map(|r|r.map(|_|false)),
            None => self.target.poll_flush(cx).map(|r|r.map(|_|true))
        };

        match poll_continue {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => {
                self.done = true;
                Poll::Ready(Err(e))
            },
            Poll::Ready(Ok(false)) => Poll::Pending,
            Poll::Ready(Ok(true)) => {
                self.done = true;
                Poll::Ready(Ok(()))
            }
        }
    }
}