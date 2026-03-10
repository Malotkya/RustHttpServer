/// Http Headers: Age
/// 
/// RFC-2616 14.6
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.6
/// 
/// "Age" ":" age-value
///
/// age-value = delta-seconds
/// 
use std::ops::{Deref, DerefMut};
use http_macro::build_header_value;
use super::{HeaderType, HeaderName};


build_header_value!(
    pub struct AgeValue(Option<usize>);
    fn new() -> Self {
        Self(None)
    };
    HeaderName::Age;
    fn from(value: &'a HeaderType<'a>) -> Self {
        match value {
            HeaderType::WildCard => Self::new(),
            HeaderType::Text(str) => match str.parse::<usize>() {
                Ok(value) => value.into(),
                Err(_) => Self::new()
            }
        }
    };
    fn to_string(&self) -> String {
        match self.0 {
            None => String::new(),
            Some(value) => value.to_string()
        }
    }
);


impl Deref for AgeValue {
    type Target = Option<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AgeValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for AgeValue {
    fn from(value: usize) -> Self {
        Self(Some(value))
    }
}

