use std::rc::Rc;
use crate::component::{
    attributes::AttributeData,
    document::DocumentData,
    element::ElementData,
    node::{
        Node,
        NodeError
    },
    other::*,
    ChildIterator,
    AttributeIterator
};

#[allow(unused_variables)]
pub trait NodeInternalData {
    fn children(&self) -> ChildIterator {
        ChildIterator::empty()
    }
    fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError>{
        Err(NodeError::NoDescendents)
    }
    fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
        Err(NodeError::NoDescendents)
    }
    fn set_children(&mut self, list: &mut[Node]) -> Result<(), NodeError>{
        Err(NodeError::NoDescendents)
    }

    fn attributes(&self) -> AttributeIterator {
        AttributeIterator::empty()
    }

    fn namespace(&self) -> Option<&str> {
        None
    }
    fn local_name(&self) -> &str;
    
    fn parrent(&self) -> Option<&Node>;
    fn set_parrent(&mut self, parrent: Option<&Node>);
}

#[derive(PartialEq)]
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
    fn inner(&self) -> &dyn NodeInternalData {
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

    fn inner_mut(&mut self) -> &mut dyn NodeInternalData {
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

impl NodeInternalData for NodeData {
    fn children(&self) -> ChildIterator {
        self.inner().children()
    }
    fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError>{
        self.inner_mut().add_child(child, index)
    }
    fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
        self.inner_mut().remove_child(index)
    }
    fn set_children(&mut self, list: &mut[Node]) -> Result<(), NodeError>{
        self.inner_mut().set_children(list)
    }

    fn attributes(&self) -> AttributeIterator {
        self.inner().attributes()
    }

    fn namespace(&self) -> Option<&str> {
        self.inner().namespace()
    }
    fn local_name(&self) -> &str {
        self.inner().local_name()
    }
    
    fn parrent(&self) -> Option<&Node>{
        self.inner().parrent()
    }

    fn set_parrent(&mut self, parrent: Option<&Node>){
        self.inner_mut().set_parrent(parrent);
    }
}