use super::{JsonValue, JsonError};

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

impl JsonValue {
    pub fn integer(&self) -> Result<Type, JsonError> {
        match self {
            Self::Undefined => Ok(0),
            Self::Object(None) => Ok(0),
            Self::Array(a) => Ok(a.len() as Type),
            Self::Object(Some(o)) => Ok(o.len() as Type),
            _ => self.coarse_integer()
        }
    }

    pub fn coarse_integer(&self) -> Result<Type, JsonError> {
        match self {
            Self::Boolean(b) => if *b {
                Ok(1)
            } else {
                Ok(0)
            },
            Self::Integer(i) => Ok(*i), 
            Self::Number(n) => Ok(n.round() as Type),
            Self::String(s) =>  s.parse::<Type>().map_err(|_|{JsonError::NotAnInteger(self.type_of())}),
            _ => Err(JsonError::NotAnInteger(self.type_of())),
        }
    }
}