
#[derive(PartialEq)]
pub enum AttributeName {
    Static(&'static str),
    Alloc(String)
}

impl AttributeName {
    pub fn value(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Alloc(s) => s
        }
    }
}

impl Clone for AttributeName {
    fn clone(&self) -> Self {
        match self {
            Self::Static(s) => Self::Static(*s),
            Self::Alloc(s) => Self::Alloc(s.clone())
        }
    }
}