use std::{cell::RefCell, rc::Rc, collections::LinkedList};
use super::{
    node::*,
    attributes::{AttributeName, AttributeValue}
};

pub(crate) struct CdataSectionData {
    parrent: Option<Node>,
    children: LinkedList<Node>
}

impl PartialEq for CdataSectionData {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
    }
}

impl NodeInternalData for CdataSectionData {
    DefaultChildrenAccess!();
    DefaultParrentAccess!();
    StaticName!();
}

pub(crate) struct TextData {
    parrent: Option<Node>,
    pub value: String
}

impl PartialEq for TextData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl TextData {
    pub fn new(data:&str, parrent: Option<&Node>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self { 
                parrent: parrent.map(|n|n.node()),
                value: data.to_owned()
            }
        ))
    }
}

impl NodeInternalData for TextData {
    DefaultParrentAccess!();
    StaticName!();
}

pub(crate) struct CommentData {
    parrent: Option<Node>,
    pub value: String
}

impl PartialEq for CommentData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl NodeInternalData for CommentData {
    DefaultParrentAccess!();
    StaticName!();
}

pub(crate) struct AttributeData {
    parrent: Option<Node>,
    pub name: AttributeName,
    pub value: AttributeValue
}

impl NodeInternalData for AttributeData {
    DefaultParrentAccess!();
    StaticName!();
}

impl PartialEq for AttributeData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.value == other.value
    }
}