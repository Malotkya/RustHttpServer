use crate::component::{
    document::{DocumentItemRef, InternalRef},
    node::{IntoNode, Node, NodeData}
};

pub mod aria;
pub mod types;
pub use types::*;
mod internal;
pub use internal::*;



pub struct Attribute(pub(crate) DocumentItemRef<AttributeData>);

impl Attribute {
    pub fn name(&self) -> &str {
        self.0.borrow().name()
    }

    pub fn value(&self) -> &AttributeValue {
        self.0.borrow().value()
    }
}

impl IntoNode for Attribute {
    fn node(&self) -> Node {
        Node(
            self.0.downgrade()
        ) 
    }
}

impl TryFrom<Node> for Attribute {
    type Error = &'static str;

    fn try_from(value:Node) -> Result<Self, Self::Error> {
        TryInto::<Self>::try_into(&value)
    }
}

impl TryFrom<&Node> for Attribute {
    type Error = &'static str;

    fn try_from(value:&Node) -> Result<Self, Self::Error> {
        match &*value.0 {
            NodeData::Attribute(inner) => {
                value.0.item.inc();

                Ok(
                    Self(
                        DocumentItemRef::new (
                            value.0.doc.clone(),
                            value.0.item,
                            inner
                        )
                    )
                )
            },
            _ => Err("Unable to convert to Attribute!")
        }
    }
}