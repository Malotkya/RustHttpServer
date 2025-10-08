use std::{
    rc::Rc,
    collections::LinkedList
};
use crate::component::{
    attributes::AttributeData,
    document::DocumentData,
    element::ElementData,
    node::{
        Node,
        NodeError
    },
    other::*,
    AttributeIterator,
    ChildIterator
};

#[allow(unused_variables)]
pub trait NodeInternalData {
    fn children<'a>(&'a self) -> ChildIterator<'a> {
        ChildIterator::empty()
    }
    fn add_children(&mut self, child:&[Node], index:Option<usize>) -> Result<(), NodeError>{
        Err(NodeError::NoDescendents)
    }
    fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
        Err(NodeError::NoDescendents)
    }
    fn set_children(&mut self, list: &[Node]) -> Result<(), NodeError>{
        Err(NodeError::NoDescendents)
    }

    fn attributes<'a>(&'a self) -> AttributeIterator<'a> {
        AttributeIterator::empty()
    }

    fn is_void(&self) -> bool {
        self.children().is_void()
    }

    fn namespace(&self) -> Option<&str> {
        None
    }
    fn local_name(&self) -> &str;

    fn inner(&self) -> Option<&LinkedList<Node>> {
        None
    }

    fn inner_mut(&mut self) -> Option<&mut LinkedList<Node>> {
        None
    }
    
    fn parrent(&self) -> Option<&Node>;
    fn parrent_mut(&mut self) -> Option<&mut Node>;
    fn set_parrent(&mut self, parrent: Option<&Node>);
}

#[derive(PartialEq, Clone)]
pub(crate) enum NodeData {
    Element(ElementData),
    Attribute(AttributeData),
    Text(TextData),
    CdataSection(CdataSectionData),
    //ProcessingInstruction,
    Comment(CommentData),
    Document(Rc<DocumentData>),
    DocumentType(DocumentTypeData),
    DocumentFragment(DocumentFragmentData)
}

impl NodeData {
    pub(crate) fn inner(&self) -> &dyn NodeInternalData {
        match self {
            Self::Attribute(inner) => inner as &dyn NodeInternalData,
            Self::CdataSection(inner) => inner as &dyn NodeInternalData,
            Self::Comment(inner) => inner as &dyn NodeInternalData,
            Self::Document(inner) => inner as &dyn NodeInternalData,
            Self::DocumentFragment(inner) => inner as &dyn NodeInternalData,
            Self::DocumentType(inner) => inner as &dyn NodeInternalData,
            Self::Element(inner) => inner as &dyn NodeInternalData,
            Self::Text(inner) => inner as &dyn NodeInternalData,
        } 
    }

    pub(crate) fn inner_mut(&mut self) -> &mut dyn NodeInternalData {
        match self {
            Self::Attribute(inner) => inner as &mut dyn NodeInternalData,
            Self::CdataSection(inner) => inner as &mut dyn NodeInternalData,
            Self::Comment(inner) => inner as &mut dyn NodeInternalData,
            Self::Document(inner) => inner as &mut dyn NodeInternalData,
            Self::DocumentFragment(inner) => inner as &mut dyn NodeInternalData,
            Self::DocumentType(inner) => inner as &mut dyn NodeInternalData,
            Self::Element(inner) => inner as &mut dyn NodeInternalData,
            Self::Text(inner) => inner as &mut dyn NodeInternalData,
        } 
    }

    pub(crate) fn ptr(&self) -> *const dyn NodeInternalData {
        match self {
            Self::Attribute(inner) => inner as *const dyn NodeInternalData,
            Self::CdataSection(inner) => inner as *const dyn NodeInternalData,
            Self::Comment(inner) => inner as *const dyn NodeInternalData,
            Self::Document(inner) => inner as *const dyn NodeInternalData,
            Self::DocumentFragment(inner) => inner as *const dyn NodeInternalData,
            Self::DocumentType(inner) => inner as *const dyn NodeInternalData,
            Self::Element(inner) => inner as *const dyn NodeInternalData,
            Self::Text(inner) => inner as *const dyn NodeInternalData,
        } 
    }
}