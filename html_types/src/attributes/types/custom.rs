use std::{
    collections::HashMap,
    ops::{Deref, DerefMut}
};

pub enum CustomAttributeValue {
    String(String),
    Number(f64),
    Boolean(bool)
}

impl From<usize> for CustomAttributeValue {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}

impl From<f64> for CustomAttributeValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for CustomAttributeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for CustomAttributeValue {
    fn from(value: String) -> Self {
        Self::String(value.clone())
    }
}

impl From<&str> for CustomAttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl ToString for CustomAttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Boolean(b) => if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
    }
}

pub struct CustomAttributes(HashMap<String, CustomAttributeValue>);

impl Deref for CustomAttributes {
    type Target = HashMap<String, CustomAttributeValue>;

    fn deref(&self) -> &Self::Target {
        & self.0
    }
}

impl DerefMut for CustomAttributes {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
