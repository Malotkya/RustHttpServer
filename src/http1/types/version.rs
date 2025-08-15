///////////////////////////////////////////////////////////////
/// Http Version
/// RFC-2616 3.1
/// https://datatracker.ietf.org/doc/html/rfc2616#section-3.1
///////////////////////////////////////////////////////////////
/// HTTP-Version   = "HTTP" "/" 1*DIGIT "." 1*DIGIT
///////////////////////////////////////////////////////////////
use http::Version;
use super::{Text, ParseText};

impl ParseText for Version {
    type Error = String;
    fn parse(value: &Text) -> Result<Self, Self::Error>{
        // list = ["HTTP", "/", "\d.\d"]
        let mut list = value.tokenize();

        if let Some(value) = list.pop() {
            let major = match value.0.get(0) {
                Some(c) => match c.cast_digit() {
                    Ok(d) => d,
                    Err(e) => {
                        return Err(format!("{}", e))
                    }
                },
                None => {
                    return Err(String::from("Missing Major value!"));
                }
            };

            let minor = match value.0.get(0) {
                Some(c) => match c.cast_digit() {
                    Ok(d) => d,
                    Err(e) => {
                        return Err(format!("{}", e))
                    }
                },
                None => {
                    return Err(String::from("Missing Minor value!"));
                }
            };

            Ok(Self{major, minor})
        } else {
            Err(String::from("Text is blank."))
        }
    }
}