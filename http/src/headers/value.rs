use crate::{Method, Url};
use crate::request::uri::Uri;
use super::types::*;

use chrono::{DateTime, FixedOffset};

macro_rules! build_header {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $name<'a>(&'a [u8]);

        impl<'a> FromHeaderValue<'a> for $name<'a> {
            fn to_str(&self) -> Result<&'a str, HeaderError>{
                match str::from_utf8(self.0) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(HeaderError::InvalidUtf8)
                }
            }

            fn to_date(&self) -> Result<DateTime<FixedOffset>, HeaderError> {
                let str = self.to_str()?;
                match DateTime::parse_from_rfc2822(str) {
                    Ok(date) => Ok(date),
                    Err(_) => Err(HeaderError::DateParseError(String::from(str)))
                }
            }

            fn to_vec(&self) -> Result<Vec<HeaderValue<'a>>, HeaderError> {
                let str = self.to_str()?;
                Ok(str.split(",").map(|str|->HeaderValue<'a>{
                    HeaderValue(str.as_bytes())
                }).collect())
            }
        }
    }
}

enum HeaderError {
    InvalidUtf8,
    DateParseError(String),
    ValueParseError(String)
}

trait FromHeaderValue<'a> {
    fn to_str(&self) -> Result<&'a str, HeaderError>;
    fn to_date(&self) -> Result<DateTime<FixedOffset>, HeaderError>;
    fn to_vec(&self) -> Result<Vec<HeaderValue>, HeaderError>;
}

build_header!(HeaderValue);

build_header!(AcceptValue);

impl<'a> AcceptValue<'a> {
    fn value(&self) -> Result<Vec<AcceptValueType<'a>>, HeaderError> {
        let str = self.to_str()?;
        Ok(AcceptValueType::parse(str))
    }
}

build_header!(AcceptCharsetValue);

impl<'a> AcceptCharsetValue<'a> {
    fn value(&self) -> Result<Vec<AcceptType>, HeaderError> {
        let str = self.to_str()?;
        Ok(AcceptType::parse(str))
    }
}

build_header!(AcceptEncodingValue);

impl<'a> AcceptEncodingValue<'a> {
    fn value(&self) -> Result<Vec<AcceptType>, HeaderError> {
        let str = self.to_str()?;
        Ok(AcceptType::parse(str))
    }
}

build_header!(AcceptLanguageValue);

impl<'a> AcceptLanguageValue<'a> {
    fn value(&self) -> Result<Vec<AcceptType>, HeaderError> {
        let str = self.to_str()?;
        Ok(AcceptType::parse(str))
    }
}

build_header!(AcceptRangesValue);

impl<'a> AcceptRangesValue<'a> {
    fn value(&self) -> Result<Option<Vec<&'a str>>, HeaderError> {
        let str = self.to_str()?;
        if str.is_empty() || str.to_lowercase().trim() == "none" {
            return Ok(None);
        }
        
        Ok(Some(str.split(",").collect()))
    }
}

build_header!(AgeValue);

impl<'a> AgeValue<'a> {
    fn value(&self) -> Result<usize, HeaderError> {
        let str = self.to_str()?;
        match str.parse::<usize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(HeaderError::ValueParseError(
                format!("Unable to parse '{}' to usize!", str)
            ))
        }
    }
}

build_header!(AllowValue);

impl<'a> AllowValue<'a> {
    fn value(&self) -> Result<Vec<Method>, HeaderError> {
        let str = self.to_str()?;
        let mut vec:Vec<Method> = Vec::new();

        for str in str.split(",") {
            match Method::from(str.trim()) {
                Some(value) => vec.push(value),
                None => {
                    return Err(HeaderError::ValueParseError(
                        format!("Unable to parse '{}' into a method!", str)
                    ));
                }
            }
        }

        Ok(vec)
    }
}

build_header!(AuthorizationValue);

impl<'a> AuthorizationValue<'a> {
    fn value(&self) -> Result<&'a str, HeaderError> {
        self.to_str() //TODO add authorization parsing?
    }
}

build_header!(CacheControlValue);

impl<'a> CacheControlValue<'a> {
    fn value(&self) -> Result<Vec<CacheControlType>, HeaderError> {
        let str = self.to_str()?;
        match CacheControlType::parse(str) {
            Ok(value) => Ok(value),
            Err(str) => Err(
                HeaderError::ValueParseError(str)
            )
        }
    }
}

pub type ConnectionValue<'a> = HeaderValue<'a>;

pub type ContentEncodingValue<'a> = HeaderValue<'a>;

pub type ContentLanguageValue<'a> = HeaderValue<'a>;

build_header!(UriValue);

impl<'a> UriValue<'a> {
    fn value(&self) -> Result<Uri, HeaderError> {
        match Uri::parse(self.to_str()?) {
            Ok(value) => Ok(value),
            Err(e) => Err(
                HeaderError::ValueParseError(e)
            )
        }
    }
}

build_header!(ContentMD5Value);

impl<'a> ContentMD5Value<'a> {
    fn value(&self) -> Result<&'a str, HeaderError> {
        todo!("Need to find base64 parser!")
    }
}

build_header!(ContentRangeHeader);

impl<'a> ContentRangeHeader<'a> {
    fn value(&self) -> Result<ContentRangeType, HeaderError> {
        match ContentRangeType::parse(self.to_str()?) {
            Ok(value) => Ok(value),
            Err(e) => Err(
                HeaderError::ValueParseError(e)
            )
        }
    }
}

build_header!(ExpectValue);

impl<'a> ExpectValue<'a> {
    fn value(&self) -> Result<ExpectType, HeaderError> {
        Ok(ExpectType::parse(self.to_str()?))
    }
}

build_header!(LocationValue);

impl<'a> LocationValue<'a> {
    fn value(&self) -> Result<Url, HeaderError> {
        match Url::parse(self.to_str()?) {
            Ok(value) => Ok(value),
            Err(e) => Err(
                HeaderError::ValueParseError(
                    format!("{}", e)
                )
            )
        }
    }
}

build_header!(IfMatchValue);

impl<'a> IfMatchValue<'a> {
    fn value(&self) -> Result<WildCardList, HeaderError> {
        Ok(WildCardList::parse(self.to_str()?))
    }
}

build_header!(PrigmaDirectiveValue);

impl<'a> PrigmaDirectiveValue<'a> {
    fn value(&self) -> Result<PrigmaDirective, HeaderError> {
        Ok(PrigmaDirective::parse(self.to_str()?))
    }
}

build_header!(ListHeaderValue);

impl<'a> ListHeaderValue<'a> {
    fn value(&self) -> Result<Vec<&'a str>, HeaderError> {
        Ok(
            self.to_str()?.split(",")
            .map(|str|str.trim())
            .collect()
        )
    }
}