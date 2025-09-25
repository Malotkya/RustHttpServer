use std::{
    collections::linked_list::Iter
};
use crate::component::{
    document::{Document, DocumentItemRef},
    element::Element,
    attributes::Attribute,
    node::{
        Node,
        IntoNode
    }
};

pub(crate) enum IteratorType<'d> {
    Document(Iter<'d, (usize, usize)>, Document),
    Node(Iter<'d, Node>),
    None
}

impl<'d> IteratorType<'d> {
    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }
}

pub(crate) struct InternalIterator<'d>(IteratorType<'d>);

impl<'d> InternalIterator<'d> {
    fn is_void(&self) -> bool {
        self.0.is_none()
    }

    pub fn empty() -> Self {
        Self(IteratorType::None)
    }

    pub(crate) fn doc(it: Iter<'d, (usize, usize)>, doc:&Document) -> Self {
        InternalIterator(
            IteratorType::Document(it, doc.clone())
        )
    }

    pub(crate) fn new(it: Iter<'d, Node>) -> Self {
        InternalIterator(
            IteratorType::Node(it)
        )
    }
}

impl<'d> Iterator for InternalIterator<'d> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IteratorType::Document(it, doc) => 
                it.next().map(|(outer, inner)|{
                    doc.0.all_nodes.get(*outer, *inner).map(|item|{
                        item.node(doc)
                    })
                }).flatten(),
            IteratorType::Node(it) => {
                it.next().map(|node|node.node())
            },
            IteratorType::None => None
        }
    }
}

impl<'d> DoubleEndedIterator for InternalIterator<'d> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IteratorType::Document(it, doc) => 
                it.next_back().map(|(outer, inner)|{
                    doc.0.all_nodes.get(*outer, *inner).map(|item|{
                        Node(DocumentItemRef::new(
                            &doc,
                            item
                        ))
                    })
                }).flatten(),
            IteratorType::Node(it) => {
                it.next_back().map(|node|node.node())
            },
            IteratorType::None => None
        }
    }
}

impl<'d> ExactSizeIterator for InternalIterator<'d> {
    fn len(&self) -> usize {
        match &self.0 {
            IteratorType::Document(it, _) => it.len(),
            IteratorType::Node(it) => it.len(),
            IteratorType::None => 0
        }
    }
}

macro_rules! BuildIterator {
    ($name:ident($type:path)=$convert:expr) => {
        pub struct $name<'d>(InternalIterator<'d>);

        impl<'d> $name<'d> {
            pub(crate) fn doc(it: Iter<'d, (usize, usize)>, doc:&Document) -> Self {
                Self(InternalIterator(
                    IteratorType::Document(it, doc.clone())
                ))
            }

            pub(crate) fn new(it: Iter<'d, Node>) -> Self {
                Self(InternalIterator(
                    IteratorType::Node(it)
                ))
            }

            pub(crate) fn empty() -> Self {
                Self(InternalIterator(
                    IteratorType::None
                ))
            }

            pub fn is_void(&self) -> bool {
                self.0.is_void()
            }
        }

        impl<'d> Iterator for $name<'d> {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                while let Some(next) = self.0.next() {
                    if let Some(value) = $convert(next) {
                        return Some(value)
                    }
                }

                None
            }

            fn last(mut self) -> Option<Self::Item> {
                while let Some(back) = self.0.next_back() {
                    if let Some(value) = $convert(back) {
                        return Some(value)
                    }
                }

                None
            }
        }

        impl<'d> ExactSizeIterator for $name<'d> {
            fn len(&self) -> usize {
                self.0.len()
            }
        }
    };
}

BuildIterator!(NodeIterator(Node)=|n:Node| -> Option<Node>{
    if n.is_visual_element() {
        Some(n)
    } else {
        None
    }
});

impl<'a> Into<NodeIterator<'a>> for ChildIterator<'a> {
    fn into(self) -> NodeIterator<'a> {
        NodeIterator(self.0)
    }
}

BuildIterator!(ChildIterator(Element)=|n:Node| -> Option<Element>{
    TryInto::<Element>::try_into(n).ok()
});
BuildIterator!(AttributeIterator(Attribute)=|n:Node| -> Option<Attribute> {
    TryInto::<Attribute>::try_into(n).ok()
});

