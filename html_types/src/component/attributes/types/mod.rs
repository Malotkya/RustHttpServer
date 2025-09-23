use std::str::FromStr;
use crate::component::attributes::{FromAttribteValue, AttributeValue, ToAttributeValue};
pub use super::aria::types::*;

mod enums;
pub use enums::*;
mod macros;
pub(crate) use macros::*;

pub enum Value {
    String(String),
    Number(f64)
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value.clone())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl TryInto<f64> for Value {
    type Error = <f64 as FromStr>::Err;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::String(s) => s.parse(),
            Self::Number(n) => Ok(n)
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string()
        }
    }
}

impl ToAttributeValue for Value {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl FromAttribteValue for Value {
    fn parse_from(value:&AttributeValue) -> Self {
        match value {
            AttributeValue::String(s) => Self::String(s.to_owned()),
            AttributeValue::ClassList(list) => Self::String(list.to_string()),
            AttributeValue::Boolean(b) => if *b {
                Self::Number(1.0)
            } else {
                Self::Number(0.0)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SpaceSeperatedList(String);

impl SpaceSeperatedList {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity * 10))
    }

    fn find(&self, value:&str, start:usize) -> Option<usize> {
        if start >= value.len() {
            None
        } else if let Some(index) = self.0[start..].find(value) {
            //Make sure value is not substring of a different value.
            let mut chars = self.0.chars();

            //Prev doesn't exist or is whitespace
            let prev = chars.nth(index-1);
            if prev.is_none() || prev.unwrap().is_whitespace() {

                //Next deosn't exists or is whitespace
                let next = chars.nth(index+value.len()+1);
                if next.is_none() || next.unwrap().is_whitespace() {
                    return Some(index)
                }

            }

            self.find(value, index)
        } else {
            None
        }
    }

    pub fn has<T: ToAttributeValue>(&self, value:&T) -> bool {
        self.find(value.into_value().as_str(), 0).is_some()
    }

    pub fn add<T: ToAttributeValue>(&mut self, value:&T) -> bool {
        let value = value.into_value();
        if self.find(value.as_str(), 0).is_none() {
            self.0.push(' ');
            self.0.push_str(value.as_str());
            true
        } else {
            false
        }
    }

    pub fn remove<T: ToAttributeValue>(&mut self, value:&T) -> bool {
        let value = value.into_value();
        if let Some(index) = self.find(value.as_str(), 0) {
            self.0.replace_range(index..index+value.as_str().len(), "");
            true
        } else {
            false
        }
    }

    pub fn toggle<T: ToAttributeValue>(&mut self, value:&T) -> bool {
        let value = value.into_value();
        
        if let Some(index) = self.find(value.as_str(), 0) {
            self.0.replace_range(index..index+value.as_str().len(), "");
            false
        } else {
            self.0.push(' ');
            self.0.push_str(value.as_str());
            true
        }
    }

    pub fn replace(&mut self, old_value:&str, new_value:&str) -> bool {
        if let Some(index) = self.find(old_value, 0) {
            self.0.replace_range(index..index+old_value.len(), new_value);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.0.split_whitespace().count()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn inner(&mut self) -> &mut String {
        &mut self.0
    }
}

impl From<&str> for SpaceSeperatedList {
    fn from(value: &str) -> Self {
        Self(
            value.to_owned()
        )
    }
}

impl From<String> for SpaceSeperatedList {
    fn from(value: String) -> Self {
        Into::<Self>::into(value.as_str())
    }
}

impl ToString for SpaceSeperatedList {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl ToAttributeValue for SpaceSeperatedList {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl FromAttribteValue for SpaceSeperatedList {
    fn parse_from(value:&AttributeValue) -> Self {
        value.as_str().into()
    }
}

pub enum BoolOrString {
    String(String),
    Boolean(bool)
}

impl From<String> for BoolOrString {
    fn from(value: String) -> Self {
        Self::String(value.clone())
    }
}

impl From<bool> for BoolOrString {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl ToString for BoolOrString {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Boolean(b) => if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
    }
}