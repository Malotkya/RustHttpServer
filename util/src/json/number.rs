use super::{JsonValue, JsonError, JsonRef, JsonMutRef};

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

impl TryInto<Type> for JsonValue {
    type Error = JsonError;

    fn try_into(self) -> Result<f64, Self::Error> {
        self.number()
            .ok_or(JsonError::NotANumber(self))
    }
}

impl<'p> TryInto<Type> for JsonRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<f64, Self::Error> {
        let value = self.value();

        value.number()
            .ok_or(JsonError::NotANumber(value))
    }
}

impl<'p> TryInto<Type> for JsonMutRef<'p> {
    type Error = JsonError;

    fn try_into(self) -> Result<Type, Self::Error> {
        let value = self.value();

        value.number()
            .ok_or(JsonError::NotAnArray(value))
    }
}

impl JsonValue {
    pub fn number(&self) -> Option<Type> {
        match self {
            Self::Undefined => Some(0.0),
            Self::Boolean(b) => if *b {
                Some(1.0)
            } else {
                Some(0.0)
            },
            Self::Integer(i) => Some(*i as Type), 
            Self::Number(n) => Some(*n),
            Self::String(s) =>  s.parse::<Type>().ok(),
            Self::Object(None) => Some(0.0),
            Self::Array(a) => Some(a.len() as Type),
            Self::Object(Some(o)) => Some(o.len() as Type),
        }
    }
}

impl<'p> JsonRef<'p> {
    pub fn number(&self) -> Option<&'p Type> {
        match self {
            Self::Number(ptr) => Some(*ptr),
            _ => None
        }
    }
}

impl<'p> JsonMutRef<'p> {
    pub fn number(&'p mut self) -> Option<&'p mut Type> {
        match self {
            Self::Number(ptr) => Some(*ptr),
            _ => None
        }
    }
}