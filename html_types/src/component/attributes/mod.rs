use crate::component::{
    document::DocumentItemRef,
    node::{IntoNode, Node, NodeData}
};

pub mod aria;
pub mod types;
pub use types::*;
mod internal;
pub use internal::*;

pub struct Attribute(pub(crate) DocumentItemRef, pub(crate) *mut AttributeData);

impl Attribute {
    pub fn name(&self) -> &str {
        unsafe{ (*(self.1)).name() }
    }

    pub fn value(&self) -> &AttributeValue {
        unsafe{ (*(self.1)).value() }
    }

    pub fn set_value(&mut self, value: impl ToAttributeValue) -> AttributeValue {
        unsafe{
            let inner = &mut (*self.1);
            inner.set_value(value)
        }
    }
}

impl IntoNode for Attribute {
    fn node(&self) -> Node {
        Node(
            self.0.clone()
        ) 
    }
}

fn perform_try_clone(value:&Node, inc:bool) -> Result<Attribute, &'static str> {
    match value.0.node_data() {
            NodeData::Attribute(inner) => {
                if inc {
                    value.0.item.inc();
                }

                Ok(
                    Attribute(
                        value.0.clone(),
                        inner as *const AttributeData as *mut AttributeData
                    )
                )
            },
            _ => Err("Unable to convert to Attribute!")
        }
}

impl TryFrom<Node> for Attribute {
    type Error = &'static str;

    fn try_from(value:Node) -> Result<Self, Self::Error> {
        perform_try_clone(&value, false)
    }
}

impl TryFrom<&Node> for Attribute {
    type Error = &'static str;

    fn try_from(value:&Node) -> Result<Self, Self::Error> {
        perform_try_clone(&value, true)
    }
}