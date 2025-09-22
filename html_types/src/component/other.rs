use std::{cell::RefCell, rc::Rc, collections::LinkedList};
use super::{
    node::*,
    attributes::{AttributeName, AttributeValue, AttributeItem}
};

NodeType!(
    Node::CdataSection = CdataSection();
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
    Node::Text = Text(
        {
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
            pub fn new(data:&str, parrent: Option<&Node>) -> Rc<RefCell<Self>> {
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

NodeType!(
    Node::Comment = Comment();
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
    Node::Attribute = Attribute();
    Data{
        parrent: Option<Node>,
        pub name: AttributeName,
        pub value: AttributeValue
    }:(
        {
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