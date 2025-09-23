use std::{
    collections::LinkedList,
    rc::Rc,
    cell::RefCell
};
use super::{
    node::*,
    attributes::{AttributeItem, AttributeName},
    element::ElementData
    
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

impl Into<ElementData> for &DocumentData {
    fn into(self) -> ElementData {
        ElementData {
            name: AttributeName::Static("html"),
            attributes: self.attriubutes.clone(),
            parrent: None,
            children: self.children.iter()
                .map(|n|n.node())
                .collect()

        }
    }
}

impl NodeInternalData for DocumentData {
    DefaultChildrenAccess!();
    DefaultAttributeAccess!();
    StaticName!("html");
    
    fn parrent(&self) -> Option<&Node> {
        None
    }

    fn set_parrent(&mut self, _: Option<&Node>) {
        panic!("Attempted to set a parrent to the DocumentElement!")
    }
}

pub struct Document(Rc<RefCell<DocumentData>>);

impl IntoNode for Document {
    fn node(&self) -> Node {
        Node(NodeInternal::Document(self.0.clone()))
    }
}