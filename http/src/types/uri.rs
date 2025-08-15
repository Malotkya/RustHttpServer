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
use super::{Text, ParseText, Result, Tokens, TokenSeparator};


pub struct Authority {
    host: Text,
    port: Option<u16>,
    user: Option<Text>
}

impl ParseText for Authority {
    type Error = String;
    fn parse(value:&Text) -> Result<Self, Self::Error> {
        let mut list = value.tokenize();

        match list.len() {
            //host
            1 => {
                Ok(
                    Self {
                        host: Tokens::get_text(&mut list, 0)?,
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
                match Tokens::get_seperator(&mut list, 1)? {
                    TokenSeparator::At => {
                        user = Some(Tokens::get_text(&mut list, 0)?);
                        host = Tokens::get_text(&mut list,2)?;
                        port = None
                    },
                    TokenSeparator::Colon => {
                        user = None;
                        host = Tokens::get_text(&mut list,0)?;
                        port = Some(match Tokens::get_text(&mut list, 2)?.try_into(){
                            Ok(port) => port,
                            Err(e) =>{
                                return Err(format!("{}", e))
                            }
                        });
                    },
                    _ => {
                        return Err(String::from("Invalid Authority Syntax!"));
                    }
                }
                
                Ok(Self{host, user, port})
            },
            //user@host:port
            5 => {
                if Tokens::get_seperator(&mut list,1)? != '@'
                    || Tokens::get_seperator(&mut list,3)?  != ':'{
                    return Err(String::from("Invalid Authority Syntax!"));
                }

                Ok(
                    Self{ 
                        user: Some(Tokens::get_text(&mut list, 0)?),
                        host: Tokens::get_text(&mut list,2)?,
                        port: Some(match (Tokens::get_text(&mut list, 2)?).try_into(){
                            Ok(port) => port,
                            Err(e) =>{
                                return Err(format!("{}", e))
                            }
                        })
                    }
                )
            },
            _ => {
                Err(String::from("Invalid Authority Layout!"))
            }
        }
    }
}

pub struct AbsPath(Vec<Text>);

impl ParseText for AbsPath {
    type Error = String;
    fn parse(value: &Text) -> Result<Self, Self::Error> {
        let mut list = value.tokenize().into_iter();
        let mut vec: Vec<Text> = Vec::new();

        //Mace sure to start loop without a seperator at the front.
        if let Some(mut start) = list.next() {
            if start.is_text() {
                vec.push(start.text().unwrap());

                if let Some(sep) = list.next() {
                    match sep {
                        Tokens::Seperator(s) => match s {
                            TokenSeparator::ForwardSlash => {},
                            _ => {
                                return Err(format!("Invalid path seperator '{}'!", Into::<char>::into(s)))
                            }
                        },
                        Tokens::Text(t) => {
                            return Err(format!("Encoutnered segment \'{}\' instead of seperator!", Into::<String>::into(t)))
                        }
                    }
                }
            }
        }

        while let Some(mut segment) = list.next() {
            if segment.is_text() {
                vec.push(segment.text().unwrap())
            } else {
                return Err(format!(
                    "Encoutnered seperator \'{}\' instead of path segment!",
                    Into::<char>::into(segment.seperator().unwrap())
                ));
            }

            if let Some(mut sep) = list.next() {
                if sep.is_seperator() {
                    let sep = sep.seperator().unwrap();
                    if sep != '/' {
                        return Err(format!("Invalid path seperator '{}'!", Into::<char>::into(sep)))
                    }
                } else {
                     return Err(format!(
                        "Encoutnered segment \'{}\' instead of seperator!",
                        Into::<String>::into(sep.text().unwrap())
                    ));
                }
            }
        }

        Ok(Self(vec))
    }
}

struct AbsUri {
    scheme: Text,
    authority: Authority,
    path: AbsPath,
    query: Text
}

impl ParseText for AbsUri {
    type Error = String;
    fn parse(value: &Text) -> Result<Self, Self::Error> {

    }
}

pub enum Uri {
    Asterisk,
    AbsoluteURI(AbsUri),
    AbsolutePath(AbsPath),
    Authority(Authority)
}

impl Uri {
    pub fn parse(value: &str) -> Result<Uri, String> {
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