//pub mod attributes;
//pub mod elements;

pub(crate) mod node;
pub(crate) mod attributes;
pub use attributes::{AttributeName, AttributeValue};
pub(crate) mod document;
pub(crate) mod element;
pub(crate) mod other;