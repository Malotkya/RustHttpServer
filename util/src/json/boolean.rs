use super::{JsonValue, JsonRef, JsonMutRef};

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl Into<bool> for JsonValue {
    fn into(self) -> bool {
        self.truthy()
    }
}

impl<'p> Into<bool> for JsonRef<'p> {
    fn into(self) -> bool {
        self.truthy()
    }
}

impl<'p> Into<bool> for JsonMutRef<'p> {
    fn into(self) -> bool {
        self.truthy()
    }
}

fn integer_to_bool(value:&super::integer::Type) -> Option<bool> {
    match *value {
        0 => Some(false),
        1 => Some(true),
        _ => None
    }
}

fn number_to_bool(value:&super::number::Type) -> Option<bool> {
    match value {
        0.0 => Some(false),
        1.0 => Some(true),
        _ => None
    }
}

fn string_to_bool(value:&super::string::Type) -> Option<bool> {
    match value.to_ascii_lowercase().trim() {
        super::string::TRUE => Some(true),
        super::string::FALSE => Some(false),
        _ => match value.parse::<super::number::Type>() {
            Ok(n) => number_to_bool(&n),
            Err(_) => None
        }
    }
}

impl JsonValue {
    pub fn bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            Self::Integer(i) => integer_to_bool(i),
            Self::Number(n) => number_to_bool(n),
            Self::String(s) => string_to_bool(s),
            Self::Array(a) => Some(a.len() > 0),
            Self::Object(None) => Some(false),
            Self::Object(Some(o)) => Some(o.len() > 0),
            Self::Undefined => Some(false)
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Integer(i) => *i != 0,
            Self::Number(n) => *n != 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => false,
                "0" => false,
                _ => s.trim().len() == 0
            },
            Self::Array(a) => a.len() > 0,
            Self::Object(o) => o.is_some(),
            Self::Undefined => false
        }
    }

    pub fn falsy(&self) -> bool {
        match self {
            Self::Boolean(b) => !*b,
            Self::Integer(i) => *i == 0,
            Self::Number(n) => *n == 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => true,
                "0" => true,
                _ => s.trim().len() == 0
            },
            Self::Array(a) => a.len() == 0,
            Self::Object(o) => o.is_none(),
            Self::Undefined => true,
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

    pub fn truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => **b,
            Self::Integer(i) => **i != 0,
            Self::Number(n) => **n != 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => false,
                "0" => false,
                _ => s.trim().len() == 0
            },
            Self::Char(c) => match *c {
                '0' => false,
                _ => !c.is_ascii_whitespace()
            },
            Self::Array(a) => a.len() > 0,
            Self::Object(o) => o.is_some(),
            Self::Undefined => false
        }
    }

    pub fn falsy(&self) -> bool {
        match self {
            Self::Boolean(b) => !**b,
            Self::Integer(i) => **i == 0,
            Self::Number(n) => **n == 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => true,
                "0" => true,
                _ => s.trim().len() == 0
            },
            Self::Char(c) => match *c {
                '0' => true,
                _ => c.is_ascii_whitespace()
            },
            Self::Array(a) => a.len() == 0,
            Self::Object(o) => o.is_none(),
            Self::Undefined => true
        }
    }
} 

impl<'p> JsonMutRef<'p> {
    pub fn boolean(&'p mut self) -> Option<&'p mut bool> {
        match self {
            Self::Boolean(ptr) => Some(*ptr),
            _ => None
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => **b,
            Self::Integer(i) => **i != 0,
            Self::Number(n) => **n != 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => false,
                "0" => false,
                _ => s.trim().len() == 0
            },
            Self::Char(c) => match *c {
                '0' => false,
                _ => !c.is_ascii_whitespace()
            },
            Self::Array(a) => a.len() > 0,
            Self::Object(o) => o.is_some(),
            Self::Undefined => false
        }
    }

    pub fn falsy(&self) -> bool {
        match self {
            Self::Boolean(b) => !**b,
            Self::Integer(i) => **i == 0,
            Self::Number(n) => **n == 0.0,
            Self::String(s) => match s.to_ascii_lowercase().trim() {
                super::string::FALSE => true,
                "0" => true,
                _ => s.trim().len() == 0
            },
            Self::Char(c) => match *c {
                '0' => true,
                _ => c.is_ascii_whitespace()
            },
            Self::Array(a) => a.len() == 0,
            Self::Object(o) => o.is_none(),
            Self::Undefined => true
        }
    }
}