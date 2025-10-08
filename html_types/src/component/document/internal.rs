use std::{
    collections::LinkedList,
    rc::{Rc},
    ops::Deref
};
use crate::component::{
    node::*,
    element::ElementData,
    attributes::{
        AttributeData,
    },
    other::*,
    document::Document,
    document::list::*,
    iterator::*
};
use super::GenerateNodeFunctions;

pub(crate) struct DocumentData {
    pub(crate) all_nodes:NodeArray,
    pub(crate) doc: Document,
    children: LinkedList<(usize, usize)>,
    uri: String,
    cookie: String
}

impl DocumentData {
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

impl NodeInternalData for Rc<DocumentData> {
    StaticName!("html");

    fn children<'a>(&'a self) -> ChildIterator<'a> {
        ChildIterator::doc(
            self.children.iter(),
            &self.doc
        )
    }

    fn attributes<'a>(&'a self) -> AttributeIterator<'a> {
        AttributeIterator::doc(
            self.children.iter(),
            &self.doc
        )
    }
    
    fn parrent(&self) -> Option<&Node> {
        None
    }

    fn parrent_mut(&mut self) -> Option<&mut Node> {
        None
    }

    fn set_parrent(&mut self, _: Option<&Node>) {
        panic!("Attempted to set a parrent to the DocumentElement!")
    }
}

pub(crate) struct DocumentItemRef {
    pub doc: Document,
    pub item: *mut ListItem,
    data: Box<*mut dyn NodeInternalData>
}

impl DocumentItemRef {
    pub fn new(doc: &Document, item: &ListItem) -> Self {
        let data = item.data.inner() as *const dyn NodeInternalData;
        let item = item as *const ListItem;
        
        Self {
            doc: doc.clone(),
            item: item as *mut ListItem,
            data: Box::new(
                data as *mut dyn NodeInternalData
            )
        }
    }

    pub fn node_data(&self) -> &NodeData {
        unsafe{ &(*self.item).data }
    }

    pub fn node_data_mut(&mut self) -> &mut NodeData {
        unsafe{ &mut (*self.item).data }
    }

    pub fn borrow(&self) -> & dyn NodeInternalData {
        unsafe{ & (**self.data) }
    }

    pub unsafe fn borrow_mut(&mut self) -> &mut dyn NodeInternalData {
        unsafe{ &mut (**self.data) }
    }
}

impl Deref for DocumentItemRef {
    type Target = dyn NodeInternalData;

    fn deref(&self) -> &Self::Target {
        unsafe { &(**self.data) }
    }
}

impl PartialEq for DocumentItemRef {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self.item,
            other.item
        )
    }
}

impl Eq for DocumentItemRef {}

impl Clone for DocumentItemRef {

    fn clone(&self) -> Self {
        self.item.inc();

        Self {
            doc: self.doc.clone(),
            item: self.item,
            data: self.data.clone()
        }
    }
}

impl Drop for DocumentItemRef {
    fn drop(&mut self) {
        if self.item.dec() == 0 {
            self.doc.remove_node(self.item);
        }
    }
}