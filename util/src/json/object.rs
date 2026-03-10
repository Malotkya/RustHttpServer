use std::{collections::HashMap, string::ToString};
use super::{JsonValue, JsonError, JsonRef, JsonMutRef};

pub(crate) type Type = HashMap<String, JsonValue>;

impl<K, V> From<&HashMap<K, V>> for JsonValue
    where K: ToString,
          V: Into<JsonValue> + Clone { 

    fn from(value:&HashMap<K, V>) -> Self {
        Self::Object(Some(
            value.iter().map(|(key, value)|{
                (key.to_string(), JsonValue::from(value))
            }).collect()
        ))
    }
}

pub(crate) fn stringify(value:&Type, indent:usize, inc:usize, sep:&str) -> String {
    format!(
        "{{{}}}",
        value.iter()
            .map(|(k, v)|format!(
                "{:>indent$}{}:{}", " ",
                k,
                super::_stringify(v, indent+inc, inc, sep)
            ))
            .collect::<Vec<String>>()
            .join(sep)
    )
}

impl TryInto<Type> for JsonValue {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        self.object()
            .ok_or(JsonError::NotAnObject(self))
    }
}

impl<'p> TryInto<Type> for JsonRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.object()
            .ok_or(JsonError::NotAnObject(value))
    }
}

impl<'p> TryInto<Type> for JsonMutRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.object()
            .ok_or(JsonError::NotAnArray(value))
    }
}

impl JsonValue {
    pub fn object(&self) -> Option<Type> {
        match self {
            Self::String(str) => {
                let mut object = HashMap::with_capacity(str.len());

                for (index, char) in str.chars().enumerate() {
                    object.insert(index.to_string(), char.into());
                }

                Some(object)
            },
            Self::Array(a) => {
                let mut object = HashMap::with_capacity(a.len());

                for (index, value) in a.iter().enumerate() {
                    object.insert(index.to_string(), value.clone());
                }

                Some(object)
            }
            Self::Object(v) => v.clone(),
            _ => None
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn object(&self) -> Option<&'p Type> {
        match self {
            Self::Object(Some(ptr)) => Some(ptr),
            _ => None
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn object(&'p mut self) -> Option<&'p mut Type> {
        match self {
            Self::Object(Some(ptr)) => Some(ptr),
            _ => None
        }
    }
}