use std::{
    rc::Rc
};
use super::{
    node::*
};

mod internal;
pub(crate) use internal::*;
mod list;
mod macros;
pub(crate) use macros::*;







pub struct Document(Rc<DocumentData>);

impl IntoNode for Document {
    fn node(&self) -> Node {
        Node(NodeDocumentItemRef::new(
            self.0.clone(),
            self.0.all_nodes.get(0,0).unwrap()
        ))
    }
}

impl TryFrom<Node> for Document {
    type Error = &'static str;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        TryInto::<Self>::try_into(&value)
    }
}

impl TryFrom<&Node> for Document {
    type Error = &'static str;

    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        match &*value.0 {
            NodeData::Document(inner) => {
                value.0.item.inc();

                Ok(
                    Document(inner.clone())
                )
            },
            _ => Err("Unable to convert to document!")
        }
    }
}