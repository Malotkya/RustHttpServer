use std::fmt;

///////////////////////////////////////////////////////////////
/// Http Url
/// RFC-2616 5.1.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.2
/// RFC-2396
/// https://datatracker.ietf.org/doc/html/rfc2396
///////////////////////////////////////////////////////////////
/// Request-URI  = "*" | absoluteURI | abs_path | authority
///////////////////////////////////////////////////////////////
/// absoluteURI = scheme ":" (hier_part | opaque_part)
/// relativeURI = ( net_path | abs_path | rel_path ) [? query]
/// 
/// hier_part   = ( net_path | abs_path ) ["?" query ]
/// opaque_part = uric_no_slash *uric
/// 
/// uric_no_slash = unreserved | escaped | ";" | "?" | ":" | "@" |
///                 "&" | "+" | "+" | "$" | ","
/// 
/// net_path = "//" authority [abs_path]
/// abs_path = "/" path_segments
/// rel_path = resl_segment [abs_path]
/// 
/// rel_segment = 1*( unreserved | escaped |
///                 ";" | "@" | "&" | "=" | "+" | "$" | "," )
/// 
/// scheme    = alpha *( alpha | digit | "+" | "-" | "." )
/// authority = server | reg_name
/// reg_name  = 1*( unreserved | escaped | "$" | "," |
///               ";" | ":" | "@" | "&" | "=" | "+" )
/// 
/// server   = [[userinfo "@"] hostport]
/// userinfo = *( unreserved | escaped |
///             ";" | ":" | "&" | "=" | "+" | "$" | "," )
/// 
/// hostport    = host [: port ]
/// host        = hostname | IPv4address
/// hostname    = *( domainlabel "." ) toplabel [ "." ]
/// domainlabel = alphanum | alphanum *( alphanum | "-" ) alphanum
/// toplabel    = lpha | alpha *( alphanum | "-" ) alphanum
/// IPv4address = 1*digit "." 1*digit "." 1*digit "." 1*digit
/// port        = *digit
/// 
/// path          = [ abs_path | opaque_part ]
/// path_segments = segment *( "/" segment )
/// segment       = *pchar *(";" *pchar )
/// pchar         = unreserved | escaped |
///                   ":" | "@" | "&" | "=" | "+" | "$" | ","
/// query         = *uric
/// fragment      = *uric
/// uric          = reserved | unreserved | escaped
use super::{Text, Tokens, Seperator, Tokenizer, TokenError, TokenIterator};
use http::types::{Hostname, Url, ToUrl};

/*lazy_static::lazy_static!{
    static ref ABS_URI_REF:Text = Text::from_str();
    static ref PATH_REF:Text = Text::from_str();
    static ref SEARCH_REF:AsciiChar = AsciiChar::new('%');
    static ref HASH_REF:AsciiChar = AsciiChar::new('#');
}*/

pub struct Authority {
    host: Text,
    port: Option<u16>,
    user: Option<Text>
}

pub enum UriError {
    ParseToken(TokenError),
    InvalidPort(Text),
    SyntaxError(&'static str)
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseToken(e) => e.fmt(f),
            Self::SyntaxError(s) => write!(f, "{}", s),
            Self::InvalidPort(txt) => write!(f, "Invalid Port Number: {}!", txt.as_str())
        }
    }
}

impl Authority {
    pub fn parse(value: &Text) -> Result<Authority, UriError> {
        Self::parse_vec(value.tokenize().collect())
    }

