use std::{
    cell::{RefCell, RefMut, Ref},
    collections::{
        LinkedList
    },
    ops::{Deref, DerefMut},
    rc::Rc
};
use super::attributes::AttributeItem;

const DEFAULT_ATTRIBUTE_CAPCITY:usize = 10;

enum NodeName {
    Static(&'static str),
    Alloc(String)
}

impl Clone for NodeName {
    fn clone(&self) -> Self {
        match self {
            Self::Static(s) => Self::Static(*s),
            Self::Alloc(s) => Self::Alloc(s.clone())
        }
    }
}

//https://developer.mozilla.org/en-US/docs/Web/API/Node
pub struct NodeInternal{
    pub(crate) name: NodeName,
    pub(crate) is_void: bool,
    pub(crate) attributes: Vec<AttributeItem>,
    pub(crate) parrent: Option<Node>,
    pub(crate) children: LinkedList<Node>
}

impl NodeInternal {
    pub fn new(name: NodeName, is_void:bool, attributes: Option<Vec<AttributeItem>>, parrent: Option<Node>) -> Self {
        Self {
            name, is_void, parrent,
            attributes: attributes.unwrap_or(Vec::with_capacity(DEFAULT_ATTRIBUTE_CAPCITY)),
            children: LinkedList::new()
        }
    }
}

fn remove_child_helper(mut parrent: RefMut<'_, NodeInternal>, child: &RefMut<'_, NodeInternal>, self_ref:&Node) -> bool {

    for (index, node) in parrent.children.iter().enumerate() {
        if node.is_same_node(self_ref) {
            parrent.children.remove(index);
            return true;
        }
    }

    for node in parrent.children.iter() {
        let inner = node.0.borrow_mut();
        if remove_child_helper(inner, child, self_ref) {
            return true;
        }
    }

    false
}

#[derive(Clone)]
pub struct Node(Rc<RefCell<NodeInternal>>);

impl Deref for Node {
    type Target = Rc<RefCell<NodeInternal>>;

    fn deref(&self) -> &Self::Target {
        & self.0
    }
}

impl DerefMut for Node {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Node {
    pub(crate) fn new(value: String) -> Self {
        Self(
            Rc::new(RefCell::new(
                NodeInternal::new(
                    NodeName::Alloc(value),
                    false,
                    None, 
                    None
                )
            ))
        )
    }

    pub(crate) fn new_static(value: &'static str, is_void:bool) -> Self {
        Self(
            Rc::new(RefCell::new(
                NodeInternal::new(
                    NodeName::Static(value),
                    is_void,
                    None, 
                    None
                )
            ))
        )
    }

    //fn base_uri(&self) -> String;

    pub fn child_nodes(&self) -> Vec<Node> {
        self.0.borrow().children.iter()
            .map(|v|v.clone())
            .collect()
    }

    //fn is_connected(&self) -> bool;

    pub fn first_child(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.children.front().map(|v|v.clone())
    }

    pub fn last_child(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.children.iter().last().map(|v|v.clone())
    }

    pub fn next_sibling(&self) -> Option<Node> {
        let inner = self.0.borrow();
        if let Some(p) = inner.parrent.as_ref() {
            let p = p.0.borrow();
            let mut it = p.children.iter();
            
            while let Some(next) = it.next() {
                if next.is_same_node(self) {
                    return it.next().map(|v|v.clone())
                }
            }
        }

        None
    }

    pub fn is_same_node(&self, reference: &Node) -> bool {
        std::ptr::eq(self.0.as_ptr(), reference.0.as_ptr())
    }

    pub fn node_name(&self) -> String {
        let inner = self.0.borrow();
        match &inner.name {
            NodeName::Alloc(s) => s.clone(),
            NodeName::Static(s) => (*s).to_owned()
        }
    }

    pub fn node_type(&self) -> NodeType {
        NodeType::ElementNode
    }

    //fn owner_document(&self) -> Option<&Document>;

    pub fn parrent(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.parrent.as_ref().map(|v|v.clone())
    }

    pub fn previous_sibling(&self) -> Option<Node> {
        let inner = self.0.borrow();
        if let Some(p) = inner.parrent.as_ref() {
            let p = p.0.borrow();
            let mut prev: Option<Node> = None;
            let mut it = p.children.iter();
            
            while let Some(next) = it.next() {
                if next.is_same_node(self) {
                    return prev
                } else {
                    prev = Some(next.clone())
                }
            }
        }

        None
    }

    //fn get_text_content(&self) -> String;
    //fn set_text_content(&mut self, value: &str);

    pub fn append_child(&mut self, child:Node) {
        let mut mut_ref = child.0.borrow_mut();
        if let Some(p) = &mut_ref.parrent {
            let p = p.0.borrow_mut();
            remove_child_helper(p, &mut_ref, self);
        }

        mut_ref.parrent = Some(self.clone());
        
        let mut inner = self.0.borrow_mut();
        inner.children.push_back(child.clone());
    }

    pub fn clone_node(&self) -> Node {
        let inner = self.0.borrow();
        Self(
            Rc::new(RefCell::new(
                NodeInternal {b
                    name: inner.name.clone(),
                    parrent: None, 
                    children: LinkedList::new()
                }
            ))
        )
    }

    pub fn compare_document_position(&self, other: &Node) -> DocumentPosition {

    }

    pub fn remove_child(&mut self, reference:&Node ) -> Result<(), NodeError>{
        if remove_child_helper(self.0.borrow_mut(), &reference.0.borrow_mut(), self) {
            Ok(())
        } else {
            Err(
                NodeError::NotAChild
            )
        }
    }

}

pub enum NodeError {
    NotAChild
}



/*pub trait Node {

    fn contains(other: &impl HtmlElement) -> bool;
    fn get_root_node(&self, composed: Option<bool>) -> Option<&impl HtmlElement>;
    fn has_child_nodes(&self) -> bool;
    fn insert_before(&mut self, reference: &impl HtmlElement) -> Result<(), HtmlError>;
    //fn is_default_namespace(uri: String) -> bool;
    fn is_equal_node(&self, reference: &impl HtmlElement) -> bool;
    is_same_node
    fn normalize(&mut self);
    fn remove_child(&mut self, reference: &impl HtmlElement) -> Result<(), HtmlError>;
    fn replace_child(&mut self, reference: &impl HtmlElement) -> Result<(), HtmlError>;
}*/

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