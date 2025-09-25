use std::collections::LinkedList;
use crate::component::{
    attributes::{
        Attribute, AttributeData,
        AttributeName, AttributeValue,
        SpaceSeperatedList, ToAttributeValue
    },
    document::{DocumentItemRef, InternalRef},
    node::{
        DefaultParrentAccess, IntoNode,
        Node, NodeError, NodeInternalData
    },
    AttributeIterator,
    ChildIterator
};

pub struct ElementData {
    pub(crate) namespace:Option<AttributeName>,
    pub(crate) name: AttributeName,
    pub(crate) parrent: Option<Node>,
    pub(crate) children: LinkedList<Node>
}

impl ElementData {
    fn is_void(&self) -> bool {
        match self.name.value() {
            "area" | "base" | "br" | "col" |
            "embed" | "hr" | "img" | "input" | 
            "link" | "meta" | "param" | "source" | 
            "track" | "wbr" => true,
            _ => false
        }
    }
}

impl Clone for ElementData {
    fn clone(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            parrent: None,
            children: self.children.iter()
                .map(|n|n.node())
                .collect()
        }
    }
}

impl DocumentItemRef<ElementData> {
    pub(crate) fn get_attribute(&self, name: &str, namespace:Option<&str>) -> Option<Attribute> {
        for node in &self.children {
            if let Ok(atr) = TryInto::<Attribute>::try_into(node) {
                if atr.0.namespace() == namespace && atr.0.name() == name {
                    return Some(atr)
                }
            }
        }

        None
    }

    pub(crate) fn remove_attribute(&mut self, name:&str, namespace:Option<&str>) {
        let mut pos:Option<usize> = None;

        for (index, node ) in self.children.iter().enumerate() {
            if let Ok(atr) = TryInto::<Attribute>::try_into(node) {
                if atr.0.namespace() == namespace && atr.0.name() == name {
                    pos = Some(index);
                    break;
                }
            }
        }

        if let Some(index) = pos {
            unsafe {
                let mut tail = self.borrow_mut().children.split_off(index);
                tail.pop_front();
                self.borrow_mut().children.append(&mut tail);
            }
        }
    }

    pub(crate) fn toggle_attribute(&mut self, name:&str, namespace:Option<&str>, value:bool) {
        if value {
            self.set_attribute(name, namespace, true);
        } else {
            self.remove_attribute(name, namespace);
        }
    }

    pub(crate) fn set_attribute<T: ToAttributeValue>(&mut self, name:&str, namespace:Option<&str>, value:T) -> Option<AttributeValue>{
        if let Some(mut atr) = self.get_attribute(name, namespace) {
           let old = unsafe {
            atr.0.borrow_mut().set_value(value)
           };
           Some(old)
        } else {
            let atr = Attribute(self.doc.create_attribute(AttributeData{
                namespace: None,
                name: AttributeName::Static("class"),
                parrent: Some(Node::new_helper(self)),
                value: value.into_value()
            }));

            unsafe {
                self.borrow_mut().children.push_back(atr.node());
            }
            None
        }
    }

    pub(crate) fn class(&mut self) -> *mut SpaceSeperatedList {
        if let Some(mut value) = self.get_attribute("class", None) {
            unsafe{ value.0.borrow_mut().coarse_list() as *mut SpaceSeperatedList }
        } else {
            let mut attribute = Attribute(self.doc.create_attribute(AttributeData{
                namespace: None,
                name: AttributeName::Static("class"),
                parrent: Some(Node::new_helper(self)),
                value: AttributeValue::ClassList(SpaceSeperatedList::new())
            }));

            unsafe{
                let ptr = attribute.0.borrow_mut().coarse_list() as *mut SpaceSeperatedList;
                self.borrow_mut().children.push_back(attribute.node());
                ptr
            }
        }
    }
}

impl PartialEq for ElementData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.namespace == other.namespace
            && self.children == other.children
    }
}

impl NodeInternalData for ElementData {
    DefaultParrentAccess!();

    fn local_name(&self) -> &str {
        self.name.value()
    }

    fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(|v|v.value())
    }

    fn attributes(&self) -> AttributeIterator {
        AttributeIterator::new(self.children.iter())
    }

    fn children(&self) -> ChildIterator {
        if self.is_void() {
            ChildIterator::empty()
        } else {
            ChildIterator::new(self.children.iter())
        }
    }

    fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError> {
        if self.is_void() {
            Err(NodeError::NoDescendents)
        } else {
            if let Some(index) = index {
                let mut tail = self.children.split_off(index);
                self.children.push_back(child);
                self.children.append(&mut tail);
            } else {
                self.children.push_back(child);
            }

            Ok(())
        }
    }

    fn set_children(&mut self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        if self.is_void() {
            Err(NodeError::NoDescendents)
        } else {

            let mut new_list = LinkedList::new();

            while let Some(next) = self.children.pop_front() {
                if !next.is_visual_element() {
                    new_list.push_back(next);
                }
            }

            for new in list {
                new_list.push_back(new.node());
            }
            
            self.children = new_list;

            Ok(())
        }
    }
}