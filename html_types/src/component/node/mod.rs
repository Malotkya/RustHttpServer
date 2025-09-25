use std::collections::VecDeque;
use super::{
    other::TextData,
    document::{
        DocumentItemRef,
        NodeDocumentItemRef,
        InternalRef
    },
    NodeIterator,
};


mod internal;
pub use internal::*;
mod macros;
pub(crate) use macros::*;

#[derive(PartialEq)]
pub struct Node(pub(crate) NodeDocumentItemRef);

pub trait IntoNode {
    fn node(&self) -> Node;
}

impl IntoNode for Node {
    fn node(&self) -> Node {
        Node(self.0.clone())
    }
}

impl<T: IntoNode> IntoNode for &T {
    fn node(&self) -> Node {
        (*self).node()
    }
}

//Internal Helper Fucntions
impl Node {
    pub(crate) fn is_visual_element(&self) -> bool {
        match &*self.0 {
            NodeData::Element(_) => true,
            NodeData::Text(_) => true,
            NodeData::Document(_) => true,
            NodeData::DocumentFragment(_) => true,
            _ => false
        }
    }

    pub(crate) fn new_helper(inner: &DocumentItemRef<impl NodeInternalData>) -> Self {
        Self(inner.downgrade())
    }
}

//https://developer.mozilla.org/en-US/docs/Web/API/Node
impl Node {
    //ToDo: fn base_uri(&self) -> String;

