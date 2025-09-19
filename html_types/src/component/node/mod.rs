use std::{
    collections::LinkedList,
    rc::Rc,
    cell::{RefCell, Ref, RefMut},
};
use super::{
    attributes::AttributeItem,
    document::{DocumentData, DocumentFragmentData, DocumentTypeData},
    element::ElementData,
    other::*
};

mod macros;
pub(crate) use macros::*;

#[allow(unused_variables)]
pub trait NodeInternalData {
    fn children(&self) -> Result<&LinkedList<Node>, NodeError> {
        Result::<&LinkedList<Node>, NodeError>::Err(NodeError::NoChildrenList)
    }
    fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError>{
        Err(NodeError::NoChildrenList)
    }
    fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
        Err(NodeError::NoChildrenList)
    }
    fn set_children(&mut self, list: LinkedList<Node>) -> Result<(), NodeError>{
        Err(NodeError::NoChildrenList)
    }

    fn attributes(&self) -> Option<&Vec<AttributeItem>> {
        None
    }

    fn name(&self) -> &str;
    fn parrent(&self) -> Option<&Node>;
    fn set_parrent(&mut self, parrent: Option<&Node>);

    fn is_void(&self) -> bool{
        self.children().is_err()
    }
}

#[derive(PartialEq)]
pub(crate) enum Node {
    Element(Rc<RefCell<ElementData>>),
    Attribute(Rc<RefCell<AttributeData>>),
    Text(Rc<RefCell<TextData>>),
    CdataSection(Rc<RefCell<CdataSectionData>>),
    //ProcessingInstruction,
    Comment(Rc<RefCell<CommentData>>),
    Document(Rc<RefCell<DocumentData>>),
    DocumentType(Rc<RefCell<DocumentTypeData>>),
    DocumentFragment(Rc<RefCell<DocumentFragmentData>>)
}

pub trait IntoNode {
    fn node(&self) -> Node;
}

impl IntoNode for Node {
    fn node(&self) -> Node {
        match self {
            Self::Attribute(inner) =>
                Self::Attribute(inner.clone()),
            Self::CdataSection(inner) =>
                Self::CdataSection(inner.clone()),
            Self::Comment(inner) =>
                Self::Comment(inner.clone()),
            Self::Document(inner) =>
                Self::Document(inner.clone()),
            Self::DocumentFragment(inner) =>
                Self::DocumentFragment(inner.clone()),
            Self::DocumentType(inner) =>
                Self::DocumentType(inner.clone()),
            Self::Element(inner) =>
                Self::Element(inner.clone()),
            Self::Text(inner) =>
                Self::Text(inner.clone()),
        }
    }
}

impl<T: IntoNode> IntoNode for &T {
    fn node(&self) -> Node {
        (*self).node()
    }
}

//Internal Helper Fucntions
impl Node {
    fn borrow(&self) -> Ref<'_, dyn NodeInternalData> {
        match self {
            Self::Attribute(inner) => inner.borrow(),
            Self::CdataSection(inner) => inner.borrow(),
            Self::Comment(inner) => inner.borrow(),
            Self::Document(inner) => inner.borrow(),
            Self::DocumentFragment(inner) => inner.borrow(),
            Self::DocumentType(inner) => inner.borrow(),
            Self::Element(inner) => inner.borrow(),
            Self::Text(inner) => inner.borrow(),
        }
    }

    fn borrow_mut(&self) -> RefMut<'_, dyn NodeInternalData> {
        match self {
            Self::Attribute(inner) => inner.borrow_mut(),
            Self::CdataSection(inner) => inner.borrow_mut(),
            Self::Comment(inner) => inner.borrow_mut(),
            Self::Document(inner) => inner.borrow_mut(),
            Self::DocumentFragment(inner) => inner.borrow_mut(),
            Self::DocumentType(inner) => inner.borrow_mut(),
            Self::Element(inner) => inner.borrow_mut(),
            Self::Text(inner) => inner.borrow_mut(),
        }
    }

    fn ptr_value(&self) -> *const dyn NodeInternalData {
        match self {
            Self::Attribute(inner) => inner.as_ptr(),
            Self::CdataSection(inner) => inner.as_ptr(),
            Self::Comment(inner) => inner.as_ptr(),
            Self::Document(inner) => inner.as_ptr(),
            Self::DocumentFragment(inner) => inner.as_ptr(),
            Self::DocumentType(inner) => inner.as_ptr(),
            Self::Element(inner) => inner.as_ptr(),
            Self::Text(inner) => inner.as_ptr(),
        }
    }
}

