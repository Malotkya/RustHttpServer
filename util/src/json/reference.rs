use super::*;

#[derive(Clone, PartialEq)]
pub enum JsonRef<'ptr> {
    Boolean(&'ptr bool),
    Integer(&'ptr integer::Type),
    Number(&'ptr number::Type),
    String(&'ptr string::Type),
    Array(&'ptr array::Type),
    Object(&'ptr Option<object::Type>),
    Char(&'ptr char),
    Undefined
}

#[derive(PartialEq)]
pub enum JsonMutRef<'ptr> {
    Boolean(&'ptr mut bool),
    Integer(&'ptr mut integer::Type),
    Number(&'ptr mut number::Type),
    String(&'ptr mut string::Type),
    Array(&'ptr mut array::Type),
    Object(&'ptr mut Option<object::Type>),
    Char(&'ptr mut char),
    Undefined
}

impl<'p> From<&'p JsonValue> for JsonRef<'p> {
    fn from(value: &'p JsonValue) -> Self {
        match value {
            JsonValue::Undefined => Self::Undefined,
            JsonValue::Boolean(b) => Self::Boolean(b),
            JsonValue::Integer(i) => Self::Integer(i),
            JsonValue::Number(n) => Self::Number(n),
            JsonValue::String(s) => Self::String(s),
            JsonValue::Array(a) => Self::Array(a),
            JsonValue::Object(o) => Self::Object(o)
        }
    }
}

impl<'p> From<&'p mut JsonValue> for JsonMutRef<'p> {
    fn from(value: &'p mut JsonValue) -> Self {
        match value {
            JsonValue::Undefined => Self::Undefined,
            JsonValue::Boolean(b) => Self::Boolean(b),
            JsonValue::Integer(i) => Self::Integer(i),
            JsonValue::Number(n) => Self::Number(n),
            JsonValue::String(s) => Self::String(s),
            JsonValue::Array(a) => Self::Array(a),
            JsonValue::Object(o) => Self::Object(o)
        }
    }
}

impl<'p> Into<JsonValue> for JsonRef<'p> {
    fn into(self) -> JsonValue {
        self.value()
    }
}

impl<'p> Into<JsonValue> for JsonMutRef<'p> {
    fn into(self) -> JsonValue {
        self.value()
    }
}

impl<'p> JsonRef<'p> {
    pub fn value(&self) -> JsonValue {
        match self {
            Self::Boolean(b) => JsonValue::Boolean(**b),
            Self::Integer(i) => JsonValue::Integer(**i),
            Self::Number(n) => JsonValue::Number(**n),
            Self::String(s) => JsonValue::String(s.to_string()),
            Self::Char(c) => JsonValue::String(c.to_string()),
            Self::Array(a) => JsonValue::Array(
                a.iter().map(|v|v.clone())
                    .collect()
            ),
            Self::Object(None) => JsonValue::Object(None),
            Self::Object(Some(map)) => JsonValue::Object(Some(map.clone())),
            Self::Undefined => JsonValue::Undefined
        }
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => super::string::UNDEFINED,
            Self::Object(None) => super::string::NULL,
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "interger",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Char(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object"
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn value(&self) -> JsonValue {
        match self {
            Self::Boolean(b) => JsonValue::Boolean(**b),
            Self::Integer(i) => JsonValue::Integer(**i),
            Self::Number(n) => JsonValue::Number(**n),
            Self::String(s) => JsonValue::String(s.to_string()),
            Self::Char(c) => JsonValue::String(c.to_string()),
            Self::Array(a) => JsonValue::Array(
                a.iter().map(|v|v.clone())
                    .collect()
            ),
            Self::Object(None) => JsonValue::Object(None),
            Self::Object(Some(map)) => JsonValue::Object(Some(map.clone())),
            Self::Undefined => JsonValue::Undefined
        }
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => super::string::UNDEFINED,
            Self::Object(None) => super::string::NULL,
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "interger",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Char(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object"
        }
    }
}