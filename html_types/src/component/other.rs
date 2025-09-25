use std::collections::LinkedList;
use super::{
    node::*
};

NodeType!(
    NodeData::DocumentType = DocumentType();
    Data{parrent: Option<Node>}:(
        NodeInternalData:{
            DefaultParrentAccess!();
            StaticName!();
        };
        Clone: {
            fn clone(&self) -> Self {
                Self { parrent: None }
            }
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
    NodeData::DocumentFragment = DocumentFragment();
    Data{
        pub parrent: Option<Node>,
        pub children: LinkedList<Node>
    }: (
        NodeInternalData:{
            DefaultChildrenAccess!();
            DefaultParrentAccess!();
            StaticName!();
        };
        Clone: {
            fn clone(&self) -> Self {
                Self {
                    parrent: None,
                    children: self.children.iter()
                        .map(|n|n.node())
                        .collect()
                }
            }
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
    NodeData::CdataSection = CdataSection();
    Data{
        pub parrent: Option<Node>,
        pub children: LinkedList<Node>
    }:(
        NodeInternalData:{
            DefaultChildrenAccess!();
            DefaultParrentAccess!();
            StaticName!();
        };
        Clone: {
            fn clone(&self) -> Self {
                Self {
                    parrent: None,
                    children: self.children.iter()
                        .map(|n|n.node())
                        .collect()
                }
            }
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.children == other.children
            }
        };
    )
);

NodeType!(
    NodeData::Text = Text(
        {
            fn value(&self) -> &str {
                unsafe{ &(*(self.1)).value }
            }
        };
    );
    Data{
        pub parrent: Option<Node>,
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
        Clone: {
            fn clone(&self) -> Self{
                Self {
                    parrent: None,
                    value: self.value.clone()
                }
            }
        };
    )
);

NodeType!(
    NodeData::Comment = Comment();
    Data{
        pub parrent: Option<Node>,
        pub value: String
    }:(
        NodeInternalData: {
            DefaultParrentAccess!();
            StaticName!();
        };
        Clone: {
            fn clone(&self) -> Self{
                Self {
                    parrent: None,
                    value: self.value.clone()
                }
            }
        };
        PartialEq: {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        };
    )
);