//https://developer.mozilla.org/en-US/docs/Web/API/Node
impl Node {
    //ToDo: fn base_uri(&self) -> String;

    pub fn child_nodes(&self) -> Vec<Node> {
        match self.borrow().children() {
            Ok(list) => list.iter().map(|n|n.node()).collect(),
            Err(_) => Vec::new()
        }
    }

    fn is_connected(&self) -> bool {
        //Consider server side a shadow document.
        false
    }

    pub fn first_child(&self) -> Option<Node> {
        match self.borrow().children() {
            Ok(list) => list.front().map(|n|n.node()),
            Err(_) => None
        }
    }

    pub fn last_child(&self) -> Option<Node> {
        match self.borrow().children() {
            Ok(list) => list.iter().last().map(|n|n.node()),
            Err(_) => None
        }
    }

    pub fn next_sibling(&self) -> Option<Node> {
        if let Some(parrent) = self.borrow().parrent() {
            if let Ok(list) = parrent.borrow().children() {
                let mut it = list.iter();

                while let Some(next) = it.next() {
                    if self.is_same_node(next) {
                        return it.next().map(|n|n.node())
                    }
                }
            }
        }

        None
    }

    pub fn node_name(&self) -> String {
        self.borrow().name().to_owned()
    }

    pub fn node_type(&self) -> NodeType {
        match self {
            Self::Element(_) => NodeType::ElementNode,
            Self::Attribute(_) => NodeType::AttributeNode,
            Self::Text(_) => NodeType::TextNode,
            Self::CdataSection(_) => NodeType::CdataSectionNode,
            //Self::ProcessingInstruction => NodeType::ProcessingInstructionNode,
            Self::Comment(_) => NodeType::CommentNode,
            Self::Document(_) => NodeType::DocumentNode,
            Self::DocumentType(_) => NodeType::DocumentTypeNode,
            Self::DocumentFragment(_) => NodeType::DocumentFragmentNode
        }
    }

    //ToDo: fn owner_document(&self) -> Option<&Document>;

    pub fn parrent(&self) -> Option<Node> {
        self.borrow().parrent().map(|n|n.node())
    }

    pub fn previous_sibling(&self) -> Option<Node> {
        if let Some(parrent) = self.borrow().parrent() {
            if let Ok(list) = parrent.borrow().children() {
                let mut prev = None;
                let mut it = list.iter();

                while let Some(next) = it.next() {
                    if self.is_same_node(next) {
                        return prev
                    } else {
                        prev = Some(next.node())
                    }
                }
            }
        }

        None
    }

    fn get_text_content(&self) -> String {
        match self {
            Self::Text(inner) => inner.borrow().value.clone(),
            Self::Comment(inner) => inner.borrow().value.clone(),
            _ => if let Ok(inner) = self.borrow().children() {
                inner.iter().map(|node|node.get_text_content())
                    .collect::<Vec<String>>().join(" ")
            } else {
                String::new()
            }
        }
    }

    fn set_text_content(&mut self, value: &str) -> Result<(), NodeError> {
        match self {
            Self::Text(inner) => {
                inner.borrow_mut().value = value.to_owned();
                Ok(())
            },
            Self::Comment(inner) => {
                inner.borrow_mut().value = value.to_owned();
                Ok(())
            }
            _ => {
                let mut list = LinkedList::new();
                list.push_back(
                    Node::Text(
                        TextData::new(value, Some(self))
                    )
                );
                self.borrow_mut().set_children(list)
            }
        }
    }

