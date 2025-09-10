use std::{collections::HashMap, string::ToString};
use super::{JsonValue, JsonError};

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

fn stringify_with_name(name:&str, value:&JsonValue, length:Option<usize>) -> String {
    let mut lines:Vec<String> = value.stringify(length)
        .split("\n").map(|x|x.to_owned())
        .collect();
    lines[0] = format!("{}: {}", name, lines[0]);

    if length.is_some() {
        let width = length.unwrap();
        lines.iter()
            .map(|str|format!("{:>width$}{str}", " "))
            .collect::<Vec<String>>().join("\n")
    } else {
        lines.join("\n")
    }
    
}

pub(crate) fn stringify(value:&Type, indent:Option<usize>) -> String {
    let (start, end) = super::gen_indent(indent);
    format!(
        "{{{start}{}{end}}}",
        value.iter()
        .map(|(k, v)|stringify_with_name(k, v, indent))
        .collect::<Vec<String>>().join(&end)
    )
}

impl JsonValue {
    pub fn object(&self) -> Result<Option<Type>, JsonError> {
        match self {
            Self::String(_value) => todo!(),
            _ => self.coarse_object()
        }
    }

    pub fn coarse_object(&self) -> Result<Option<Type>, JsonError> {
        match self {
            Self::Object(v) => Ok(v.clone()),
            _ => Err(JsonError::NotAnObject(self.type_of()))
        }
    }
}