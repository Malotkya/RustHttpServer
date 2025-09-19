use std::collections::LinkedList;
use super::{
    node::*,
    attributes::AttributeItem,
};

pub(crate) struct DocumentTypeData {
    parrent: Option<Node>
}

impl PartialEq for DocumentTypeData {
    fn eq(&self, other: &Self) -> bool {
        if let Some(lhs) = &self.parrent
            && let Some(rhs) = &other.parrent {
        
            lhs.is_same_node(rhs)
        } else {
            false
        }
    }
}

impl NodeInternalData for DocumentTypeData {
    DefaultParrentAccess!();
    StaticName!();
}

pub(crate) struct DocumentFragmentData {
    parrent: Option<Node>,
    children: LinkedList<Node>
}

impl PartialEq for DocumentFragmentData {
    fn eq(&self, other: &Self) -> bool {
        if let Some(lhs) = &self.parrent
            && let Some(rhs) = &other.parrent
            && lhs.is_same_node(rhs) {
        
            self.children == other.children
        } else {
                false
        }        
    }
}

impl NodeInternalData for DocumentFragmentData {
    DefaultChildrenAccess!();
    DefaultParrentAccess!();
    StaticName!();
}

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