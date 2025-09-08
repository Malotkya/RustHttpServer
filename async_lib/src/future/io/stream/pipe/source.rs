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

    fn poll<'a, P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self>;
}

impl<T> SourcePipe for Vec<T> {
    fn poll<'a, P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self> {
        Pipe::new(self, pipe)
    }
}

impl<T> SourcePipe for VecDeque<T> {
    fn poll<'a, P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self> {
        Pipe::new(self, pipe)
    }
}

impl<S: ?Sized + Stream + Unpin> SourcePipe for &mut S {

    fn poll<'a, P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self> {
        Pipe::new(self, pipe)
    }
}

impl<S: ?Sized + Stream + Unpin> SourcePipe for Box<S> {

    fn poll<'a, P: TargetPipe<Chunk: From<<Self as SourcePipe>::Chunk>>>(&'a mut self, pipe: &'a mut P) -> Pipe<'a, P, Self> {
        Pipe::new(self, pipe)
    }
}