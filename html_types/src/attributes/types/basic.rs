use crate::attributes::{Attribute, AttributeEnum};
use std::ops::Deref;

AttributeEnum!(
    Enumerable,
    Boolean
);

#[inline]
pub(crate) fn basic_as_string(name:&str, value:&str) -> String {
    let mut output = String::with_capacity(value.len() + name.len() + 3);

    output.push_str(name);
    output.push_str("=\"");
    output.push_str(value);
    output.push('"');

    output
}

pub struct Boolean(bool);

impl Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Attribute for Boolean {
    fn generate(&self, name:&str) -> String {
        if self.0 {
            name.to_string()
        } else {
            String::new()
        }
    }
}

pub struct Integer(usize);

impl Deref for Integer {
    type Target =  usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Attribute for Integer {
    fn generate(&self, name:&str) -> String {
        basic_as_string(name, &self.0.to_string())
    }
}

pub struct Number(f64);

impl Deref for Number {
    type Target =  f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Attribute for Number {
    fn generate(&self, name:&str) -> String {
        basic_as_string(name, &self.0.to_string())
    }
}