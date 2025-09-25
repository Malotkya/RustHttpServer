use std::{
    collections::linked_list::Iter,
    rc::Rc
};
use crate::component::{
    document::{DocumentData, NodeDocumentItemRef},
    element::Element,
    node::Node
};

pub enum IteratorType<'d> {
    Document(Iter<'d, (usize, usize)>, Rc<DocumentData>),
    Node(Iter<'d, Node>),
    None
}

pub(crate) struct InternalIterator<'d>(IteratorType<'d>);

impl<'d> Iterator for InternalIterator<'d> {
    type Item = NodeDocumentItemRef;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IteratorType::Document(it, doc) => 
                it.next().map(|(outer, inner)|{
                    doc.all_nodes.get(*outer, *inner).map(|item|{
                        NodeDocumentItemRef::new(
                            doc.clone(),
                            item
                        )
                    })
                }).flatten(),
            IteratorType::Node(it) => {
                it.next().map(|node|node.0.clone())
            },
            IteratorType::None => None
        }
    }
}

macro_rules! BuildIterator {
    ($name:ident{
        $($body:tt)*
    }) => {
        pub struct $name<'d>(InternalIterator<'d>);

        impl<'d> $name<'d> {
            pub(crate) fn doc(it: Iter<'d, (usize, usize)>, doc:Rc<DocumentData>) -> Self {
                Self(InternalIterator(
                    IteratorType::Document(it, doc)
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
        }

        impl<'d> Iterator for $name<'d> {
            type Item = Element;

            fn next(&mut self) -> Option<Self::Item> {
                $($body)*
            }
        }
    };
}

BuildIterator!(ChildIterator{
    todo!()
});

BuildIterator!(AttributeIterator{
    todo!()
});