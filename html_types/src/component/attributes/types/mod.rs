use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
    str::FromStr
};
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
            AttributeValue::Boolean(b) => if *b {
                Self::Number(1.0)
            } else {
                Self::Number(0.0)
            }
        }
    }
}

pub struct SpaceSeperatedList(HashSet<String>);

impl Deref for SpaceSeperatedList {
    type Target = HashSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpaceSeperatedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for SpaceSeperatedList {
    fn from(value: &str) -> Self {
        Self(
            value.split_whitespace()
                .map(|s|s.to_string())
                .collect()
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
        self.0.iter().map(|s|s.clone())
            .collect::<Vec<String>>()
            .join(" ")
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

pub type BoolOrString = AttributeValue;

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