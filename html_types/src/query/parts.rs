use crate::component::{
    attributes::{
        AttributeMatchOperator,
        ToAttributeValue,
        AttributeValue
    },
    element::Element
};
use super::{
    QueryFilter,
    functions::match_attribute
};

#[derive(Clone)]
pub struct Id(pub String);

impl QueryFilter for Id {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node, 
            "id", 
            Some(MatchOptions { 
                ops: AttributeMatchOperator::WhitespaceValue, 
                value: &self.0 
            })
        )
    }
}

#[derive(Clone)]
pub struct Class(pub String);

impl QueryFilter for Class {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node, 
            "class", 
            Some(MatchOptions { 
                ops: AttributeMatchOperator::WhitespaceValue, 
                value: &self.0 
            })
        )
    }
}

#[derive(Clone)]
pub struct Name {
    pub namespace: Option<String>,
    pub tag_name: String
}

impl QueryFilter for Name {
    fn filter(&self, node:&Element) -> bool {
        if let Some(match_namespace) = self.namespace.as_ref() 
            && match_namespace != "*" {
            
            if let Some(elm_namespace) = node.prefix() {
                if &elm_namespace != match_namespace {
                    return false
                }
            } else {
                return false;
            }
        }


        self.tag_name == node.local_name()
    }
}

#[derive(Clone)]
pub struct MatchOptions<T:ToAttributeValue = AttributeValue> {
    pub ops: AttributeMatchOperator,
    pub value:T
}

impl<T:ToAttributeValue> MatchOptions<T> {
    fn as_ref(&self) -> MatchOptions<&T> {
        MatchOptions {
            ops: self.ops.clone(),
            value: &self.value
        }
    }
}

#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub ops: Option<MatchOptions>
}

impl QueryFilter for Attribute {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node,
            &self.name,
            self.ops.as_ref()
                .map(|v|v.as_ref())
        )
    }
}