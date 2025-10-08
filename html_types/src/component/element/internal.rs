use std::collections::LinkedList;
use crate::component::{
    attributes::AttributeName,
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

    fn add_children(&mut self, list: &[Node], index:Option<usize>) -> Result<(), NodeError> {
        if self.is_void() {
            return Err(NodeError::NoDescendents)
        }
        let length = self.children.len();
        let index = index.unwrap_or(length);

        let mut new_list:std::collections::LinkedList<Node> = list.iter()
            .map(|n|n.node())
            .collect();

        if index == 0 {
            new_list.append(&mut self.children);
            self.children = new_list;
        } else if index >= length {
            self.children.append(&mut new_list);
        } else {
            let mut tail = self.children.split_off(index);
            self.children.append(&mut new_list);
            self.children.append(&mut tail);
        }
        
        Ok(())
    }

    fn set_children(&mut self, list: &[Node]) -> Result<(), NodeError> {
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

    fn inner(&self) -> Option<&LinkedList<Node>> {
        Some(& self.children)
    }

    fn inner_mut(&mut self) -> Option<&mut LinkedList<Node>> {
        Some(&mut self.children)
    }

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