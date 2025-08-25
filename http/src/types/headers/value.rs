use httpdate::{HttpDate};
use std::string::{ToString};

pub struct HeaderValue(Vec<u8>);

#[derive(Debug)]
pub enum HeaderError {
    InvalidUtf8,
    DateParseError(String),
    ValueParseError(String)
}


impl HeaderValue {
    pub fn ref_str<'a>(&'a self) -> Result<&'a str, HeaderError>{
        match str::from_utf8(&self.0) {
            Ok(value) => Ok(value),
            Err(_) => Err(HeaderError::InvalidUtf8)
        }
    }

    pub fn date(&self) -> Result<HttpDate, HeaderError> {
        let str = self.ref_str()?;
        match str.parse::<HttpDate>() {
            Ok(date) => Ok(date),
            Err(_) => Err(HeaderError::DateParseError(String::from(str)))
        }
    }

    pub fn value(&self) -> Result<String, HeaderError> {
        match str::from_utf8(&self.0) {
            Ok(value) => Ok(value.to_owned()),
            Err(_) => Err(HeaderError::InvalidUtf8)
        }
    }
}

impl<'a> Into<&'a str> for &'a HeaderValue {
    fn into(self) -> &'a str {
        self.ref_str().unwrap()
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(vec: &[u8]) -> Self {
        Self(vec.to_vec())
    }
}

impl From<&str> for HeaderValue {
    fn from(value:&str) -> Self {
        Self(
            value.as_bytes().to_vec()
        )
    }
}

impl From<&HttpDate> for HeaderValue {
    fn from(date: &HttpDate) -> Self {
        Self (
            date.to_string().as_bytes().into()
        )
    }
}

