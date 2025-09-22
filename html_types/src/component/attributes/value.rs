#![allow(unused_imports)]
use std::{
    collections::{HashSet, HashMap},
    ops::{Deref, DerefMut},
    str::FromStr
};
use super::SpaceSeperatedList;

pub use super::aria::types::*;

#[derive(Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    ClassList(SpaceSeperatedList),
    Boolean(bool)
}

impl ToAttributeValue for AttributeValue {
    fn into_value(&self) -> AttributeValue {
        self.clone()
    }
}

impl FromAttribteValue for AttributeValue {
    fn parse_from(value:&AttributeValue) -> Self {
        value.clone()
    }
}

impl AttributeValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s,
            Self::ClassList(list) => {
                list.as_str()
            },
            Self::Boolean(b) => if *b {
                "true"
            } else {
                "false"
            }
        }
    }

    pub fn is_truthy(&self) -> bool {
        match &self {
            Self::Boolean(b) => *b,
            Self::ClassList(_) => false,
            Self::String(s) =>
                s.to_ascii_lowercase().trim() == "true"
        }
    }

    pub(crate) fn is_list(&self) -> bool {
        match self {
            Self::ClassList(_) => true,
            _ => false
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut SpaceSeperatedList> {
        match self {
            Self::ClassList(list) => Some(list),
            _ => None
        }
    }

    pub fn list(&self) -> Option<& SpaceSeperatedList> {
        match self {
            Self::ClassList(list) => Some(list),
            _ => None
        }
    }

    pub fn from<T: ToAttributeValue>(value: T) -> Self {
        value.into_value()
    }

    pub fn try_parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.as_str().parse()
    }

    pub fn parse<T: FromAttribteValue>(&self) -> T {
        T::parse_from(self)
    }
}

pub trait ToAttributeValue {
    fn into_value(&self) -> AttributeValue;
}

impl ToAttributeValue for String {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl ToAttributeValue for &str {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl ToAttributeValue for f64 {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl ToAttributeValue for usize {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

impl ToAttributeValue for bool {
    fn into_value(&self) -> AttributeValue {
        AttributeValue::Boolean(*self)
    }
}

pub trait FromAttribteValue {
    fn parse_from(value:&AttributeValue) -> Self;
}

impl FromAttribteValue for String {
    fn parse_from(value:&AttributeValue) -> Self {
        value.as_str().to_owned()
    }
}

impl FromAttribteValue for bool {
    fn parse_from(value:&AttributeValue) -> Self {
        value.is_truthy()
    }
}
