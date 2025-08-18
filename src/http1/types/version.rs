///////////////////////////////////////////////////////////////
/// Http Version
/// RFC-2616 3.1
/// https://datatracker.ietf.org/doc/html/rfc2616#section-3.1
///////////////////////////////////////////////////////////////
/// HTTP-Version   = "HTTP" "/" 1*DIGIT "." 1*DIGIT
///////////////////////////////////////////////////////////////
use http::types::Version;
use crate::http1::types::Seperator;

use super::{Text, Tokens, tokenize, Tokenizer};

pub fn parse_version(text:&Text) -> Result<Version, ()> {
    let list = tokenize(text);

    //"HTTP"
    match list.get(0) {
        Some(value) => match value {
            Tokens::Text(http) => {
                if http.as_str().to_uppercase() != "HTTP" {
                    return Err(());
                }
            }
            Tokens::Seperator(_) => return Err(())
        },
        None => return Err(())
            
    }

    //"/"
    match list.get(1) {
        Some(value) => match value {
            Tokens::Seperator(sep) => {
                if *sep != Seperator::ForwardSlash {
                    return Err(())
                }
            },
            Tokens::Text(_) => return Err(())
        },
        None => return Err(())
    }

        //\d.\d"
        let version = match list.get(2) {
            Some(value) => match value {
                Tokens::Text(t) => t,
                Tokens::Seperator(_) => return Err(())
            },
            None => return Err(()),
        };

        let major = match version.at(0) {
            Some(c) => match (*c as char).to_digit(10) {
                Some(d) => d as u8,
                None => return Err(())
            },
            None => return Err(())
        };

        let minor = match version.at(0) {
            Some(c) => match (*c as char).to_digit(10) {
                Some(d) => d as u8,
                None => return Err(())
            },
            None => return Err(())
        };

        Ok(Version{major, minor})
}