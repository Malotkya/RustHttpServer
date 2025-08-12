/// Http Request URI
/// 
/// RFC-2616 5.1.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-5
/// 
/// Request-URI    = "*" | absoluteURI | abs_path | authority
/// 
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