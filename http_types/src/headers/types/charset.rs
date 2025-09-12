/// Http Headers: Accept-Charset
/// 
/// RFC-2616 14.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.1
/// 
/// "Accept-Charset" ":" ( [charset | *] [";" "q" "=" qvalue]? ),
/// 
use http_macro::build_header_value;
use super::{HeaderName, QValue, HeaderType};

build_header_value!(
    pub struct Charset<'a> {
        pub charset: HeaderType<'a>,
        pub q: Option<QValue>
    },
    fn new() -> Self {
        Self {
            charset: HeaderType::WildCard,
            q: None
        }
    }
    HeaderName::AcceptCharset,
    fn from(value:&'a HeaderType<'a>) -> Self {
        parse(value.as_str())
    },
    fn parse(value:&'a HeaderType<'a>) -> Vec<Self> {
        value.as_str().split(",").map(parse).collect()
    },
    fn to_string(&self) -> String {
        let mut output = self.charset.as_str().to_owned();

        if self.q.is_some() {
            output.push_str(" ;q=");
            output.push_str(&self.q.as_ref().unwrap().to_string());
        }

        output
    }
);

fn parse<'a>(str:&'a str) -> Charset<'a>{
    let lines: Vec<_> = str.split(";").collect();

    let charset:HeaderType<'a> = match lines.get(0) {
        Some(str) => if *str == "*"  {
            HeaderType::WildCard
        } else {
            HeaderType::Text(*str)
        },
        None => HeaderType::WildCard
    };

    let q:Option<QValue> = match lines.get(1) {
        Some(str) => match str.find("=") {
            Some(index) => {
                let key = str[..index].trim();
                let value = str[index+1..].trim();

                if key == "q" {
                    match value.parse::<f32>() {
                        Ok(value) => Some(value),
                        Err(_) => None
                    }
                } else {
                    None
                }
            },
            None => None
        },
        None => None
    };

    Charset {
        charset,
        q
    }
}