/*use crate::{Method, Url};
use crate::request::uri::Uri;
use super::types::*;



trait FromHeaderValue<'a, T>{
    fn to_str(&self) -> Result<&'a str, HeaderError>;
    fn to_date(&self) -> Result<DateTime<FixedOffset>, HeaderError>;
    fn to_vec(&self) -> Result<Vec<HeaderValue>, HeaderError>;
    fn value(&'a self) -> Result<T, HeaderError>;
}



macro_rules! build_header {
    ($name:ident) => {
        build_header!(
            $name,
            String,
            (this:&'a $name) -> Result<String, HeaderError> => {
                Ok(String::from(this.to_str()?))
            }
        );
    };
    (
        $name:ident,
        $type:ty,
        ($fun:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $name<'a>(&'a [u8]);

        impl<'a> FromHeaderValue<'a, $type> for $name<'a> {
            fn to_str(&self) -> Result<&'a str, HeaderError>{
                match str::from_utf8(self.0) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(HeaderError::InvalidUtf8)
                }
            }

            

            fn to_vec(&self) -> Result<Vec<HeaderValue<'a>>, HeaderError> {
                let str = self.to_str()?;
                Ok(str.split(",").map(|str|->HeaderValue<'a>{
                    HeaderValue(str.as_bytes())
                }).collect())
            }

            fn value(&'a self) -> Result<$type, HeaderError> {
                $fun!(self)
            }
        }
    }
}

build_header!(HeaderValue);

#[inline]
fn accept_value<'a>(this: &'a AcceptValue) -> Result<Vec<AcceptValueType<'a>>, HeaderError> {
    Ok(AcceptValueType::parse(this.to_str()?))
} build_header!(
    AcceptValue,
    Vec<AcceptValueType<'a>>,
    accept_value
);

#[inline]
fn charset_value<'a>(this: &'a AcceptCharsetValue ) -> Result<Vec<AcceptType<'a>>, HeaderError> {
    Ok(AcceptType::parse(this.to_str()?))
} build_header!(
    AcceptCharsetValue,
    Vec<AcceptType<'a>>,
    charset_value
);

#[inline]
fn encoding_value<'a>(this: &'a AcceptEncodingValue) -> Result<Vec<AcceptType<'a>>, HeaderError> {
    Ok(AcceptType::parse(this.to_str()?))
} build_header!(
    AcceptEncodingValue,
    Vec<AcceptType<'a>>,
    encoding_value
);

#[inline]
fn language_value<'a>(this: &'a AcceptLanguageValue) -> Result<Vec<AcceptType<'a>>, HeaderError> {
    Ok(AcceptType::parse(this.to_str()?))
} build_header!(
    AcceptLanguageValue,
    Vec<AcceptType<'a>>,
    language_value
);

#[inline]
fn ranges_value<'a>(this: &'a AcceptRangesValue) -> Result<Option<Vec<&'a str>>, HeaderError> {
    let str = this.to_str()?;
    if str.is_empty() || str.to_lowercase().trim() == "none" {
        return Ok(None);
    }
        
    Ok(Some(str.split(",").collect()))
} build_header!(
    AcceptRangesValue,
    Option<Vec<&'a str>>,
    ranges_value
);

#[inline]
fn age_value(this: &AgeValue) -> Result<usize, HeaderError> {
    let str = this.to_str()?;
    match str.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => Err(HeaderError::ValueParseError(
            format!("Unable to parse '{}' to usize!", str)
        ))
    }
} build_header!(
    AgeValue,
    usize,
    age_value
);

#[inline]
fn allow_value(this: &AllowValue) -> Result<Vec<Method>, HeaderError> {
    let str = this.to_str()?;
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
} build_header!(
    AllowValue,
    Vec<Method>,
    allow_value
);

#[inline]
fn authorization_value<'a>(this:&'a AuthorizationValue) -> Result<&'a str, HeaderError> {
        this.to_str() //TODO add authorization parsing?
} build_header!(
    AuthorizationValue,
    &'a str,
    authorization_value
);

#[inline]
fn cache_control_value<'a>(this: &'a CacheControlValue) -> Result<Vec<CacheControlType<'a>>, HeaderError> {
    let str = this.to_str()?;
    match CacheControlType::parse(str) {
        Ok(value) => Ok(value),
        Err(str) => Err(
            HeaderError::ValueParseError(str)
        )
    }
} build_header!(
    CacheControlValue,
    Vec<CacheControlType<'a>>,
    cache_control_value
);

build_header!(ConnectionValue);
build_header!(ContentEncodingValue);
build_header!(ContentLanguageValue);

#[inline]
fn uri_value(this: &UriValue) -> Result<Uri, HeaderError> {
    match Uri::parse(this.to_str()?) {
        Ok(value) => Ok(value),
        Err(e) => Err(
            HeaderError::ValueParseError(e)
        )
    }
} build_header!(
    UriValue,
    Uri,
    uri_value
);

#[inline]
fn md5_value<'a>(_: &'a ContentMD5Value) -> Result<&'a str, HeaderError> {
    todo!("Need to find base64 parser!")
} build_header!(
    ContentMD5Value,
    &'a str,
    md5_value
);

#[inline]
fn contnet_range_value<'a>(this: &'a ContentRangeHeader) -> Result<ContentRangeType<'a>, HeaderError> {
    match ContentRangeType::parse(this.to_str()?) {
        Ok(value) => Ok(value),
        Err(e) => Err(
            HeaderError::ValueParseError(e)
        )
    }
} build_header!(
    ContentRangeHeader,
    ContentRangeType<'a>,
    contnet_range_value
);

#[inline]
fn expect_value<'a>(this: &'a ExpectValue) -> Result<ExpectType<'a>, HeaderError> {
    Ok(ExpectType::parse(this.to_str()?))
} build_header!(
    ExpectValue,
    ExpectType<'a>,
    expect_value
);

#[inline]
fn location_value(this: &LocationValue) -> Result<Url, HeaderError> {
    match Url::parse(this.to_str()?) {
        Ok(value) => Ok(value),
        Err(e) => Err(
            HeaderError::ValueParseError(
                format!("{}", e)
            )
        )
    }
} build_header!(
    LocationValue,
    Url,
    location_value
);

#[inline]
fn wild_card_list_value<'a>(this: &'a WildCardListValue) -> Result<WildCardList<'a>, HeaderError> {
    Ok(WildCardList::parse(this.to_str()?))
} build_header!(
    WildCardListValue,
    WildCardList<'a>,
    wild_card_list_value
);

#[inline]
fn prigma_value<'a>(this: &'a PrigmaDirectiveValue) -> Result<PrigmaDirective<'a>, HeaderError> {
    Ok(PrigmaDirective::parse(this.to_str()?))
} build_header!(
    PrigmaDirectiveValue,
    PrigmaDirective<'a>,
    prigma_value
);

#[inline]
fn list_header_value<'a>(this: &'a ListHeaderValue) -> Result<Vec<&'a str>, HeaderError> {
    Ok(
        this.to_str()?.split(",")
        .map(|str|str.trim())
        .collect()
    )
} build_header!(
    ListHeaderValue,
    Vec<&'a str>,
    list_header_value
);*/