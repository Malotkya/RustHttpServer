/// Http Headers: Accept-Ranges
/// 
/// RFC-2616 14.5
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.5
/// 
/// ""Accept-Ranges" ":" acceptable-ranges
///
/// acceptable-ranges = 1#range-unit | "none"
/// 
use std::ops::{Deref, DerefMut};
use http_macro::build_header_value;
use super::{HeaderType, HeaderName};

build_header_value!(
    pub struct RangeValue(Option<usize>);
    fn new() -> Self {
        Self(None)
    };
    HeaderName::AcceptRanges;
    fn from(value: & HeaderType<'_>) -> Self {
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
            None => String::from("none"),
            Some(value) => value.to_string()
        }
    }
);

impl Deref for RangeValue {
    type Target = Option<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RangeValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for RangeValue {
    fn from(value: usize) -> Self {
        Self(Some(value))
    }
}
