/// Http Headers: Accept
/// 
/// RFC-2616 14.1
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.1
/// 
/// "Accept" ":" ( [media-range] [accept-params]? ),
/// 
/// media-range = "*/*" | [type]/"*" | [type]/[subtype]
/// accept-params = ";" "q" "=" [qvalue] [accept-extension]?
/// accept-extension = ";" [token] ["=" [token \ quoted-string]]?
/// 
use std::collections::{HashMap, VecDeque};
use http_macro::build_header_value;
use super::{MediaType, QValue, HeaderType, HeaderName};

build_header_value!(
    pub struct AcceptValueType<'a> {
        pub range: MediaType<'a>,
        pub q: Option<QValue>,
        pub extensions: HashMap<&'a str, &'a str>
    };
    fn new() -> Self {
        Self {
            range: MediaType::new(),
            q: None,
            extensions: HashMap::new()
        }
    };
    HeaderName::Accept;
    fn from(value: &'a HeaderType<'a>) -> Self {
        match value {
            HeaderType::WildCard => Self::new(),
            HeaderType::Text(str) => parse(str)
        }
    };
    fn to_string(&self) -> String {
        let mut output = self.range.to_string();

        if self.q.is_some() {
            output.push_str("; q=");
            output.push_str(&self.q.as_ref().unwrap().to_string());
        }

        for (key, value) in self.extensions.iter()  {
            output.push_str("; ");
            output.push_str(key);
            if value.len() > 0 {
                output.push_str("=");
                output.push_str(value);
            }
        }

        output
    };
    fn parse(value: &'a HeaderType<'a>) -> Vec<Self> {
        value.as_str().split(",").map(parse).collect()
    };
);

fn parse<'a>(str:&'a str) -> AcceptValueType<'a> {
    let mut lines:VecDeque<_> = str.split(";").collect();

    let range:MediaType<'a> = match lines.pop_front() {
        Some(str) => MediaType::from(str),
        None => MediaType::new()
    };

    let mut q:Option<QValue> = None;
    let mut extensions:HashMap<&'a str, &'a str> = HashMap::new();

    while let Some(str) = lines.pop_front() {
        match str.find("=") {
            Some(index) => {
                let key = str[..index].trim();
                let value = str[index+1..].trim();

                if key == "q" {
                    q = match value.parse::<f32>() {
                        Ok(value) => Some(value),
                        Err(_) => None
                    }
                } else {
                    extensions.insert(key, value);
                }
            },
            None => { 
                extensions.insert(str, "");
            }
        }
    }

    AcceptValueType {
        range, q, extensions
    }
}
