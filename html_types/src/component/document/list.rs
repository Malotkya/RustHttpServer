use crate::component::{
    document::{DocumentItemRef, Document},
    node::{Node, NodeData}
};
use std::{
    collections::LinkedList
};

const SECTION_SIZE:usize = 1000;

pub(crate) struct ListItem {
    pub(crate) data: NodeData,
    count: usize
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self as *const Self,
            other as *const Self
        )
    }
}

impl PartialEq<*mut ListItem> for &ListItem {
    fn eq(&self, other: &*mut ListItem) -> bool {
        std::ptr::eq(
            (*self) as *const ListItem,
            (*other) as *const ListItem
        )
    }
}

impl ListItem {
    pub fn new(data:NodeData) -> Self {
        Self {
            data,
            count: 0
        }
    }

    pub fn inc(self: *mut Self) -> usize {
        unsafe {
            (*self).count += 1;
            (*self).count
        }
    }

    pub fn dec(self: *mut Self) -> usize {
        unsafe {
            (*self).count += 1;
            (*self).count
        }
    }

    pub fn node(&self, doc:&Document) -> Node {
        Node(DocumentItemRef::new(
            doc,
            self,
        ))
    }
}

pub(crate) struct NodeArray(LinkedList<Vec<ListItem>>);

impl NodeArray {
    pub fn add(self: *mut Self, value:NodeData) -> *mut ListItem {
        let list = unsafe {
            &mut (*self).0
        };

        for section in list {
            if section.len() < section.capacity() {
                section.push(ListItem::new(value));
                return section.last().unwrap() as *const ListItem as *mut ListItem
            }
        }

        let list = unsafe {
            &mut (*self).0
        };

        let mut new_section = Vec::with_capacity(SECTION_SIZE);
        new_section.push(ListItem::new(value));
        list.push_back(new_section);

        list.back().unwrap().last().unwrap() as *const ListItem as *mut ListItem
    }

    pub fn find(&self, value:DocumentItemRef) -> Option<(usize, usize)> {
        for (outer, section) in self.0.iter().enumerate() {
            for (inner, item) in section.iter().enumerate() {
                if item == value.item {
                    return Some((outer, inner))
                }
            }
        }

        None
    }

    pub fn get(&self, mut section:usize, index:usize) -> Option<&ListItem> {
        let mut it = self.0.iter();

        while section >= 0 && let Some(next) = it.next() {
            if section == 0 {
                return next.get(index);
            } else {
                section -= 1;
            }
        }

        None
    }

    pub fn remove(self: *mut Self, ptr: *mut ListItem) -> bool {
        let list = unsafe {
            &mut (*self).0
        };

        unsafe {
            if (*ptr).count != 0 {
                return false;
            }
        }

        for section in list {
            for (index, item) in section.iter().enumerate() {
                if std::ptr::eq(
                    item as *const ListItem,
                    ptr as *const ListItem
                ) {
                    section.remove(index);
                    return true;
                }
            }
        }

        return false;
    }
}