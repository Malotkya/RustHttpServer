use std::{
    collections::LinkedList,
    rc::{Rc},
    ops::Deref
};
use crate::component::{
    node::*,
    element::ElementData,
    attributes::AttributeData,
    other::*,
    document::list::*,
    iterator::*
};
use super::GenerateNodeFunctions;

pub(crate) struct DocumentData {
    pub(crate) all_nodes:NodeArray,
    children: LinkedList<(usize, usize)>,
    uri: String,
    cookie: String
}

impl DocumentData {
    fn create_node(self: Rc<Self>, data:NodeData) -> NodeDocumentItemRef {
        let list = &self.all_nodes as *const NodeArray as *mut NodeArray;
        
        let ptr = NodeArray::add(list, data);
        let data = unsafe{ &(& *ptr).data as *const NodeData };
        
        NodeDocumentItemRef {
            doc: self.clone(),
            item: ptr,
            data: data as *mut NodeData
        }
    }

    GenerateNodeFunctions!(
        (create_element, Element),
        (create_attribute, Attribute),
        (create_comment, Comment),
        (create_text, Text),
        (create_data, CdataSection),
        //(crate_instruction, ProcessingInstruction),
        (create_fragment, DocumentFragment),
        (create_doc_type, DocumentType)
    );

    pub fn delete(&self, data:*mut ListItem) -> bool {
        (&self.all_nodes as *const NodeArray as *mut NodeArray).remove(data)
    }
}

impl PartialEq for DocumentData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self as *const DocumentData,
            other as *const DocumentData
        )
    }
}

/*impl Into<ElementData> for &DocumentData {
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
}*/

impl NodeInternalData for DocumentData {
    StaticName!("html");

    fn children(&self) -> ChildIterator {
        ChildIterator::doc(
            self.children.iter(),
            self
        )
    }

    fn attributes(&self) -> AttributeIterator {
        AttributeIterator::doc(
            self.children.iter(),
            self
        )
    }
    
    fn parrent(&self) -> Option<&Node> {
        None
    }

    fn set_parrent(&mut self, _: Option<&Node>) {
        panic!("Attempted to set a parrent to the DocumentElement!")
    }
}

pub(crate) struct DocumentItemRef<T:NodeInternalData + Sized> {
    pub doc: Rc<DocumentData>,
    item: *mut ListItem,
    data: *mut T
}

impl<T:NodeInternalData + Sized> DocumentItemRef<T> {
    pub fn new(doc: Rc<DocumentData>, item: *mut ListItem, inner:&T) -> Self {
        Self {
            doc, item,
            data: inner as *const T as *mut T
        }
    }

    pub fn downgrade(&self) -> NodeDocumentItemRef {
        self.item.inc();
        let data = unsafe{ &(& *self.item).data as *const NodeData };

        NodeDocumentItemRef {
            doc: self.doc.clone(),
            item: self.item,
            data: data as *mut NodeData
        }
    }
}

pub(crate) struct NodeDocumentItemRef {
    pub doc: Rc<DocumentData>,
    pub item: *mut ListItem,
    data: *mut NodeData
}

impl NodeDocumentItemRef {
    pub fn new(doc: Rc<DocumentData>, item:&ListItem) -> Self {
        let data = &item.data as *const NodeData;
        let item = item as *const ListItem;

        Self {
            doc,
            item: item as *mut ListItem,
            data: data as *mut NodeData
        }
    }
}

impl Deref for NodeDocumentItemRef {
    type Target = NodeData;

    fn deref(&self) -> &Self::Target {
        unsafe{ & (*self.data) }
    }
}

pub(crate) trait InternalRef {
    type Imp;

    fn borrow(&self) -> &Self::Imp;
    unsafe fn borrow_mut(&mut self) -> &mut Self::Imp;
    fn ptr(&self) -> usize;
}

impl<'a, T:NodeInternalData> InternalRef for DocumentItemRef<T> {
    type Imp = T;

    fn borrow(&self) -> &T {
        unsafe{ &(*self.data) }
    }

    fn ptr(&self) -> usize {
        self.item as usize
    }

    unsafe fn borrow_mut(&mut self) -> &mut T {
        unsafe{ &mut (*self.data) }
    }
}

impl InternalRef for NodeDocumentItemRef {
    type Imp = NodeData;

    fn borrow(&self) ->& Self::Imp {
        unsafe{ & (*self.data) }
    }

    fn ptr(&self) -> usize {
        self.item as usize
    }

    unsafe fn borrow_mut(&mut self) ->&mut Self::Imp {
        unsafe{ &mut (*self.data) }
    }
}

impl<'a, T:NodeInternalData> PartialEq for DocumentItemRef<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self.item,
            other.item
        )
    }
}

impl PartialEq for NodeDocumentItemRef {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self.item,
            other.item
        )
    }
}

impl<T:NodeInternalData> Eq for DocumentItemRef<T> {}
impl Eq for NodeDocumentItemRef {}

impl<T:NodeInternalData> Clone for DocumentItemRef<T> {

    fn clone(&self) -> Self {
        self.item.inc();

        Self {
            doc: self.doc.clone(),
            item: self.item,
            data: self.data
        }
    }
}

impl Clone for NodeDocumentItemRef {
    fn clone(&self) -> Self {
        self.item.inc();

        Self {
            doc: self.doc.clone(),
            item: self.item,
            data: self.data.clone()
        }
    }
}

impl<T:NodeInternalData> Drop for DocumentItemRef<T> {
    fn drop(&mut self) {
        if self.item.dec() == 0 {
            self.doc.delete(self.item);
        }
    }
}

impl Drop for NodeDocumentItemRef {
    fn drop(&mut self) {
        if self.item.dec() == 0 {
            self.doc.delete(self.item);
        }
    }
}