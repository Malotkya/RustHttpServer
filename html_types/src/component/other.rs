use std::{cell::RefCell, rc::Rc, collections::LinkedList};
use super::{
    node::*,
    attributes::{AttributeName, AttributeValue, AttributeItem},
    element::ElementData
};

NodeType!(
    NodeInternal::DocumentType = DocumentType();
    Data{parrent: Option<Node>}:(
        NodeInternalData:{
            DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                if let Some(lhs) = &self.parrent
                    && let Some(rhs) = &other.parrent {
                
                    lhs.is_same_node(rhs)
                } else {
                    false
                }
            }
        };
    )
);

NodeType!(
    NodeInternal::DocumentFragment = DocumentFragment();
    Data{
        parrent: Option<Node>,
        children: LinkedList<Node>
    }: (
        NodeInternalData:{
            DefaultChildrenAccess!();
            DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                if let Some(lhs) = &self.parrent
                    && let Some(rhs) = &other.parrent
                    && lhs.is_same_node(rhs) {
                
                    self.children == other.children
                } else {
                        false
                }
            } 
        };
    )
);

NodeType!(
    NodeInternal::CdataSection = CdataSection();
    Data{
        parrent: Option<Node>,
        children: LinkedList<Node>
    }:(
        NodeInternalData:{
            DefaultChildrenAccess!();
            DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.children == other.children
            }
        };
    )
);

NodeType!(
    NodeInternal::Text = Text(
        {
            pub(crate) fn new(value:&str) -> Self {
                Self(TextData::new(value, None))
            }

            fn value(&self) -> String {
                self.0.borrow().value.clone()
            }
        }
    );
    Data{
        parrent: Option<Node>,
        pub value: String
    }:(
        {
            pub(crate) fn new(data:&str, parrent: Option<&Node>) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(
                    Self { 
                        parrent: parrent.map(|n|n.node()),
                        value: data.to_owned()
                    }
                ))
            }
        };
        NodeInternalData: {
            DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        };
    )
);

impl Into<ElementData> for &TextData {
    fn into(self) -> ElementData {
        let mut list:LinkedList<Node> = LinkedList::new();
        list.push_front(Text::new(&self.value).node());

        ElementData {
            name: AttributeName::Static(""),
            attributes: Vec::new(),
            parrent: self.parrent.as_ref()
                .map(|n|n.node()),
            children: list
        }
    }
}

NodeType!(
    NodeInternal::Comment = Comment();
    Data{
        parrent: Option<Node>,
        pub value: String
    }:(
        NodeInternalData: {
            DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        };
    )
);

NodeType!(
    NodeInternal::Attribute = Attribute({
        pub(crate) fn new(value:&AttributeItem, parrent: Option<&impl IntoNode>) -> Self {
            Self(
                AttributeData::new(
                    value.clone(),
                    parrent.map(|n|n.node())
                )
            )
        }
    });
    Data{
        parrent: Option<Node>,
        pub name: AttributeName,
        pub value: AttributeValue
    }:(
        {
            pub fn new(value: AttributeItem, parrent: Option<Node>) -> Rc<RefCell<Self>> {
                Rc::new(
                    RefCell::new(
                        Self {
                            parrent,
                            name: value.0,
                            value: value.1
                        }
                    )
                )
            }

            pub fn item(&self) -> AttributeItem {
                AttributeItem(self.name.clone(), self.value.clone())
            }
        };
        NodeInternalData: {
             DefaultParrentAccess!();
            StaticName!();
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name
                    && self.value == other.value
            }
        };
    )
);