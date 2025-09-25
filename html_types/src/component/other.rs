use std::{
    collections::LinkedList,
    ops::Deref
};
use super::{
    node::*,
    //element::ElementData,
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
        parrent: Option<Node>,
        children: LinkedList<Node>
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
        parrent: Option<Node>,
        children: LinkedList<Node>
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
                &self.0.value
            }
        };
    );
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

/*impl Into<ElementData> for &TextData {
    fn into(self) -> ElementData {
        let mut data = ElementData {
            name: AttributeName::Static(""),
            attributes: Vec::new(),
            parrent: self.parrent.as_ref()
                .map(|n|n.node()),
            children: LinkedList::new()
        };

        let mut text_clone = self.clone();
        text_clone.parrent = Some(data);
        data.children.push_back(text_clone);

        data
    }
}*/

NodeType!(
    NodeData::Comment = Comment();
    Data{
        parrent: Option<Node>,
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