    fn parse_vec<'a>(value:Vec<Tokens>) -> Result<Authority, UriError> {
        match value.len() {
            //host
            1 => {
                Ok(
                    Authority {
                        host: value.get(0)
                                   .unwrap().text(Some("host"))
                                   .map_err(|e|UriError::ParseToken(e))?,
                        port: None,
                        user: None
                    }
                )
            },
            //host:port or user@host
            3 => {  
                let user:Option<Text>;
                let host:Text;
                let port:Option<Text>;
                match value.get(1).unwrap() {
                    Tokens::Seperator(sep) => match sep {
                        Seperator::At => {
                            user = Some(value.get(0)
                                        .unwrap().text(Some("user"))
                                        .map_err(|e|UriError::ParseToken(e))?);
                            host = value.get(2)
                                        .unwrap().text(Some("host"))
                                        .map_err(|e|UriError::ParseToken(e))?;
                            port = None;
                        },
                        Seperator::Colon => {
                            user = None;
                            host = value.get(0)
                                        .unwrap().text(Some("host"))
                                        .map_err(|e|UriError::ParseToken(e))?;
                            port = Some(value.get(2)
                                        .unwrap().text(Some("port"))
                                        .map_err(|e|UriError::ParseToken(e))?);
                        },
                        _ => {
                            return Err(UriError::SyntaxError("Invalid Authority Syntax!"));
                        }
                    },
                    Tokens::Text(txt) => return Err(UriError::ParseToken(
                        TokenError::SeperatorError(None, txt.clone())
                    ))                
                }
                
                Ok(Authority{
                    host, user,
                    port: match port {
                        Some(t) => match t.as_str().parse() {
                            Ok(number) => Some(number),
                            Err(_) => return Err(
                                UriError::InvalidPort(t)
                            )
                        }
                        None => None
                    }
                })
            },
            //user@host:port
            5 => {
                let user = value.get(0)
                    .unwrap().text(Some("user"))
                    .map_err(|e|UriError::ParseToken(e))?;

                value.get(1).unwrap()
                    .seperator(Some(Seperator::At))
                    .map_err(|e|UriError::ParseToken(e))?;

                let host = value.get(2)
                    .unwrap().text(Some("host"))
                    .map_err(|e|UriError::ParseToken(e))?;

                value.get(3).unwrap()
                     .seperator(Some(Seperator::Colon))
                     .map_err(|e|UriError::ParseToken(e))?;

                
                let port = value.get(4)
                    .unwrap().text(Some("port"))
                    .map_err(|e|UriError::ParseToken(e))?;

                Ok(
                    Authority{ 
                        user: Some(user),
                        host: host,
                        port: match port.as_str().parse(){
                            Ok(number) => Some(number),
                            Err(_) =>return Err(UriError::InvalidPort(port))
                        }
                    }
                )
            },
            _ => {
                Err(UriError::SyntaxError("Invalid Authority Layout!"))
            }
        }
    }

    fn hostname(&self) -> Hostname {
        self.host.to_string().into()
    }

    fn auth(&self) -> Option<(String, String)> {
        match &self.user {
            Some(value) =>{
                let str = value.to_string();
                match str.find(":") {
                    Some(index) => Some((
                        String::from(&str[..index]),
                        String::from(&str[index+1..])
                    )),
                    None => None
                }
            },
            None => None
        }
    }

    fn to_url_path(&self, hostname:Hostname, port:u16, path:Vec<String>) -> Url {
        let hostname = match self.hostname() {
            Hostname::None => hostname,
            other => other
        };
        let port = self.port.unwrap_or(port);
        let mut url = Url::new(hostname, port, path);

        if let Some((username, password)) = self.auth() {
            url.set_auth(username, password);
        }

        url
    }
}

impl ToUrl for Authority {
    type Error = String;
    fn to_url(&self, default_hostname:Hostname, default_port:u16) -> Result<Url, Self::Error> {
        Ok(self.to_url_path(default_hostname, default_port, Vec::new()))
    }
}

impl ToString for Authority {
    fn to_string(&self) -> String {
        let mut str = String::new();

        if self.user.is_some() {
            str.push_str(self.user.as_ref().unwrap().as_str());
            str.push('@');
        }

        str.push_str(self.host.as_str());

        if self.port.is_some() {
            str.push(':');
            str.push_str(&self.port.as_ref().unwrap().to_string());
        }

        str
    }
}

pub struct AbsPath(Vec<Text>);

impl AbsPath {
    fn parse(value: &Text) -> Result<Self, TokenError> {
        // "/"?, [path_section]?, ("/", [path_section]?,)*
        let mut it = value.tokenize();
        let mut start:Option<Text> = None;

        //Mace sure to start loop without a seperator at the front.
        if let Some(token) = it.next() {
            if token.is_text() {
                start = Some(token.text(None).unwrap());

                //Handle Next Seperator
                if let Some(sep) = it.next() {
                    sep.seperator(Some(Seperator::ForwardSlash))?;
                }
            }
        }

        let (result, err_sep) = Self::parse_iter(&mut it, start)?;
        if err_sep.is_some() {
            Err(TokenError::MismatchError(Seperator::ForwardSlash, err_sep.unwrap()))
        } else {
            Ok(result)
        }
    }

