use crate::component::node::{
    DefaultParrentAccess, IntoNode, Node, NodeInternalData, StaticName
};
use super::types::SpaceSeperatedList;

mod macros;
pub(crate) use macros::*;
pub mod name;
pub use name::*;
pub mod value;
pub use value::*;



#[derive(PartialEq)]
pub(crate) struct AttributeData {
    pub namespace: Option<String>,
    pub parrent: Option<Node>,
    pub name: AttributeName,
    pub value: AttributeValue
}

impl Clone for AttributeData {
    fn clone(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            parrent: None,
            name: self.name.clone(),
            value: self.value.clone()
        }
    }
}

impl AttributeData {
    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    pub fn coarse_list(&mut self) -> &mut SpaceSeperatedList {
        if !self.value.is_list() {
            self.value = AttributeValue::ClassList(self.value.as_str().into());
        }
        
        self.value.list_mut().unwrap()
    }

    pub fn set_value<T: ToAttributeValue>(&mut self, value:T) -> AttributeValue {
        let old_value = self.value.clone();
        self.value = value.into_value();
        old_value
    }

    pub fn toggle_value(&mut self, value:bool) {
        self.value = AttributeValue::Boolean(value)
    }
}

impl ToString for AttributeData {
    fn to_string(&self) -> String {
        match &self.value {
            AttributeValue::Boolean(b) => if *b {
                self.name.value().to_owned()
            } else {
                String::new()
            },
            AttributeValue::ClassList(list) => {
                let key = self.name.value();
                let value = list.to_string();

                let mut output = String::with_capacity(key.len() + value.len() + 3);
                output.push_str(key);
                output.push_str("=\"");
                output.push_str(&value);
                output.push('"');

                output
            }
            AttributeValue::String(value) => {
                let key = self.name.value();

                let mut output = String::with_capacity(key.len() + value.len() + 3);
                output.push_str(key);
                output.push_str("=\"");
                output.push_str(value);
                output.push('"');
                output
            }
        }
    }
}

impl NodeInternalData for AttributeData {
    StaticName!("");
    DefaultParrentAccess!();

    fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref()
            .map(|s|s.as_str())
    }
}

