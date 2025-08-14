///////////////////////////////////////////////////////////////
/// Basic Http Types
/// RFC-2616 2.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-2.2
///////////////////////////////////////////////////////////////
use std::collections::VecDeque;
use std::fmt;
mod version;
mod url;
mod uri;

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
            Self::InvalidAlphaChar(c) => write!(f, "'{}' is not a valid Html Alpha value!", c),
            Self::InvalidDigitChar(c) => write!(f, "'{}' is not a valid Html Digit value!", c)
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Octet(u8);

impl Into<u8> for Octet {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<char> for Octet {
    fn into(self) -> char {
        self.0 as char
    }
}

impl<'a> Into<&'a u8> for &'a Octet {
    fn into(self) -> &'a u8 {
        &self.0
    }
}

impl<'a> Into<&'a char> for &'a Octet {
    fn into(self) -> &'a char {
        unsafe{ &*(( (&self.0) as *const u8) as *const char) }
    }
}

impl PartialEq<u8> for Octet {
    fn eq(&self, other:&u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for Octet {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }

    fn lt(&self, other: &u8) -> bool {
        self.0 < *other
    }

    fn gt(&self, other: &u8) -> bool {
        self.0 > *other
    }
}

impl PartialEq<char> for Octet {
    fn eq(&self, other:&char) -> bool {
        self.0 == *other as u8
    }
}

impl fmt::Display for Octet {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> fmt::Result {
        let b:u8 = (*self).into();
        write!(f, "{}", b)
    }
}

impl Octet {
    pub fn encode(&self) -> Char {
        match self.0 {
            0..=9 => Char(Self(b'0' + self.0)),
            10..=255 => Char(Self(b'A' - 10 + self.0))
        }
    }
}

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
        Self(Octet(c as u8))
    }

    pub fn is_alpha(&self) -> bool {
        ( self.0 >= b'a' && self.0 <= b'z')
            || (self.0 >= b'A' && self.0 <= b'Z')
    }

    pub fn is_digit(&self) -> bool {
        self.0 >= b'0' && self.0 <= b'9'
    }

    pub fn is_ctl(&self) -> bool {
        (self.0 >=0 && self.0 <= 31) || self.0 == 127
    }

    pub fn is_lws(&self) -> bool {
        *self == SP || *self == HT || *self == CR || *self == LF
    }

    pub fn is_text(&self) -> bool {
        !self.is_ctl() || self.is_lws()
    }

    pub fn is_hex(&self) -> bool {
        (self.0 >= b'a' && self.0 <= b'f')
            || (self.0 >= b'A' && self.0 <= b'F')
            || self.is_digit()
    }

    pub fn is_separator(&self) -> bool {
        match self.0.into() {
            '(' | ')' | '<' | '>' | '@' |
            ',' | ';' | ':' | '\\'| '"' |
            '/' | '[' | ']' | '?' | '=' |
            '{' | '}' => true,
            _ => false
        }
    }

    pub fn encode(&self) -> Text {
        let b:u8 = self.0.into();
        Text(vec![Char(Octet(b'%')), Octet(b >> 4).encode(), Octet(b & 15).encode()])
    }

    pub fn is_token(&self) -> bool {
        !self.is_ctl() && !self.is_separator() && !self.is_lws()
    }

    pub fn cast_digit(&self) -> Result<u8, TypeError> {
        if self.is_digit() {
            Ok(self.0.0 - b'0')
        } else {
            Err(TypeError::InvalidDigitChar(self.clone()))
        }
    }

    unsafe fn cast_digit_unchecked(&self) -> u8 {
        self.0.0 - b'0'
    }
}

impl TryFrom<Octet> for Char {
    type Error = TypeError;
    fn try_from(b:Octet) -> std::result::Result<Self, Self::Error> {
        if b.0 > MAX_ASCII_VALUE {
            Err(TypeError::AsciiOutOfBounds(b))
        } else {
            Ok(Self(b))
        }
    }
}

impl TryFrom<char> for Char {
    type Error = TypeError;
    fn try_from(c:char) -> std::result::Result<Self, Self::Error> {
        Octet(c as u8).try_into()
    }
}

impl PartialEq<char> for Char {
    fn eq(&self, other:&char) -> bool {
        self.0 == *other as u8
    }
}

impl PartialOrd<char> for Char {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp( &(*other as u8) )
    }

    fn lt(&self, other: &char) -> bool {
        self.0 < *other as u8
    }

    fn gt(&self, other: &char) -> bool {
        self.0 > *other as u8
    }
}

impl Into<Octet> for Char {
    fn into(self) -> Octet {
        self.0
    }
}

impl Into<char> for Char {
    fn into(self) -> char {
        self.0.into()
    }
}

impl<'a> Into<&'a char> for &'a Char {
    fn into(self) -> &'a char {
        Into::<&'a char>::into(&self.0)
    }
}

impl Into<u8> for Char {
    fn into(self) -> u8 {
        self.0.0
    }
}

impl<'a> PartialEq<Char> for &'a Char {
    fn eq(self: &&'a Char, other:&Char) -> bool {
        **self == *other
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> fmt::Result {
        let c:char = (*self).into();
        write!(f, "{}", c)
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

    pub fn encode(&self) -> Self {
        let mut vec:Vec<_> = Vec::with_capacity(self.0.len());

        for c in self.0.iter() {
            match (*c).into() {
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' |  b'-' | b'.' | b'_' | b'~' => {
                    vec.push(c.clone());
                },
                _ => {
                    vec.append(&mut c.encode().0)
                },
            }
        }

        Self(vec)
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

impl Into<String> for Text {
    fn into(self) -> String {
        self.0.iter().map(|c|-> &char {c.into()}).collect()
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other:&&str) -> bool {
        let len = self.0.len();
        if len != other.len()  {
            return false;
        }

        let mut chars = other.chars();
        for i in 0..len {
            if *self.0.get(i).unwrap() != chars.nth(i).unwrap() {
                return false;
            }
        }

        return true;
    }
}

impl PartialOrd<&str> for Text {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        let len = if self.0.len() > other.len() {
            self.0.len()
        } else {
            other.len()
        };

        let mut chars = other.chars();
        for i in 0..len {
            let lhs = self.0.get(i);
            let rhs = chars.next();

            if lhs.is_some() && rhs.is_some()  {
                let r = lhs.unwrap().partial_cmp(&rhs.unwrap());

                if r.is_some() && r.unwrap() == std::cmp::Ordering::Equal {
                    continue;
                }

                return r;
            } else if lhs.is_some() {
                return Some(std::cmp::Ordering::Greater);
            } else if rhs.is_some() {
                return Some(std::cmp::Ordering::Less);
            } else {
                return None;
            }
        }

        return Some(std::cmp::Ordering::Equal);
    }
}

impl TryInto<u16> for Text {
    type Error = TypeError;
    fn try_into(self) -> std::result::Result<u16, Self::Error> {
        let mut value: u16 = 0;
        for char in self.0 {
            value *= 10;
            value += char.cast_digit()? as u16;
        }

        Ok(value)
    }
}

pub trait ParseText where Self: Sized {
    type Error;
    fn parse(value:&Text) -> Result<Self, Self::Error>;
}