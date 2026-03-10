use std::{
    string::ToString,
    iter::Iterator
};
pub use attributes::{Attribute, AttributeValue};
pub use element::Element;
use element::{ElementRef, ElementMutRef};

pub mod node;
pub mod element;
pub mod attributes;
pub mod function;

pub trait Node: Sized{
    fn name(&self) -> &str;
    fn children(&self) -> impl Iterator<Item = ElementRef<'_>>;
    fn get_content(&self) -> String;
    fn get_inner_text(&self) -> String;
    fn stringify(&self) -> String;
}

pub trait NodeMut: Node {
    fn append<N:Into<Element>>(&mut self, value:N);
    fn remove<'a, N:Into<ElementRef<'a>>>(&mut self, node:N);
    fn clear(&mut self);
    fn children_mut(&mut self) -> impl Iterator<Item = ElementMutRef<'_>>;
    fn set_content<S:ToString>(&mut self, value:S);
    fn set_inner_text<S:ToString>(&mut self, value:S);
}

pub struct Comment(node::NodeData);

impl Comment {
    pub fn new<S:ToString>(value:S) -> Self {
        Self(node::NodeData::Comment(value.to_string()))
    }

    pub fn append_text<S:ToString>(&mut self, value:S) {
        self.0.append(value.to_string())
    }

    pub fn set_text<S:ToString>(&mut self, value:S) {
        self.0.set_text(value);
    }

    pub fn stringify(&self) -> String {
        self.0.to_string()
    }
}

#[allow(dead_code)]
pub struct EmptyIter<T>(*const T);

impl<T> EmptyIter<T> {
    fn new() -> Self {
        Self(std::ptr::null())
    }
}

impl<T> Iterator for EmptyIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Node for Comment {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn children(&self) -> impl Iterator<Item = ElementRef<'_>> {
        EmptyIter::new()
    }

    fn get_content(&self) -> String {
        self.get_inner_text()
    }

    fn get_inner_text(&self) -> String {
        self.0.get_text()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    fn stringify(&self) -> String {
        self.0.to_string()
    }
}

impl NodeMut for Comment {
    fn append<N:Into<Element>>(&mut self, value:N) {
        self.0.append(value.into().0)
    }

    fn remove<'b, R:Into<ElementRef<'b>>>(&mut self, child:R) {
        self.0.remove(child.into().0);
    }

    fn clear(&mut self) {
        self.0.clear();
    } 

    fn children_mut(&mut self) -> impl Iterator<Item = ElementMutRef<'_>> {
        EmptyIter::new()
    }

    fn set_content<S:ToString>(&mut self, value:S) {
        self.0.set_content(value);
    }

    fn set_inner_text<S:ToString>(&mut self, value:S) {
        self.0.set_text(value);
    }
}
