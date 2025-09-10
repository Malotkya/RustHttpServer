use super::{JsonValue, JsonError};

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

fn integer_to_bool(value:&super::integer::Type) -> Result<bool, ()> {
    match *value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(())
    }
}

fn number_to_bool(value:&super::number::Type) -> Result<bool, ()> {
    match value {
        0.0 => Ok(false),
        1.0 => Ok(true),
        _ => Err(())
    }
}

fn string_to_bool(value:&super::string::Type) -> Result<bool, ()> {
    match value.to_ascii_uppercase().as_str() {
        super::string::TRUE => Ok(true),
        super::string::FALSE => Ok(false),
        _ => match value.parse::<super::number::Type>() {
            Ok(n) => number_to_bool(&n),
            Err(_) => Err(())
        }
    }
}

impl JsonValue {
    pub fn bool(&self) -> bool {
        match self {
            Self::Array(a) => a.len() > 0,
            Self::Object(Some(o)) => o.len() > 0,
            _ => self.coarse_bool().unwrap_or(false)
        }
        
    }

    pub fn coarse_bool(&self) -> Result<bool, JsonError> {
        match self {
            Self::Boolean(b) => Ok(*b),
            Self::Integer(i) => integer_to_bool(i)
                .map_err(|_|JsonError::NotABoolean(self.type_of())),
            Self::Number(n) => number_to_bool(n)
                .map_err(|_|JsonError::NotABoolean(self.type_of())),
            Self::String(s) => string_to_bool(s)
                .map_err(|_|JsonError::NotABoolean(self.type_of())),
            _ => Err(JsonError::NotABoolean(self.type_of())),
        }
    }
}