    fn parse_iter<'a>(value:&mut TokenIterator<'a, Text>, start:Option<Text>) -> Result<(Self, Option<Seperator>), TokenError> {
        let mut vec: Vec<Text> = Vec::new();

        if start.is_some() {
            vec.push(start.unwrap());
        }

        while let Some(segment) = value.next() {
            vec.push(segment.text(None)?);

            // ([path_section]?, "/"?)*
            if let Some(segment) = value.next() {
                let seperator = segment.seperator(None)?;
                if seperator != Seperator::ForwardSlash {
                    return Ok((Self(vec), Some(seperator)))
                }
            }
        }

        Ok((Self(vec), None))
    }

    fn collect(&self) -> Vec<String> {
        self.0.iter().map(|t|t.to_string()).collect()
    }
}

impl ToUrl for AbsPath {
    type Error = String;
    fn to_url(&self, default_hostname:Hostname, default_port:u16) -> Result<Url, Self::Error> {
        Ok(
            Url::new(
                default_hostname, 
                default_port,
                self.collect()
            )
        )
    }
}

impl ToString for AbsPath {
    fn to_string(&self) -> String {
        String::from("/") + &self.collect().join("/")
    }
}

struct AbsUri {
    scheme: Text,
    authority: Option<Authority>,
    path: AbsPath,
    query: Vec<Text>
}

impl AbsUri {
    fn parse(value: &Text) -> Result<Self, UriError> {
        let mut it = value.tokenize();

        // [scheme] ":", "/" , ([authority],)? "/" ([abs_path],)? ("?", [query])
        let scheme: Text = match it.next() {
            Some(t) => t.text(Some("scheme"))
                                .map_err(|e|UriError::ParseToken(e))?,
            None => return Err(UriError::SyntaxError("Unexpected end when parsing scheme!"))
        };

        //":", "/" , ([authority],)? "/" ([abs_path],)? ("?", [query])
        match it.next() {
            Some(t) => t.seperator(Some(Seperator::Colon))
                                .map_err(|e|UriError::ParseToken(e))?,
            None => return Err(UriError::SyntaxError("Unexpected end when parsing scheme!"))
        };

        //"/" , ([authority],)? "/" ([abs_path],)? ("?", [query])
        match it.next() {
            Some(t) => t.seperator(Some(Seperator::ForwardSlash))
                                .map_err(|e|UriError::ParseToken(e))?,
            None => return Err(UriError::SyntaxError("Unexpected end when parsing authority or path!"))
        };


        // ([authority],)? "/" ([abs_path],)? ("?", [query])
        let mut next = match it.next() {
            Some(tok) => tok,
            None => return Err(UriError::SyntaxError("Unexpected end when parsing authority or path!"))
        };

        //if authority
        let authority = if next.is_text() {
            let mut vec:Vec<Tokens> = Vec::with_capacity(5);
            vec.push(next.clone());
            
            for _ in 0..5 {
                next = match it.next() {
                    Some(tok) => tok,
                    None => return Err(UriError::SyntaxError("Unexpected end when parsing Authority!"))
                };

                match next {
                    Tokens::Seperator(sep) => {
                        if sep == Seperator::Colon ||  sep == Seperator::At {
                            vec.push(next.clone())
                        } else {
                            break
                        }
                    },
                    Tokens::Text(_) => vec.push(next.clone())
                }
            }
            
            Some(Authority::parse_vec(vec)?)
        } else {
            None
        };

        //"/" ([abs_path],)? ("?", [query],)
        next.seperator(Some(Seperator::ForwardSlash))
            .map_err(|e|UriError::ParseToken(e))?;
        
        let (path, next) = AbsPath::parse_iter(&mut it, None)
                .map_err(|e|UriError::ParseToken(e))?;

        let mut query = Vec::new();
        //("?", [query],)
        if let Some(token) = next && token == Seperator::QuestionMark {
            while let Some(token) = it.next() {
                match token {
                    Tokens::Seperator(sep) => {
                        if sep != Seperator::Equals {
                            return Err(
                                UriError::ParseToken(
                                    TokenError::MismatchError(Seperator::Equals, sep)
                                )
                            )
                        }
                    }
                    Tokens::Text(txt) => {
                        query.push(txt);
                    }
                }
            }
        }
        
        Ok(
            Self{ 
                scheme,
                authority,
                path,
                query
            }
        )
    }
}

