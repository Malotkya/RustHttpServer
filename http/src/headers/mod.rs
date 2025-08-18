/// Http Headers
/// 
/// RFC-2616 14
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14
/// 
pub use name::HeaderName;
pub use value::HeaderValue;
use std::collections::HashMap;

mod value;
mod name;
//mod types;

pub struct Headers(HashMap<HeaderName, HeaderValue>);

#[allow(dead_code)]
impl Headers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self,key:&str, value:&str) {
        self.0.insert(
            HeaderName::from(key),
            HeaderValue::from(value)
        );
    }

    pub fn get(&self, key:&str) -> Option<&HeaderValue> {
        self.0.get(
            &HeaderName::from(key)
        )
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}