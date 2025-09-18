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
