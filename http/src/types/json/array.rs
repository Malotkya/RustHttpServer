use std::{collections::hash_map::Values, slice::Iter};
use super::{JsonValue, JsonRef, JsonError};

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

impl<I> FromIterator<I> for JsonValue where I: Into<JsonValue> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::Array(iter.into_iter().map(|t|t.into()).collect())
    }
}

enum JsonValueIterator<'ptr> where {
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

pub(crate) fn stringify(value:&Type, indent:Option<usize>) -> String {
    let (start, sep) = super::gen_indent(indent);

    format!(
        "[{start}{}{sep}]",
        value.iter()
        .map(|v|v.stringify(indent))
        .collect::<Vec<String>>().join(&sep)
    )
}

impl JsonValue {
    pub fn array(&self) -> Result<Type, JsonError> {
        match self {
            Self::String(_value) => todo!(),
            _ => self.coarse_array()
        }
    }

    pub fn coarse_array(&self) -> Result<Type, JsonError> {
        match self {
            Self::Array(v) => Ok(v.clone()),
            _ => Err(JsonError::NotAnArray(self.type_of()))
        }
    }
}