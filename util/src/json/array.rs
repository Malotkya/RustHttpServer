use std::{collections::hash_map::Values, slice::Iter};
use super::{JsonValue, JsonRef, JsonMutRef, JsonError};

pub(crate) type Type = Vec<JsonValue>;

impl<I> From<Vec<I>> for JsonValue where I: Into<JsonValue> + Copy {
    fn from(value: Vec<I>) -> Self {
        Self::Array(value.iter().map(|x|(*x).into()).collect())
    }
}

impl<I> From<&[I]> for JsonValue where I: Into<JsonValue> + Copy {
    fn from(value:&[I]) -> Self {
        Self::Array(value.iter().map(|x|(*x).into()).collect())
    }
}

impl From<Vec<JsonValue>> for JsonValue {
    fn from(value:Vec<JsonValue>) -> Self {
        Self::Array(value)
    }
}

impl From<&[JsonValue]> for JsonValue {
    fn from(value:&[JsonValue]) -> Self {
        Self::Array(value.into())
    }
}

impl<I> FromIterator<I> for JsonValue where I: Into<JsonValue> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::Array(iter.into_iter().map(|t|t.into()).collect())
    }
}

impl TryInto<Type> for JsonValue {
    type Error = JsonError;

    fn try_into(self) -> Result<Vec<JsonValue>, Self::Error> {
        self.array()
            .ok_or(JsonError::NotAnArray(self))
    }
}

impl<'p> TryInto<Type> for JsonRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.array()
            .ok_or(JsonError::NotAnArray(value))
    }
}

impl<'p> TryInto<Type> for JsonMutRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.array()
            .ok_or(JsonError::NotAnArray(value))
    }
}

pub enum JsonValueIterator<'ptr> {
    Array(Iter<'ptr, JsonValue>),
    Object(Values<'ptr, String, JsonValue>),
    String(&'ptr str, isize),
    NotIterable
} 

impl<'a> Iterator for JsonValueIterator<'a> {
    type Item = JsonRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::NotIterable => None,
            Self::Array(it) => it.next().map(|v|v.into()),
            Self::Object(it) => it.next().map(|v|v.into()),
            Self::String(str, index) => if *index >= str.len() as isize {
                None
            } else {
                let ptr = unsafe {
                    str.as_ptr().byte_offset(*index) as *mut char
                };
                *index += 1;
                Some(ptr.into())
            }
        }
    }
}

impl<'a> IntoIterator for &'a JsonValue {
    type Item = JsonRef<'a>;
    type IntoIter = JsonValueIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            JsonValue::Undefined => JsonValueIterator::NotIterable,
            JsonValue::Boolean(_) |
            JsonValue::Integer(_) |
            JsonValue::Number(_) => JsonValueIterator::NotIterable,
            JsonValue::String(str) => JsonValueIterator::String(str, 0),
            JsonValue::Array(vec) => JsonValueIterator::Array(vec.iter()),
            JsonValue::Object(Some(map)) => JsonValueIterator::Object(map.values()),
            JsonValue::Object(None) => JsonValueIterator::NotIterable
        }
    }
}

pub(crate) fn stringify(value:&Type, indent:usize, inc:usize, sep:&str) -> String {
    format!(
        "[{}]",
        value.iter()
            .map(|v|super::_stringify(v, indent+inc, inc, sep))
            .collect::<Vec<String>>()
            .join(sep)
    )
}

impl JsonValue {
    pub fn array(&self) -> Option<Type> {
        match self {
            Self::String(str) => Some(
                str.chars()
                    .map(|char|char.into())
                    .collect()
            ),
            Self::Array(v) => Some(v.clone()),
            _ => None
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn array(&self) -> Option<&'p Type> {
        match self {
            Self::Array(ptr) => Some(*ptr),
            _ => None
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn array(&'p mut self) -> Option<&'p mut Type> {
        match self {
            Self::Array(ptr) => Some(*ptr),
            _ => None
        }
    }
}