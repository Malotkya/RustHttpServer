use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::DerefMut,
    pin::Pin,
    io,
    task::{Context, Poll}
};
use super::super::Stream;
use super::{TargetPipe, Pipe};

pub trait SourcePipe: Stream + Sized {
    type Chunk = Self::Item;

    fn pipe<P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> Pipe<P, Self>;
}

impl<T: 'static> SourcePipe for Vec<T> {
    fn pipe<P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}

impl<T: 'static> SourcePipe for VecDeque<T> {
    fn pipe<P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}

/*impl<S: ?Sized + Stream + Unpin> SourcePipe for &mut S {

    fn pipe<P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}*/

impl<S: ?Sized + Stream + Unpin + 'static> SourcePipe for Box<S> {

    fn pipe<P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>> + 'static>(&mut self, pipe: &mut P) -> Pipe<P, Self> {
        Pipe::new(self, pipe)
    }
}