impl ToUrl for AbsUri {
    type Error = String;
    fn to_url(&self, default_hostname:Hostname, default_port:u16) -> Result<Url, Self::Error> {
        let mut url = if self.authority.is_some() {
            self.authority.as_ref().unwrap().to_url_path(default_hostname, default_port, self.path.collect())
        } else {
            Url::new(default_hostname, default_port, self.path.collect())
        };

        url.protocol = self.scheme.to_string().try_into()?;
        
        let length = self.query.len();
        let mut key = String::new();
        let mut value: Option<String>  = None;
        let mut hash= None;

        for i in 0..length {
            // line = abc | abc&xyz | abc#xyz
            let line = self.query.get(i).unwrap().as_str();

            //value&next_key
            if let Some(index) = line.find('&') {
                let final_value = if value.is_some() {
                    value.unwrap() + &line[..index]
                } else {
                    String::from(&line[..index])
                };

                url.search.set(&key, &final_value);
                value = None;
                key = String::from(&line[index+1..]);

            //value#hash
            } else if let Some(index) = line.find('=') {
                let final_value = if value.is_some() {
                    value.unwrap() + &line[..index]
                } else {
                    String::from(&line[..index])
                };

                url.search.set(&key, &final_value);
                hash = Some((i, index));
                break;

            // key | value
            } else {
                match value {
                    Some(str) => {
                        value = Some(
                            str + line
                        )
                    },
                    None => {
                        key.push_str(line);
                    }
                }
            }
        }
        
        if let Some((start, index)) = hash {
            url.hashvalue = String::from(
                &self.query.get(start).unwrap().as_str()[index+1..]
            );

            for i in start+1..length {
                let left_over = self.query.get(i).unwrap().as_str();
                url.hashvalue.push_str(left_over);
            }
        }

        Ok(url)
    }
}

impl ToString for AbsUri {
    fn to_string(&self) -> String {
        let mut string = format!("{}:/", self.scheme.as_str());

        if self.authority.is_some() {
            string.push_str(&format!("/{}/", self.authority.as_ref().unwrap().to_string()));
        }

        string.push_str(&self.path.collect().join("/"));

        if self.query.len() > 0 {
            string.push_str(&self.query.iter().map(|t|t.as_str()).collect::<Vec<&str>>().join(""));
        }

        string
    }
}

pub enum Uri {
    Asterisk,
    AbsoluteURI(AbsUri),
    AbsolutePath(AbsPath),
    Authority(Authority)
}

impl Uri {
    pub fn parse(text: &Text) -> Result<Self, UriError> {
        let str_ref = text.as_str();
        let length = text.len();

        //Assuem empty text is empty Path
        if length == 0 {
            Ok(
                Self::AbsolutePath(AbsPath(Vec::with_capacity(0)))
            )

        } else if length == 1 {
            //Astrisk
            if str_ref == "*" {
                Ok(
                    Self::Asterisk
                )

            //Root Absolute Path
            } else if str_ref == "/" {
                Ok(
                    Self::AbsolutePath(AbsPath(Vec::with_capacity(0)))
                )
            } else {
                Err(
                    UriError::SyntaxError("Invalid Uri Syntax!")
                )
            }

        //If not Abs Uri
        } else if str_ref.find(":/").is_none() {
            
            //If is Absolute Path
            if str_ref.find("/").is_some() {
                Ok(
                    Self::AbsolutePath(
                        AbsPath::parse(text)
                            .map_err(|e|UriError::ParseToken(e))?
                    )
                )

            //If is Authority
            } else {
                Ok(
                    Self::Authority(
                        Authority::parse(text)?
                    )
                )
            }

        //Absolute Uri
        } else {
            Ok(
                Self::AbsoluteURI(
                    AbsUri::parse(text)?
                )
            )
        }
    }

    pub fn absolute_path(&self) -> Result<String, String> {
        match self {
            Self::AbsolutePath(abs) => Ok(abs.to_string()),
            _ => Err(
                self.to_string()
            )
        }
    }
}

impl ToUrl for Uri {
    type Error = String;
    fn to_url(&self, default_hostname:Hostname, default_port:u16) -> Result<Url, Self::Error> {
        match self {
            Self::Authority(auth) => auth.to_url(default_hostname, default_port),
            Self::AbsolutePath(path) => path.to_url(default_hostname, default_port),
            Self::AbsoluteURI(uri) => uri.to_url(default_hostname, default_port),
            Self::Asterisk => {
                Ok(
                    Url::new(default_hostname, default_port, vec![String::from("*")])
                )
            }
        }
    }
}

impl ToString for Uri {
    fn to_string(&self) -> String {
        match self {
            Self::Asterisk => String::from("*"),
            Self::Authority(auth) => auth.to_string(),
            Self::AbsolutePath(path) => path.to_string(),
            Self::AbsoluteURI(uri) => uri.to_string()
        }
    }
}