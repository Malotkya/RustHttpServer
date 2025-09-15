#![allow(unused_imports)]
use std::{
    collections::{HashSet, HashMap},
    ops::{Deref, DerefMut},
    str::FromStr
};
use super::{Attribute};

mod basic;
pub use basic::*;
mod enums;
pub use enums::*;

pub use super::aria::types::{
    AutoComplete as AriaAutoComplete,
    PopUp as AriaPopUp,
    Orientation as AriaOrientation,
    Pressed as AriaPressed,
    Sort as AriaSort,
    Live as AriaLive,
    Relevant as AriaRelevant,
    DropEffect as AriaDropEffect,
    Current as AriaCurrent,
};

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

impl Attribute for SpaceSeperatedList {
    fn generate(&self, name:&str) -> String {
        let mut it = self.0.iter();
        let mut string;

        if let Some(first) = it.next() {
            string = String::with_capacity(self.0.len() * 10);
            string.push_str(first);
        } else {
            string = String::new()
        };

        while let Some(next) = it.next() {
            string.push_str(next);
        }

        let mut output = String::with_capacity(name.len() + string.len() + 3);

        output.push_str(name);
        output.push_str("=\"");
        output.push_str(&string);
        output.push('"');

        output
    }
}

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

impl Attribute for Value {
    fn generate(&self, name:&str) -> String {
        let mut output = name.to_string();

        output.push_str("=\"");
        match self {
            Self::String(s) => output.push_str(s),
            Self::Number(f) => output.push_str(&f.to_string())
        };
        output.push('"');

        output
    }
}

struct CustomAttributes(HashMap<String, String>);

impl Deref for CustomAttributes {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomAttributes {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Attribute for CustomAttributes {
    fn generate(&self, _:&str) -> String {
        self.0.iter()
            .map(|(key, value)|basic_as_string(key, value))
            .collect::<Vec<String>>().join(" ")
    }
}