#![allow(unused_imports)]
use std::{
    collections::{HashSet, HashMap},
    ops::{Deref, DerefMut},
    str::FromStr
};
pub use super::aria::types::*;

pub enum AttributeValue {
    String(String),
    Boolean(bool)
}

impl AttributeValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s ,
            Self::Boolean(b) => if *b {
                "true"
            } else {
                "false"
            }
        }
    }

    pub fn from<T: ToString>(value: T) -> Self {
        Self::String(value.to_string())
    }

    pub fn try_parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.as_str().parse()
    }

    pub fn parse<T: Default + FromStr>(&self) -> T {
        self.as_str().parse().unwrap_or(T::default())
    }
}
