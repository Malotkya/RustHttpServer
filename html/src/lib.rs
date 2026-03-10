use std::string::ToString;
pub use attributes::{Attribute, AttributeValue};
pub use element::Element;

pub(crate) mod node;
pub mod element;
pub mod attributes;
pub mod function;


pub struct Comment(node::Node);

impl Comment {
    pub fn new<S:ToString>(value:S) -> Self {
        Self(node::Node::Comment(value.to_string()))
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn get_text(&self) -> String {
        self.0.get_text()
            .collect::<Vec<&str>>()
            .join(" ")
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


