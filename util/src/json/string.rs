use crate::json::JsonError;

use super::{JsonValue, JsonRef, JsonMutRef};
use std::string::ToString;

pub(crate) const UNDEFINED:&'static str = "undefined";
pub(crate) const TRUE:&'static str = "true";
pub(crate) const FALSE:&'static str = "false";
pub(crate) const NULL:&'static str = "null";

pub(crate) type Type = String;

impl From<char> for JsonValue {
    fn from(value: char) -> Self {
        Self::String(String::from(value))
    }
}

impl From<String> for JsonValue {
    fn from(value:String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for JsonValue {
    fn from(value:&str) -> Self {
        Self::String(String::from(value))
    }
}

impl<'p> From<*mut char> for JsonRef<'p> {
    fn from(value:*mut char) -> Self {
        Self::Char(
            unsafe {
                &mut *value
            }
        )
    }
}

impl<'p> From<*const char> for JsonRef<'p> {
    fn from(value:*const char) -> Self {
        Self::Char(
            unsafe {
                &*value
            }
        )
    }
}

impl<'p> From<&'p char> for JsonRef<'p> {
    fn from(value: &'p char) -> Self {
        Self::Char(value)
    }
}

impl<'p> From<*mut char> for JsonMutRef<'p> {
    fn from(value:*mut char) -> Self {
        Self::Char(
            unsafe {
                &mut *value
            }
        )
    }
}

impl<'p> From<&'p mut char> for JsonMutRef<'p> {
    fn from(value: &'p mut char) -> Self {
        Self::Char(value)
    }
}

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        self.string()
    }
}

impl<'p> ToString for JsonRef<'p> {
    fn to_string(&self) -> String {
        self.value().string()
    }
}

impl<'p> ToString for JsonMutRef<'p> {
    fn to_string(&self) -> String {
        self.value().string()
    }
}

impl<'p> TryInto<&'p str> for JsonRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<&'p str, Self::Error> {
        self.string()
            .ok_or(JsonError::NotAString(self.value()))
    }
}

impl JsonValue {
    pub fn string(&self) -> Type {
        match self {
            Self::Undefined => String::from(UNDEFINED),
            Self::Boolean(b) => if *b {
                String::from(TRUE)
            } else {
                String::from(FALSE)
            },
            Self::Integer(i) => i.to_string(),
            Self::Number(n) => n.to_string(),
            Self::String(s) => s.clone(),
            Self::Object(None) => String::from(NULL),
            Self::Array(value) =>
                super::array::stringify(value, 0, 0, ", "),
            Self::Object(Some(value)) =>
                super::object::stringify(value, 0, 0, ", ")
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn string(&self) -> Option<&'p str> {
        match self {
            Self::String(ptr) => Some(*ptr),
            Self::Char(c) => unsafe {
                Some(std::str::from_raw_parts(
                    (*c as *const char) as *const u8,
                    1
                ))
            },
            _ => None
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn string(&'p mut self) -> Option<&'p mut str> {
        match self {
            Self::String(ptr) => Some(*ptr),
            Self::Char(c) => unsafe {
                Some(std::str::from_raw_parts_mut(
                    (*c as *mut char) as *mut u8,
                    1
                ))
            },
            _ => None
        }
    }
}