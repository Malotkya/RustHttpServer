use super::{JsonValue, JsonError};

pub(crate) type Type = f64;

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        Self::Number(value.into())
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl JsonValue {
    pub fn number(&self) -> Result<Type, JsonError> {
        match self {
            Self::Undefined => Ok(0.0),
            Self::Object(None) => Ok(0.0),
            Self::Array(a) => Ok(a.len() as Type),
            Self::Object(Some(o)) => Ok(o.len() as Type),
            _ => self.coarse_number()
        }
    }

    pub fn coarse_number(&self) -> Result<Type, JsonError> {
        match self {
            Self::Boolean(b) => if *b {
                Ok(1.0)
            } else {
                Ok(0.0)
            },
            Self::Integer(i) => Ok(*i as Type), 
            Self::Number(n) => Ok(*n),
            Self::String(s) =>  s.parse::<Type>().map_err(|_|{JsonError::NotANumber(self.type_of())}),
            _ => Err(JsonError::NotANumber(self.type_of())),
        }
    }
}