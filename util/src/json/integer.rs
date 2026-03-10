use super::{JsonValue, JsonError, JsonRef, JsonMutRef};

pub(crate) type Type = i128;

impl From<u8> for JsonValue {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i8> for JsonValue {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u16> for JsonValue {
    fn from(value: u16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i16> for JsonValue {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u32> for JsonValue {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u64> for JsonValue {
    fn from(value: u64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i128> for JsonValue {
    fn from(value: i128) -> Self {
        Self::Integer(value)
    }
}

impl TryInto<i128> for JsonValue {
    type Error = JsonError;

    fn try_into(self) -> Result<i128, Self::Error> {
        self.integer()
            .ok_or(JsonError::NotAnInteger(self))
    }
}

impl<'p> TryInto<i128> for JsonRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<i128, Self::Error> {
        let value = self.value();

        value.integer()
            .ok_or(JsonError::NotAnInteger(value))
    }
}

impl<'p> TryInto<Type> for JsonMutRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.integer()
            .ok_or(JsonError::NotAnArray(value))
    }
}

impl JsonValue {
    pub fn integer(&self) -> Option<Type> {
        match self {
            Self::Undefined => Some(0),
            Self::Boolean(b) => if *b {
                Some(1)
            } else {
                Some(0)
            },
            Self::Integer(i) => Some(*i), 
            Self::Number(n) => Some(n.round() as Type),
            Self::String(s) =>  s.parse::<Type>().ok(),
            Self::Object(None) => Some(0),
            Self::Array(a) => Some(a.len() as Type),
            Self::Object(Some(o)) => Some(o.len() as Type),
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn integer(&self) -> Option<&'p Type> {
        match self {
            Self::Integer(ptr) => Some(*ptr),
            _ => None
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn integer(&'p mut self) -> Option<&'p mut Type> {
        match self {
            Self::Integer(ptr) => Some(*ptr),
            _ => None
        }
    }
}