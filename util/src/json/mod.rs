use std::{
    fmt,
    iter::{Iterator, Enumerate},
    collections::{
        HashMap,
        hash_map::{Keys, Values, Iter as MapIter, IterMut as MapIterMut}
    },
    str::Chars,
    slice::{Iter, IterMut, from_raw_parts, from_raw_parts_mut}
};
pub use reference::{JsonRef, JsonMutRef};

mod number;
mod array;
mod integer;
mod string;
mod object;
mod boolean;
mod reference;

#[derive(Clone, PartialEq)]
pub enum JsonValue {
    Boolean(bool),
    Integer(integer::Type),
    Number(number::Type),
    String(string::Type),
    Array(array::Type),
    Object(Option<object::Type>),
    Undefined
}

pub enum KeyIter<'a> {
    Indexed(usize, usize),
    Object(Keys<'a, String, JsonValue>),
    None
}

pub enum JsonKey<'a> {
    Index(usize),
    Key(&'a str)
}

impl<'a> Iterator for KeyIter<'a> {
    type Item = JsonKey<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Indexed(i, len) => {
                *i += 1;
                if i < len {
                    Some(JsonKey::Index(*i))
                } else {
                    None
                }
            },
            Self::Object(keys) =>
                keys.next().map(|s|JsonKey::Key(s)),
            Self::None => None
        }
    }
}

pub enum ValueIter<'a> {
    String(Chars<'a>),
    Array(Iter<'a, JsonValue>),
    Object(Values<'a, String, JsonValue>),
    None
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String(chars) =>
                chars.next().map(|c|c.into()),
            Self::Array(it) =>
                it.next().map(|v|v.clone()),
            Self::Object(v) =>
                v.next().map(|v|v.clone()),
            Self::None => None
        }
    }
}

pub enum EntryIter<'a> {
    String(Enumerate<Iter<'a, char>>),
    Array(Enumerate<Iter<'a, JsonValue>>),
    Object(MapIter<'a, String, JsonValue>),
    None
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = (JsonKey<'a>, JsonRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String(chars) => chars.next()
                .map(|(i, c)|{
                    (JsonKey::Index(i), c.into())
                }),
            Self::Array(it) => it.next()
                .map(|(i, v)|{
                    (JsonKey::Index(i), v.into())
                }),
            Self::Object(entry) => entry.next()
                .map(|(key, value)|{
                    (JsonKey::Key(key), value.into())
                }),
            Self::None => None
        }
    }
}

pub enum EntryIterMut<'a> {
    String(Enumerate<IterMut<'a, char>>),
    Array(Enumerate<IterMut<'a, JsonValue>>),
    Object(MapIterMut<'a, String, JsonValue>),
    None
}

impl<'a> Iterator for EntryIterMut<'a> {
    type Item = (JsonKey<'a>, JsonMutRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String(chars) => chars.next()
                .map(|(i, c)|{
                    (JsonKey::Index(i), c.into())
                }),
            Self::Array(it) => it.next()
                .map(|(i, v)|{
                    (JsonKey::Index(i), v.into())
                }),
            Self::Object(entry) => entry.next()
                .map(|(key, value)|{
                    (JsonKey::Key(key), value.into())
                }),
            Self::None => None
        }
    }
}

pub enum JsonError {
    NotABoolean(JsonValue),
    NotAnInteger(JsonValue),
    NotANumber(JsonValue),
    NotAString(JsonValue),
    NotAnArray(JsonValue),
    NotAnObject(JsonValue)
}

impl JsonError {
    fn data(&self) -> (&str, &str, String) {
        match self {
            Self::NotABoolean(v) => ("boolean", v.type_of(), v.string()),
            Self::NotAnInteger(v) => ("integer", v.type_of(), v.string()),
            Self::NotANumber(v) => ("number", v.type_of(), v.string()),
            Self::NotAString(v) => ("string", v.type_of(), v.string()),
            Self::NotAnArray(v) => ("array", v.type_of(), v.string()),
            Self::NotAnObject(v) => ("object", v.type_of(), v.string())
        }
    }
}

impl fmt::Debug for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (expected, type_of, value) = self.data();
        write!(f, "JsonError{{ expected:{expected}, {type_of}:{value}}}")
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (expected, type_of, value) = self.data();
        write!(f, "Failed to convert {value} as {type_of} to {expected}!")
    }
}

impl JsonValue {
    pub fn keys(&self) -> KeyIter<'_> {
        match self {
            Self::String(s) => KeyIter::Indexed(0, s.len()),
            Self::Array(a) => KeyIter::Indexed(0, a.len()),
            Self::Object(Some(o)) => KeyIter::Object(o.keys()),
            _ => KeyIter::None
        }
    }

    pub fn values(&self) -> ValueIter<'_> {
        match self {
            Self::String(s) => ValueIter::String(s.chars()),
            Self::Array(a) => ValueIter::Array(a.iter()),
            Self::Object(Some(o)) => ValueIter::Object(o.values()),
            _ => ValueIter::None
        }
    }

    pub fn entries(&self) -> EntryIter<'_> {
        match self {
            Self::String(str) => unsafe {
                EntryIter::String(from_raw_parts(
                    str.as_ptr() as *const char,
                    str.len()
                ).iter().enumerate())
            },
            Self::Array(a) => EntryIter::Array(a.iter().enumerate()),
            Self::Object(Some(o)) => EntryIter::Object(o.iter()),
            _ => EntryIter::None
        }
    }

    pub fn entries_mut(&mut self) -> EntryIterMut<'_> {
        match self {
            Self::Object(Some(o)) => EntryIterMut::Object(o.iter_mut()),
            Self::Array(a) => EntryIterMut::Array(a.iter_mut().enumerate()),
            Self::String(s) => unsafe {
                EntryIterMut::String(from_raw_parts_mut(
                    s.as_mut_ptr() as *mut char,
                    s.len()
                ).iter_mut().enumerate())
            },
            _ => EntryIterMut::None
        }
    }

    pub fn from<T>(value:&T) -> Self where T: Into<Self> + Clone {
        value.clone().into()
    }

    pub fn to_ref<'a>(&'a self) -> JsonRef<'a> {
        self.into()
    }

    pub fn null() -> Self {
        Self::Object(None)
    }

    pub fn new() -> Self {
        Self::Object(Some(HashMap::new()))
    }

    pub fn with_capacity(capacity:usize) -> Self {
        Self::Object(Some(HashMap::with_capacity(capacity)))
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => string::UNDEFINED,
            Self::Object(None) => string::NULL,
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object"
        }
    }
    
}

pub(crate) fn _stringify(value:&JsonValue, indent:usize, inc:usize, sep:&str) -> String {
    format!("{:>indent$}{}", " ",
        match value {
            JsonValue::Object(Some(obj)) => object::stringify(obj, indent+inc, inc, sep),
            JsonValue::Array(arr) => array::stringify(arr, indent+inc, inc, sep),
            JsonValue::String(str) => format!("\"{}\"", str),
            _ => value.string()
        }
    )
}

pub fn stringify(value:&JsonValue, indent:Option<usize>) -> String {
    _stringify(value, 0, indent.unwrap_or(0), ",\n")
}

pub fn objectify(_string:&str) -> JsonValue {
    todo!("Objectify not yet implemented!")
}