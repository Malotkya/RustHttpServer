use super::HeaderType;

pub struct MediaType<'a>(HeaderType<'a>, HeaderType<'a>);

impl<'a> MediaType<'a> {
    pub fn new() -> Self {
        Self(HeaderType::WildCard, HeaderType::WildCard)
    }

    pub fn from(str: &'a str) -> Self {
        if str.is_empty() {
            return Self::new();
        }

        match str.find("/") {
            Some(index) => {
                Self(
                    HeaderType::Text(&str[..index]),
                    HeaderType::Text(&str[index+1..])
                )
            },
            None => {
                Self(
                    HeaderType::Text(str),
                    HeaderType::WildCard
                )
            }
        }
    }
}

impl<'a> From<&'a super::HeaderType<'a>> for MediaType<'a> {
    fn from(value:&'a super::HeaderType<'a>) -> Self {
        match value {
            HeaderType::WildCard => Self::new(),
            super::HeaderType::Text(str) => Self::from(str)
        }
    }
}

impl<'a> ToString for MediaType<'a> {
    fn to_string(&self) -> String {
        vec![self.0.as_str(), "/", self.1.as_str()].join("")
    }
}