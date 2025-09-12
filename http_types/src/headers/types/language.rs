use std::collections::VecDeque;

/// Http Headers: Accept-Language
/// 
/// RFC-2616 14.4
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.4
/// 
/// "Accept-Language" ":" ( language-range [ ";" "q" "=" qvalue ] ), +
/// 
/// media-range = language-range  = ( ( 1*8ALPHA *( "-" 1*8ALPHA ) ) | "*" )
/// 
use http_macro::build_header_value;
use super::{MediaType, QValue, HeaderType, HeaderName};

build_header_value!(
    struct LanguageValue<'a> {
        start: MediaType<'a>,
        end: Option<MediaType<'a>>,
        q: Option<QValue>
    };
    fn new() -> Self {
        Self {
            start: MediaType::new(),
            end: None,
            q: None
        }
    };
    HeaderName::AcceptLanguage;
    fn from(value: &'a HeaderType<'a>) -> Self {
        match value {
            HeaderType::WildCard => Self::new(),
            HeaderType::Text(str) => parse(str)
        }
    };
    fn to_string(&self) -> String {
        let mut output = self.start.to_string();

        if self.end.is_some() {
            output.push('-');
            output.push_str(&self.end.as_ref().unwrap().to_string());
        }

        if self.q.is_some() {
            output.push_str("; q=");
            output.push_str(&self.q.unwrap().to_string())
        }

        output
    };
    fn parse(value: &'a HeaderType<'a>) -> Vec<Self> {
        value.as_str().split(",").map(parse).collect()
    }
);

fn parse<'a>(str: &'a str) -> LanguageValue<'a> {
    let mut lines:VecDeque<_> = str.split(";").collect();

    let start:MediaType;
    let end: Option<MediaType>;
    if let Some(line) = lines.pop_front() {
        if let Some(index) = line.find("-") {
            let (one, two) = line.split_at(index);

            start = MediaType::from(one);
            end = Some(MediaType::from(two)); //May need to slice
        } else {
            start = MediaType::from(line);
            end = None;
        }
    } else {
        start = MediaType::new();
        end = None;
    }

    let q = if let Some(line) = lines.pop_front() {
        if let Some(index) = line.find("=") {
            let (key, value) = line.split_at(index);

            if key == "q" {
                match value.parse::<f32>() {
                    Ok(value) => Some(value),
                    Err(_) => None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    LanguageValue {
        start, end, q
    }
}