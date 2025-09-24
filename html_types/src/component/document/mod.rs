use std::{
    rc::Rc
};

mod internal;
pub(crate) use internal::*;
mod list;
mod macros;
pub(crate) use macros::*;







pub struct Document(Rc<DocumentData>);

/*impl IntoNode for Document {
    fn node(&self) -> Node {
        Node(NodeInternal::Document(self.0.clone()))
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
        match &value.0 {
            NodeInternal::Document(inner) => Ok(
                Document(inner.clone())
            ),
            _ => Err("Unable to convert to document!")
        }
    }
}*/