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

impl<'p> JsonRef<'p> {
    pub fn to_owned(&self) -> JsonValue {
        match self {
            Self::Undefined => JsonValue::Undefined,
            Self::Boolean(b) => JsonValue::Boolean((*b).clone()),
            Self::Integer(i) => JsonValue::Integer((*i).clone()),
            Self::Number(n) => JsonValue::Number((*n).clone()),
            Self::String(s) => JsonValue::String((*s).clone()),
            Self::Array(a) => JsonValue::Array((*a).clone()),
            Self::Object(o) => JsonValue::Object((*o).clone()),
            Self::Char(c) => JsonValue::String(String::from(**c))
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn boolean(&self) -> Option<&'p bool> {
        match self {
            Self::Boolean(ptr) => Some(*ptr),
            _ => None
        }
    }

    pub fn integer(&self) -> Option<&'p integer::Type> {
        match self {
            Self::Integer(ptr) => Some(*ptr),
            _ => None
        }
    }

    pub fn number(&self) -> Option<&'p number::Type> {
        match self {
            Self::Number(ptr) => Some(*ptr),
            _ => None
        }
    }

    pub fn string(&self) -> Option<&'p str> {
        match self {
            Self::String(ptr) => Some(*ptr),
            Self::Char(ptr) => unsafe {
                Some(std::str::from_raw_parts(
                    (*ptr as *const char) as *const u8, 
                    1
                ))
            },
            _ => None
        }
    }

    pub fn array(&self) -> Option<&'p array::Type> {
        match self {
            Self::Array(ptr) => Some(*ptr),
            _ => None
        }
    }

    pub fn object(&self) -> Option<&'p object::Type> {
        match self {
            Self::Object(Some(ptr)) => Some(ptr),
            _ => None
        }
    }
}