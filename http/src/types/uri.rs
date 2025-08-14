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
use super::{Text, ParseText};


pub struct Authority {
    host: Text,
    port: Option<u16>,
    user: Option<Text>
}

impl ParseText for Authority {
    type Error = String;
    fn parse(value:&Text) -> super::Result<Self, Self::Error> {
        let list = value.tokenize();

        match list.len() {
            //host
            1 => {
                Ok(
                    Self {
                        host: *list.get(0).unwrap(),
                        port: None,
                        user: None
                    }
                )
            },
            //host:port or user@host
            3 => {  
                let user:Option<Text>;
                let host:Text;
                let port:Option<u16>;
                let char = list.get(1).unwrap();
                if *char == "@" {
                    user = Some(*list.get(0).unwrap());
                    host = *list.get(2).unwrap();
                    port = None;
                } else if *char == ":" {
                    user = None;
                    port = Some(list.get(2).unwrap().into()?);
                    host = *list.get(0).unwrap();
                } else {
                    return Err(String::from("Invalid Authority Syntax!"));
                }
                
                Ok(Self{host, user, port})
            },
            //user@host:port
            5 => {
                if *list.get(1).unwrap() != "@" || *list.get(3).unwrap() != ":"{
                    return Err(String::from("Invalid Authority Syntax!"));
                }

                Ok(
                    Self{ 
                        user: Some(*list.get(0).unwrap()),
                        host: *list.get(2).unwrap(),
                        port: Some(list.get(4).unwrap().into()?)
                    }
                )
            },
            _ => {
                Err(String::from("Invalid Authority Layout!"))
            }
        }
    }
}


use crate::Url;

pub enum Uri {
    Asterisk,
    AbsoluteURI(Url),
    AbsolutePath(String),
    Authority(String, String)
}

pub type UriParseError = String;

impl Uri {
    pub fn parse(value: &str) -> Result<Uri, UriParseError> {
        if value == "*" {
            return Ok(Self::Asterisk);
        }

        match value.find("http") {
            Some(index) =>{
                if  index == 0 {
                    match Url::parse(value) {
                        Ok(url) => {
                            return Ok(
                                Self::AbsoluteURI(url)
                            );
                        },
                        Err(e) => {
                            return Err(
                                format!("{}", e)
                            )
                        }
                    }
                    
                }
            },
            None => {}
        }
        
        match value.find(":") {
            Some(index) => {
                return Ok(
                    Self::Authority(
                        String::from(&value[..index]),
                        String::from(&value[index+1..])
                    )
                )
            },
            None => {}
        }

        match value.find("/") {
            Some(index) => {
                if index == 0 {
                    return Ok (
                        Self::AbsolutePath(String::from(value))
                    )
                }
            },
            None => {}
        }

        Err(
            format!("Invalid uri: '{}'", value)
        )
    }
}