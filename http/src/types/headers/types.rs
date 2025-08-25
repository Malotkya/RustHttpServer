use std::collections::{HashMap, VecDeque};

pub type QValue = f32;

const EMPTY_STRING:&'static str = "";

pub enum Type<'a> {
    WildCard,
    Text(&'a str),
}

impl<'a> Type<'a> {
    pub fn value(&self) -> &'a str {
        match self {
            Self::Text(v) => *v,
            Self::WildCard => "*" 
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Text(v) => String::from(*v),
            Self::WildCard => String::from("*")
        }
    }
}

pub struct MediaType<'a>(Type<'a>, Type<'a>);

impl<'a> MediaType<'a> {
    pub fn new() -> Self {
        Self(Type::WildCard, Type::WildCard)
    }

    pub fn from(str: &'a str) -> Self {
        if str.is_empty() {
            return Self::new();
        }

        match str.find("/") {
            Some(index) => {
                Self(
                    Type::Text(&str[..index]),
                    Type::Text(&str[index+1..])
                )
            },
            None => {
                Self(
                    Type::Text(str),
                    Type::WildCard
                )
            }
        }
    }
}

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
pub struct AcceptValueType<'a> {
    pub range: MediaType<'a>,
    pub q: Option<QValue>,
    pub extensions: HashMap<&'a str, &'a str>
}

impl<'a> AcceptValueType<'a> {
    pub fn parse(value:&'a str) -> Vec<Self> {
        value.split(",").map(|str | -> Self {
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

            Self {
                range, q, extensions
            }
        }).collect()
    }
}

/// Http Headers: Accept-Charset
/// 
/// RFC-2616 14.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-14.1
/// 
/// "Accept-Charset" ":" ( [charset | *] [";" "q" "=" qvalue]? ),
/// 
pub struct AcceptType<'a>{
    pub charset: Type<'a>,
    pub q: Option<QValue>
}

impl<'a> AcceptType<'a> {
    pub fn parse(value:&'a str) -> Vec<Self> {
        value.split(",").map(|str|->AcceptType<'a> {
            let lines: Vec<_> = str.split(";").collect();

            let charset:Type<'a> = match lines.get(0) {
                Some(str) => {
                    if *str == "*"  {
                        Type::WildCard
                    } else {
                        Type::Text(*str)
                    }
                },
                None => Type::WildCard
            };

            let q:Option<QValue> = match lines.get(1) {
                Some(str) => {
                    match str.find("=") {
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
                    }
                },
                None => None
            };

            Self {
                charset,
                q
            }
        }).collect()
    }
}

pub struct AuthorizationType<'a>(&'a str, &'a str);

pub enum CacheControlType<'a> {
    NoCache(Option<&'a str>),
    NoStore,
    MaxAge(usize),
    MaxStale(Option<usize>),
    MinFresh(usize),
    NoTransform,
    OnlyIfCached,
    Public,
    Private(Option<&'a str>),
    MustRevalidate,
    ProxyRevalidate,
    SMaxAge(usize),
    CacheExtension(&'a str, &'a str),
}

impl<'a> CacheControlType<'a> {
    pub fn parse(value:&'a str) -> Result<Vec<Self>, String> {
        let mut vec: Vec<Self> = Vec::new();

        for str in value.split(",") {
            vec.push(Self::from(str)?);
        }

        Ok(vec)
    }

    pub fn from(value: &'a str) -> Result<Self, String>{
        let split: Vec<_> = value.split("=").collect();

        let key:&'a str = split.get(0).unwrap().trim();
        let value = ||{
            match split.get(1) {
                Some(value) => Some(value.trim()),
                None => None
            }
        };

        match key.to_lowercase().as_str() {
            "no-cache" => Ok(
                CacheControlType::NoCache(value())
            ),
            "no-store" => Ok(
                CacheControlType::NoStore
            ),
            "max-age" => match value() {
                Some(value) => {
                    match value.parse::<usize>() {
                        Ok(value) => Ok(
                            CacheControlType::MaxAge(value)
                        ),
                        Err(e) => Err(
                            format!("{}", e)
                        )
                    }
                },
                None => Err(
                    String::from("Missing max-age value!")
                )
            },
            "max-stale" => match value(){
                Some(value) => match value.parse::<usize>() {
                    Ok(value) => Ok(
                        CacheControlType::MaxStale(
                            Some(value)
                        )
                    ),
                    Err(e) => Err(
                        format!("{}", e)
                    )
                },
                None => Ok(
                    CacheControlType::MaxStale(None)
                )
            },
            "min-fresh" => match value() {
                Some(value) => {
                    match value.parse::<usize>() {
                        Ok(value) => Ok(
                            CacheControlType::MinFresh(value)
                        ),
                        Err(e) => Err(
                            format!("{}", e)
                        )
                    }
                },
                None => Err(
                    String::from("Missing min-fresh value!")
                )
            },
            "no-transform" => Ok(
                CacheControlType::NoTransform
            ),
            "only-if-cached" => Ok(
                CacheControlType::OnlyIfCached
            ),
            "public" => Ok(
                CacheControlType::Public
            ),
            "private" => match value() {
                Some(value) => Ok(
                    CacheControlType::Private(
                        Some(value)
                    )
                ),
                None => Ok(
                    CacheControlType::Private(
                        None
                    )
                ),
            },
            "must-revalidate" => Ok(
                CacheControlType::MustRevalidate
            ),
            "proxy-revalidate" => Ok(
                CacheControlType::ProxyRevalidate
            ),
            "s-maxage" => match value() {
                Some(value) => {
                    match value.parse::<usize>() {
                        Ok(value) => Ok(
                            CacheControlType::SMaxAge(value)
                        ),
                        Err(e) => Err(
                            format!("{}", e)
                        )
                    }
                },
                None => Err(
                    String::from("Missing s-maxage value!")
                )
            },
            _ => {
                Ok(
                    CacheControlType::CacheExtension(
                        key,
                        value().unwrap_or("")
                    )
                )
            }
        } 
    }
}

pub struct ContentRangeType<'a> {
    unit: &'a str,
    first_byte: Option<usize>,
    last_byte: Option<usize>,
    length: Option<usize>
}

