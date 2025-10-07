#![allow(unused_imports)]
use std::{
    collections::{HashSet, HashMap},
    ops::{Deref, DerefMut},
    str::FromStr
};
use crate::component::attributes::SpaceSeperatedList;

#[derive(Clone)]
pub enum AttributeMatchOperator {
    Exact,
    WhitespaceValue,
    HyphinMatch,
    Contains,
    Prefix,
    Suffix
}

#[derive(Clone)]
pub enum AttributeValue {
    String(String),
    ClassList(SpaceSeperatedList),
    Boolean(bool)
}

impl<T: ToAttributeValue> PartialEq<T> for AttributeValue {
    fn eq(&self, other: &T) -> bool {
        let other = other.into_value();
        if let Self::Boolean(value) = self {
            if *value {
                other.is_truthy()
            } else {
                other.as_str().to_lowercase() == "false"
            }
        } else {
            self.as_str() == other.as_str()
        }
    }
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

    #[inline]
    pub fn has<T: ToAttributeValue>(&self, value:&T) -> bool {
        self.as_str().find(value.into_value().as_str()).is_some()
    }

    #[inline]
    pub(crate) fn has_i<T: ToAttributeValue>(&self, value:&T) -> bool {
        self.as_str().to_lowercase()
            .find(&value.into_value().as_str().to_lowercase())
            .is_some()
    }

    #[inline]
    pub fn ends_with<T: ToAttributeValue>(&self, value:&T) -> bool {
        self.as_str().ends_with(value.into_value().as_str())
    }

    #[inline]
    pub(crate) fn ends_with_i<T: ToAttributeValue>(&self, value:&T) -> bool {
        self.as_str().to_lowercase()
            .ends_with(&value.into_value().as_str().to_lowercase())
    }

    #[inline]
    pub fn has_hyphin_value<T: ToAttributeValue>(&self, value: &T) -> bool {
        let value = value.into_value();
        if self.as_str() == value.as_str() {
            true
        } else {
            if let Some(index) = self.as_str().rfind('-') {
                &self.as_str()[index..] == value.as_str()
            } else {
                false
            }
        }
    }

    #[inline]
    pub(crate) fn has_hyphin_value_i<T: ToAttributeValue>(&self, value: &T) -> bool {
        let lhs = self.as_str().to_lowercase();
        let rhs = value.into_value().as_str().to_lowercase();

        if lhs == rhs {
            true
        } else {
            if let Some(index) = lhs.rfind('-') {
                &lhs[index..] == rhs.as_str()
            } else {
                false
            }
        }
    }

    #[inline]
    pub fn starts_with<T: ToAttributeValue>(&self, value:&T) -> bool {
        if let Some(index) = self.as_str().find(value.into_value().as_str()) {
            index == 0
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn starts_with_i<T: ToAttributeValue>(&self, value:&T) -> bool {
        if let Some(index) = self.as_str().to_lowercase()
                .find(&value.into_value().as_str().to_ascii_uppercase()) {
            index == 0
        } else {
            false
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

    pub fn compare<T: ToAttributeValue>(&self, operator:AttributeMatchOperator, other:&T) -> bool {
        match operator {
            AttributeMatchOperator::Prefix => self.starts_with(other),
            AttributeMatchOperator::Suffix => self.ends_with(other),
            AttributeMatchOperator::Contains => self.has(other),
            AttributeMatchOperator::Exact => Self::eq(self, other),
            AttributeMatchOperator::HyphinMatch => self.has_hyphin_value(other),
            AttributeMatchOperator::WhitespaceValue => match self {
                Self::ClassList(list) => list.has(other),
                _ => {
                    let other = other.into_value();
                    for str in self.as_str().split_whitespace() {
                        if str == other.as_str() {
                            return true;
                        }
                    }

                    false
                }
            }
        }
    }

    pub(crate) fn compare_insensitive<T: ToAttributeValue>(&self, operator:AttributeMatchOperator, other:&T) -> bool {
        match operator {
            AttributeMatchOperator::Prefix => self.starts_with_i(other),
            AttributeMatchOperator::Suffix => self.ends_with_i(other),
            AttributeMatchOperator::Contains => self.has_i(other),
            AttributeMatchOperator::HyphinMatch => self.has_hyphin_value_i(other),
            AttributeMatchOperator::Exact => self.as_str().to_ascii_lowercase()
                == ToAttributeValue::into_value(other).as_str().to_lowercase(),
            AttributeMatchOperator::WhitespaceValue => match self {
                Self::ClassList(list) => list.has_i(other),
                _ => {
                    let other = other.into_value().as_str().to_lowercase();
                    for str in self.as_str().split_whitespace() {
                        if str.to_lowercase() == other {
                            return true;
                        }
                    }

                    false
                }
            }
        }
    }
}

pub trait ToAttributeValue {
    fn into_value(&self) -> AttributeValue;
}

impl<T: ToAttributeValue> ToAttributeValue for &T {
    fn into_value(&self) -> AttributeValue {
        (*self).into_value()
    }
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
