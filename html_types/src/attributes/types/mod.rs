#![allow(unused_imports)]
use std::{
    collections::{HashSet, HashMap},
    ops::{Deref, DerefMut},
    str::FromStr
};

mod custom;
pub use custom::*;
mod enums;
pub use enums::*;

AttributeEnum!(
    Enumerable,
    Boolean
);

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

macro_rules! AttributeEnum {
    (
        $enum_name:ident,
        $( ($name:ident, $value:literal), )*
        Boolean
    ) => {
        $crate::attributes::AttributeEnum!(
            $enum_name,
            $( ($name, $value), )*
            (True, "true"),
            (False, "false")
        );

        impl From<bool> for $enum_name {
            fn from(value: bool) -> Self {
                if value {
                    Self::True
                } else {
                    Self::False
                }
            }
        }
    };
    (
        $enum_name:ident,
        $( ($name:ident, $value:literal) ),+
    ) => {
        pub enum $enum_name {
            $( $name ),+
        }

        impl $enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $value),+
                }
            }
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                self.as_str().to_string()
            }
        }

        impl TryFrom<&str> for $enum_name {
            type Error = String;

            fn try_from(value:&str) -> Result<Self, String> {
                match value.to_ascii_lowercase().as_str() {
                    $( $value => Ok(Self::$name), )*
                    _ => Err(format!("{} is not {}!", value, stringify!($enum_name)))
                }
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, String> {
                TryInto::<Self>::try_into(value.as_str())
            }
        }
    };
}

pub(crate) use AttributeEnum;