impl<'a> ContentRangeType<'a> {
    pub fn parse(string:&'a str) -> Result<Self, String> {
        let mut buffer = string.clone();

        let unit:&'a str;
        match buffer.find(" ") {
            Some(index) => {
                unit = &buffer[..index];
                buffer = &buffer[index..]
            },
            None => {
                unit = buffer;
                buffer = "";
            }
        }

        let length = match buffer.find("/") {
            Some(index) => {
                let value = &buffer[index+1..];
                match value.parse::<usize>() {
                    Ok(value) => {
                        buffer = &buffer[..index];
                        Some(value)
                    },
                    Err(e) => {
                        if value.trim() == "*" {
                            None
                        } else {
                            return Err(
                                format!("{}", e)
                            )
                        }
                    }
                }
            },
            None => {
                None
            }
        };

        let first_byte: Option<usize>;
        let last_byte: Option<usize>;
        match buffer.find("-") {
            Some(index) => {
                let value = buffer[..index].trim();
                first_byte = match value.parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(e) => {
                        return Err(
                            format!("{}", e)
                        )
                    }
                };

                let value = buffer[index+1..].trim();
                last_byte = match value.parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(e) => {
                        return Err(
                            format!("{}", e)
                        )
                    }
                }
            },
            None => {
                if buffer.trim() == "*" {
                    first_byte = None;
                    last_byte = None;
                } else {
                    return Err(
                        String::from("Invalid byte-range-spec!")
                    )
                }
            }
        }

        Ok(
            Self {
                unit,
                first_byte,
                last_byte,
                length
            }
        )
    }
}

pub enum ExpectType<'a> {
    Contune,
    Extension(&'a str, ExpectParamType<'a>)
}

pub enum ExpectParamType<'a> {
    Token(&'a str),
    Params(HashMap<&'a str, &'a str>),
    None
}

impl<'a> ExpectType<'a> {
    pub fn parse(string:&'a str) -> Self {
        let mut token = string.trim();
        if token.to_ascii_uppercase().trim() == "100-continue" {
            return ExpectType::Contune
        }
        let params:ExpectParamType<'a>;
        
        match token.find("=") {
            Some(index) => {
                let value = &token[index+1..];
                token = &token[..index];

                match value.find(";") {
                    None => {
                        params = ExpectParamType::Token(value);
                    },
                    Some(_) => {
                        let mut map = HashMap::new();

                        for str in value.split(";") {
                            let lines: Vec<_> = str.split("=").collect();

                            let key = lines.get(0).unwrap().trim();
                            let value = lines.get(1).unwrap_or(&"").trim();

                            map.insert(key, value);
                        }

                        params = ExpectParamType::Params(map);
                    }
                }
            },
            None => {
                params = ExpectParamType::None;
            }
        }

        Self::Extension(token, params)
    }
}

pub enum WildCardList<'a> {
    WildCard,
    List(Vec<&'a str>)
}

impl<'a> WildCardList<'a> {
    pub fn parse(string: &'a str) -> Self {
        let buffer = string.trim();

        if buffer.is_empty() || buffer == "*" {
            return Self::WildCard;
        }

        let mut list:Vec<&'a str> = Vec::new();

        for mut str in buffer.split(",") {
            if &str[..1] == "\"" {
                str = &str[1..];
            }
            
            let index = str.len()-1;
            if &str[index..] == "\"" {
                str = &str[..=index];
            }

            list.push(str);
        }

        Self::List(list)
    }
}

pub enum PrigmaDirective<'a> {
    NoCache,
    Extension(&'a str, Option<&'a str>)
}

impl<'a> PrigmaDirective<'a> {
    pub fn parse(value:&'a str) -> Self {
        match value.to_lowercase().as_str() {
            "no-cache" => Self::NoCache,
            _ => {
                let list:Vec<_> = value.split("=").collect();

                let key = list.get(0).unwrap().trim();
                let value = match list.get(1) {
                    Some(value) => Some(value.trim()),
                    None => None
                };

                Self::Extension(key, value)
            }
        }
    }
}