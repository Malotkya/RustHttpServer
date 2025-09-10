/// Http Headers
/// 
/// RFC-2616 14
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14
/// 
pub use name::HeaderName;
pub use value::HeaderValue;
use std::collections::HashMap;
use std::collections::hash_map::Iter;

mod value;
mod name;
mod types;

pub struct Headers(HashMap<HeaderName, HeaderValue>);

#[allow(dead_code)]
impl Headers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set<V: Into<HeaderValue>>(&mut self,key:&str, value:V) -> &mut Self {
        self.0.insert(
            HeaderName::from(key),
            value.into()
        );
        self
    }

    pub fn insert<H: HttpHeader+Into<HeaderValue>>(&mut self, value:H) -> &mut Self {
        self.0.insert(
            H::name(),
            value.into()
        );
        self
    }

    pub fn get(&self, key:&str) -> Option<&HeaderValue> {
        self.0.get(
            &HeaderName::from(key)
        )
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a HeaderName, &'a HeaderValue);
    type IntoIter = Iter<'a, HeaderName, HeaderValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub trait HttpHeader {
    fn name() -> HeaderName;
}