    pub fn append_child<T: IntoNode>(&mut self, child:&T) -> Result<(), NodeError> {
        let child = child.node();

        // Check if there is a child list
        if self.borrow().is_void() {
            return Err(NodeError::NoChildrenList)
        }
        
        let mut child_inner = child.borrow_mut();

        // Make sure to remove from previous location
        if let Some(parrent) = child_inner.parrent()
            && let Some((parrent, index)) = find_child_helper(parrent, &child)? {

            let mut inner = parrent.borrow_mut();
            inner.remove_child(index)?;
        }

        child_inner.set_parrent(Some(self));
        self.borrow_mut().add_child(child.node(), None)?;

        Ok(())
    }

    pub fn clone_node(&self) -> Node {
        self.node()
    }

    pub fn compare_document_position<T: IntoNode>(&self, _other: &T) -> DocumentPosition {
        DocumentPosition::ImplementationSpecific
    }

    fn contains<T: IntoNode>(&self, other: &T) -> bool {
        let other = other.node();

        match find_child_helper(self, &other) {
            Ok(opt) => opt.is_some(),
            Err(_) => false
        }
    }

    fn get_root_node(&self) -> Option<Node> {
        self.borrow().parrent().map(|n|{
            match n {
                Self::Document(_) => Some(n.node()),
                _ => n.get_root_node()
            }
        }).flatten()
    }

    fn has_child_nodes(&self) -> bool {
        match self.borrow().children() {
            Ok(list) => !list.is_empty(),
            Err(_) => false
        }
    }

    fn insert_before<T: IntoNode, Q: IntoNode>(&mut self, new_node:&T, reference:&Q) -> Result<(), NodeError> {
        let reference = reference.node();

        //Make sure refference is a child (Will fail if no list)
        if let Some((parrent, index)) = find_child_helper(self, &reference)? {
            let child = new_node.node();
            let mut child_inner = child.borrow_mut();

            // Make sure to remove from previous location
            if let Some(parrent) = child_inner.parrent()
                && let Some((parrent, index)) = find_child_helper(parrent, &child)? {

                parrent.borrow_mut().remove_child(index)?;
            }

            child_inner.set_parrent(Some(&parrent));
            parrent.borrow_mut().add_child(child.node(), Some(index))?;

            Ok(())
        } else {
            Err(NodeError::NotAChild)
        }
    }

    //ToDo: fn is_default_namespace(uri: String) -> bool;

    fn is_equal_node<T: IntoNode>(&self, reference: &T) -> bool {
        let reference = reference.node();
        Node::eq(self, &reference)
    }

    pub fn is_same_node<T: IntoNode>(&self, reference: &T) -> bool {
        std::ptr::eq(
            self.ptr_value(),
            reference.node().ptr_value()
        )
    }

    //ToDo: fn normalize(&mut self);

    pub fn remove_child(&mut self, reference:&Node ) -> Result<(), NodeError>{
        if let Some((parrent, index)) = find_child_helper(self, reference)? {
            parrent.borrow_mut().remove_child(index)
        } else {
            Err(NodeError::NotAChild)
        }
    }

    //ToDO: fn replace_child(&mut self, reference: &impl HtmlElement) -> Result<(), HtmlError>;

}

fn find_child_helper(parrent: &Node, child:&Node) -> Result<Option<(Node, usize)>, NodeError> {
    let inner = parrent.borrow();
    let list = inner.children()?;

    for (index, node) in list.iter().enumerate() {
        if node.is_same_node(child) {
            return Ok(Some((parrent.node(), index)));
        }
    }

    for node in list {
        let value = find_child_helper(&node, child)?;
        if value.is_some() {
            return Ok(value);
        }
    }

    Ok(None)

}

pub enum NodeError {
    NotAChild,
    NoChildrenList,
    NotElementCompatable
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
    ElementNode = 1,
    AttributeNode = 2,
    TextNode = 3,
    CdataSectionNode = 4,
    ProcessingInstructionNode = 7,
    CommentNode = 8,
    DocumentNode = 9,
    DocumentTypeNode = 10,
    DocumentFragmentNode = 11
}

#[repr(u8)]
pub enum DocumentPosition {
    Disconnected = 1,
    Preceding = 2,
    Following = 4,
    Contains = 8,
    ContainedBy = 16,
    ImplementationSpecific = 32
}