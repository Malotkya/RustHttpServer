use std::{
    rc::Rc
};

use super::{
    node::*,
    attributes::*,
    other::*
};

mod internal;
pub(crate) use internal::*;
mod list;
use list::ListItem;
mod macros;
pub(crate) use macros::*;

#[derive(Clone)]
pub struct Document(pub(crate) Rc<DocumentData>);

impl Document {
    pub(crate) fn remove_node(&self, node:*mut ListItem) {
        self.0.delete(node);
    }

    pub(crate) fn create_attribute(&self, name:&str, namespace:Option<&str>, value: impl ToAttributeValue) -> Attribute {
        let data = AttributeData {
            parrent: None,
            namespace: namespace.map(|s|s.to_string()),
            name: AttributeName::Alloc(name.to_string()),
            value: value.into_value()
        };
        let ptr = &data as *const AttributeData;
        Attribute(self.0.create_attribute(data), ptr as *mut AttributeData)
    }

    pub(crate) fn create_text_node(&self, value:&str) -> Text {
        let data = TextData{
            parrent: None,
            value: value.to_string()
        };

        let ptr = &data as *const TextData;
        Text(self.0.create_text(data), ptr as *mut TextData)

    }
}

impl IntoNode for Document {
    fn node(&self) -> Node {
        Node(DocumentItemRef::new(
            self,
            self.0.all_nodes.get(0,0)
                .unwrap()
        ))
    }
}

fn perform_try_clone(value:&Node, inc:bool) -> Result<Document, &'static str> {
    match value.0.node_data() {
            NodeData::Document(inner) => {
                if inc {
                    value.0.item.inc();
                }

                Ok(
                    Document(
                        inner.clone()
                    )
                )
            },
            _ => Err("Unable to convert to Document!")
        }
}

impl TryFrom<Node> for Document {
    type Error = &'static str;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        perform_try_clone(&value, false)
    }
}

impl TryFrom<&Node> for Document {
    type Error = &'static str;

    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        perform_try_clone(value, true)
    }
}