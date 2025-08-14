///////////////////////////////////////////////////////////////
/// Basic Http Types
/// RFC-2616 2.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-2.2
///////////////////////////////////////////////////////////////
use std::collections::VecDeque;
use std::fmt;
mod version;

pub type Result<T, E: fmt::Display> = std::result::Result<T, E>;

const MAX_ASCII_VALUE:u8 = 127;

enum TypeError {
    AsciiOutOfBounds(Octet),
    InvalidAlphaChar(Char),
    InvalidDigitChar(Char)
}

impl fmt::Display for TypeError {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::AsciiOutOfBounds(b) => write!(f, "{} is not a valid Html Ascii value!", b),
            Self::InvalidAlphaChar(c) => write!(f, "'{}' is not a valid Html Alpha value!", c.0 as char),
            Self::InvalidDigitChar(c) => write!(f, "'{}' is not a valid Html Digit value!", c.0 as char)
        }
    }
}

pub type Octet = u8;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Char(Octet);

pub const CR:Char = Char::new('\r');
pub const LF:Char = Char::new('\n');
pub const SP:Char = Char::new(' ');
pub const HT:Char = Char::new('\t');
pub const QUOTES:Char = Char::new('"');

lazy_static::lazy_static!{
    static ref CRLF:Text = Text(vec![CR, LF]);
}

impl Char {
    const fn new(c:char) -> Self{
        Self(c as u8)
    }

    fn is_alpha(&self) -> bool {
        (self.0 >= 'a' as u8 && self.0 <= 'z' as u8)
            || (self.0 >= 'A' as u8 && self.0 <= 'Z' as u8)
    }

    fn is_digit(&self) -> bool {
        self.0 >= '0' as u8 && self.0 <= '9' as u8
    }

    fn is_ctl(&self) -> bool {
        (self.0 >=0 && self.0 <= 31) || self.0 == 127
    }

    fn is_lws(&self) -> bool {
        *self == SP || *self == HT || *self == CR || *self == LF
    }

    fn is_text(&self) -> bool {
        !self.is_ctl() || self.is_lws()
    }

    fn is_hex(&self) -> bool {
        (self.0 >= 'a' as u8 && self.0 <= 'f' as u8)
            || (self.0 >= 'A' as u8 && self.0 <= 'F' as u8)
            || self.is_digit()
    }

    fn is_separator(&self) -> bool {
        match self.0 as char {
            '(' | ')' | '<' | '>' | '@' |
            ',' | ';' | ':' | '\\'| '"' |
            '/' | '[' | ']' | '?' | '=' |
            '{' | '}' => true,
            _ => false
        }
    }

    fn is_token(&self) -> bool {
        !self.is_ctl() && !self.is_separator() && !self.is_lws()
    }

    fn cast_digit(&self) -> Result<u8, TypeError> {
        if self.is_digit() {
            Ok(self.0 - '0' as u8)
        } else {
            Err(TypeError::InvalidDigitChar(self.clone()))
        }
    }

    unsafe fn cast_digit_unchecked(&self) -> u8 {
        self.0 - '0' as u8
    }
}

impl TryFrom<Octet> for Char {
    type Error = TypeError;
    fn try_from(b:Octet) -> std::result::Result<Self, Self::Error> {
        if b > MAX_ASCII_VALUE {
            Err(TypeError::AsciiOutOfBounds(b))
        } else {
            Ok(Self(b))
        }
    }
}

impl Into<Octet> for Char {
    fn into(self) -> Octet {
        self.0
    }
}

impl<'a> PartialEq<Char> for &'a Char {
    fn eq(self: &&'a Char, other:&Char) -> bool {
        **self == *other
    }
}

#[derive(Eq, Clone)]
pub struct Text(Vec<Char>);

impl PartialEq for Text {
    fn eq(&self, value:&Self) -> bool {
        let len = self.0.len();
        if len != value.0.len() {
            return false;
        }

        for i in 0..len {
            if self.0.get(i).unwrap() != 
                value.0.get(i).unwrap() {

                return false;
            }
        }

        return true;
    }
}

impl Text {
    pub fn tokenize(&self) -> Vec<Text> {
        let mut vec: Vec<_> = Vec::new();
        let mut str: Vec<Char> = Vec::new();

        for c in &self.0[..] {
            if c.is_token() {
                str.push(c.clone());
            } else {
                if str.len() > 0 {
                    vec.push(Self(str));
                    str = Vec::new()
                }
                
                if c.is_separator() {
                    vec.push(Text(vec![c.clone()]))
                }
            }
        }

        vec
    }

    pub fn split(&self, value:Char) -> Vec<Text>{
        let mut vec: Vec<_> = Vec::new();
        let mut str = Text(Vec::new());

        for c in &self.0 {
            if value == *c {
                vec.push(str);
                str = Text(Vec::new());
            } else {
                str.add(c.clone())
            }
        }

        vec
    }

    pub fn lines(&self, value:&Text) -> Vec<Text> {
        let value_len = value.0.len();
        let self_len = self.0.len();

        if  value_len > self_len {
            return vec![self.clone()]
        } else if value_len == self_len {
            if self == value {
                return Vec::new();
            } else {
                return vec![self.clone()]
            }
        }

        let mut compare: VecDeque<&Char> = VecDeque::with_capacity(value_len);
        let mut str: Text = Text(Vec::new());
        let mut vec:Vec<_> = Vec::new();

        let mut i = 0;
        while i < self_len {

            //Fill compare buffer
            while compare.len() < value_len
                  && let Some(value) = self.0.get(i) {

                compare.push_back(value);
                i += 1;
            }

            //Check if buffer matches 
            if compare.iter().eq(value.0.iter()) {
                vec.push(str);
                str = Text(Vec::new());
                compare.clear();
            } else {
                str.add(compare.pop_front().unwrap().clone());
            }
        }

        while compare.len() > 0 {
            str.add(compare.pop_front().unwrap().clone());
        }
        vec.push(str);

        vec
    }

    pub fn add(&mut self, c:Char) {
        self.0.push(c);
    }

    pub fn concat(&mut self, txt:Text) {
        self.0.append(txt.0.to_vec().as_mut())
    }
}

impl TryFrom<&[Octet]> for Text{
    type Error = TypeError;
    fn try_from(str:&[Octet]) -> std::result::Result<Self, Self::Error> {
        let mut vec:Vec<Char> = Vec::new();

        for b in str {
            vec.push((*b).try_into()?);
        }

        Ok(Self(vec))
    }
}

pub trait ParseText where Self: Sized {
    type Error;
    fn parse(value:&Text) -> Result<Self, Self::Error>;
}