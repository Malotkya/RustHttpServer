use std::collections::LinkedList;
use super::{
    node::*,
    attributes::AttributeItem,
};

pub(crate) struct DocumentData {
    pub(crate) attriubutes: Vec<AttributeItem>,
    pub(crate) children: LinkedList<Node>
}

impl PartialEq for DocumentData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self as *const DocumentData,
            other as *const DocumentData
        )
    }
}

impl NodeInternalData for DocumentData {
    DefaultChildrenAccess!();
    DefaultAttributeAccess!();
    StaticName!();
    
    fn parrent(&self) -> Option<&Node> {
        None
    }

    fn set_parrent(&mut self, _: Option<&Node>) {
        panic!("Attempted to set a parrent to the DocumentElement!")
    }
}