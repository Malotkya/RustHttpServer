use super::{JsonValue, JsonRef, JsonError};
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

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        match self {
            Self::Undefined => String::from(UNDEFINED),
            Self::Object(None) => String::from(NULL),
            Self::Boolean(value) => if *value {
                String::from(TRUE)
            } else {
                String::from(FALSE)
            },
            Self::Integer(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::String(value) => format!("\"{}\"", value),
            Self::Array(value) => super::array::stringify(value, None),
            Self::Object(Some(value)) => super::object::stringify(value, None)
        }
    }
}

impl JsonValue {
    pub fn string(&self) -> Type {
        match self {
            Self::Undefined => String::from(UNDEFINED),
            Self::Object(None) => String::from(NULL),
            Self::Array(value) => super::array::stringify(value, None),
            Self::Object(Some(value)) => super::object::stringify(value, None),
            _ => self.coarse_string().unwrap()
        }
    }

    pub fn coarse_string (&self)-> Result<Type, JsonError> {
        match self {
            Self::Boolean(b) => if *b {
                Ok(String::from(TRUE))
            } else {
                Ok(String::from(FALSE))
            },
            Self::Integer(i) => Ok(i.to_string()),
            Self::Number(n) => Ok(n.to_string()),
            Self::String(s) => Ok(s.clone()),
            _ => Err(JsonError::NotAString(self.type_of()))
        }
    }
}