    pub fn child_nodes<'a>(&'a self) -> NodeIterator<'a> {
        self.0.children().into()
    }

    pub fn is_connected(&self) -> bool {
        //Consider server side a shadow document.
        false
    }

    pub fn first_child(&self) -> Option<Node> {
        self.child_nodes().next()
    }

    pub fn last_child(&self) -> Option<Node> {
        self.child_nodes().last()
    }

    pub fn next_sibling(&self) -> Option<Node> {
        if let Some(parrent) = self.0.parrent() {
            let mut it = parrent.child_nodes();
            while let Some(next) =  it.next() {
                if self.is_same_node(&next) {
                    return it.next()
                }
            }
        }

        None
    }

    pub fn node_name(&self) -> String {
        self.0.local_name().to_owned()
    }

    pub fn node_type(&self) -> NodeType {
        match &*self.0 {
            NodeData::Element(_) => NodeType::ElementNode,
            NodeData::Attribute(_) => NodeType::AttributeNode,
            NodeData::Text(_) => NodeType::TextNode,
            NodeData::CdataSection(_) => NodeType::CdataSectionNode,
            //Self::ProcessingInstruction => NodeType::ProcessingInstructionNode,
            NodeData::Comment(_) => NodeType::CommentNode,
            NodeData::Document(_) => NodeType::DocumentNode,
            NodeData::DocumentType(_) => NodeType::DocumentTypeNode,
            NodeData::DocumentFragment(_) => NodeType::DocumentFragmentNode
        }
    }

    //ToDo:pub fn owner_document(&self) -> Option<&Document>;

    pub fn parrent(&self) -> Option<Node> {
        self.0.parrent().map(|n|n.node())
    }

    pub fn previous_sibling(&self) -> Option<Node> {
        if let Some(parrent) = self.0.parrent() {
            let mut prev = None;
            let mut it = parrent.child_nodes();

            while let Some(next) =  it.next() {
                if self.is_same_node(&next) {
                    return prev;
                } else {
                    prev = Some(next)
                }
            }
        }

        None
    }

    pub fn get_text_content(&self) -> String {
        match &*self.0 {
            NodeData::Text(inner) => inner.value.clone(),
            NodeData::Comment(inner) => inner.value.clone(),
            _ => self.child_nodes()
                .map(|node|node.get_text_content())
                .collect::<Vec<String>>()
                .join(" ")
        }
    }

    pub fn set_text_content(&mut self, value: &str) -> Result<(), NodeError> {
        match unsafe{ self.0.borrow_mut() } {
            NodeData::Text(inner) => {
                inner.value = value.to_owned();
                Ok(())
            },
            NodeData::Comment(inner) => {
                inner.value = value.to_owned();
                Ok(())
            }
            _ => {
                let data = self.0.doc.create_text(TextData {
                    parrent: None,
                    value: value.to_owned()
                });

                unsafe {
                    self.0.borrow_mut().set_children(&[Node(
                        data.downgrade()
                    )])
                }
            }
        }
    }

    pub fn append_child<T: IntoNode>(&mut self, child:&T) -> Result<(), NodeError> {
        let mut child = child.node();

        // Check if there is a child list
        if self.0.children().is_void() {
            return Err(NodeError::NoDescendents)
        }
        
        unsafe {
            // Make sure to remove from previous location
            if let Some(parrent) = child.0.parrent()
                && let Some((mut parrent, index)) = find_child_helper(parrent, &child)? {

                parrent.0.borrow_mut().remove_child(index)?;
            }

            child.0.borrow_mut().set_parrent(Some(self));
            self.0.borrow_mut().add_child(child.node(), None)?;
        }
        

        Ok(())
    }

    pub fn clone_node(&self) -> Node {
        self.node()
    }

    pub fn compare_document_position<T: IntoNode>(&self, _other: &T) -> DocumentPosition {
        DocumentPosition::ImplementationSpecific
    }

    pub fn contains<T: IntoNode>(&self, other: &T) -> bool {
        let other = other.node();

        match find_child_helper(self, &other) {
            Ok(opt) => opt.is_some(),
            Err(_) => false
        }
    }

    pub fn get_root_node(&self) -> Option<Node> {
        self.0.borrow().parrent().map(|n|{
            match &*n.0 {
                NodeData::Document(_) => Some(n.node()),
                _ => n.get_root_node()
            }
        }).flatten() 
    }

    pub fn has_child_nodes(&self) -> bool {
        self.child_nodes().len() > 0
    }

    pub fn insert_before<T: IntoNode, Q: IntoNode>(&mut self, new_node:&T, reference:&Q) -> Result<(), NodeError> {
        let reference = reference.node();

        //Make sure refference is a child (Will fail if no list)
        if let Some((mut parrent, index)) = find_child_helper(self, &reference)? {
            let mut child = new_node.node();

            unsafe {
                // Make sure to remove from previous location
                if let Some(parrent) = child.0.borrow().parrent()
                    && let Some((mut parrent, index)) = find_child_helper(parrent, &child)? {

                    parrent.0.borrow_mut().remove_child(index)?;
                }

                child.0.borrow_mut().set_parrent(Some(&parrent));
                parrent.0.borrow_mut().add_child(child.node(), Some(index))?;
            }

            Ok(())
        } else {
            Err(NodeError::NoParrent,
)
        }
    }

    //ToDo: fn is_default_namespace(uri: String) -> bool; //Need to figure out how to manipulate inner trait

    pub fn is_equal_node<T: IntoNode>(&self, reference: &T) -> bool {
        let reference = reference.node();
        Node::eq(self, &reference)
    }

    pub fn is_same_node<T: IntoNode>(&self, reference: &T) -> bool {
        self.0.ptr() == reference.node().0.ptr()
    }

    //ToDo: fn normalize(&mut self);

    pub fn remove_child(&mut self, reference:&Node ) -> Result<(), NodeError>{
        if let Some((mut parrent, index)) = find_child_helper(self, reference)? {
            unsafe{ parrent.0.borrow_mut().remove_child(index) }
        } else {
            Err(NodeError::NoParrent)
        }
    }

    //ToDO:pub fn replace_child(&mut self, reference: &impl HtmlElement) -> Result<(), HtmlError>;

}

fn find_child_helper(parrent: &Node, child:&Node) -> Result<Option<(Node, usize)>, NodeError> {
    let it = parrent.child_nodes().enumerate();
    let mut list:VecDeque<Node> = VecDeque::with_capacity(it.len());
    
    for (index, node) in it {
        if node.is_same_node(child) {
            return Ok(Some((parrent.node(), index)));
        } else {
            list.push_back(node)
        }
    }

    while let Some(node) = list.pop_front() {
        let value = find_child_helper(&node, child)?;
        if value.is_some() {
            return Ok(value);
        }
    }

    Ok(None)

}

pub enum NodeError {
    NoParrent,
    NoDescendents,
}

impl std::fmt::Debug for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoParrent => write!(f, "Node does not have a parrent element to access!"),
            Self::NoDescendents => write!(f, "Node does not have the ability to take descendents!"),
        }
    }
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