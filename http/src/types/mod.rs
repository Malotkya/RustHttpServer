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

const CTRL_NULL:char =   0 as char;
const CTRL_US:char   =  31 as char;
const CTRL_DEL:char  = 127 as char;
const LWS_VT:char    = 0x0B as char;

lazy_static::lazy_static!{
    pub static ref CRLF:Text = Text::from_str("\r\n");
}

pub enum TypeError {
    AsciiOutOfBounds(Octet),
    InvalidAlphaChar(char),
    InvalidWhiteSpaceChar(char),
    InvalidDigitChar(char),
    InvalidSyperatorChar(char)
}

impl fmt::Display for TypeError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AsciiOutOfBounds(b) => write!(f, "{} is not a valid Html Ascii character!", b),
            Self::InvalidAlphaChar(c) => write!(f, "'{}' is not a valid Html Alpha char!", c),
            Self::InvalidDigitChar(c) => write!(f, "'{}' is not a valid Html Digit char!", c),
            Self::InvalidWhiteSpaceChar(c) => write!(f, "'{}' is not a valid Html White Space char!", c),
            Self::InvalidSyperatorChar(c) => write!(f, "'{}' is not a valid Html Seperator char!", c)
        }
    }
}

impl fmt::Debug for TypeError {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::AsciiOutOfBounds(b) => write!(f, "octet:{}", b),
            Self::InvalidAlphaChar(c) => write!(f, "char:'{}'", *c as u8),
            Self::InvalidDigitChar(c) => write!(f, "char:'{}'", *c as u8),
            Self::InvalidWhiteSpaceChar(c) => write!(f, "char:'{}'", *c as u8),
            Self::InvalidSyperatorChar(c) => write!(f, "char:'{}'", *c as u8)
        }
    }
}

pub type Octet = u8;

#[inline]
fn hex_encode(b:Octet) -> Octet {
    match b {
        0..=9 => b'0' + b,
        10..=255 => b'A' - 10 + b
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum TokenSeparator {
    OpenParenthesis,
    CloseParenthesis,
    OpenAngleBracket,
    CloseAngleBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    At,
    Comma,
    DoubleQuotes,
    QuestionMark,
    Equals,
    Colon,
    Semicolon,
    ForwardSlash,
    BackSlash
}

impl Into<char> for TokenSeparator {
    fn into(self) -> char {
        match self {
            Self::OpenParenthesis    => '(',
            Self::CloseParenthesis   => ')',
            Self::OpenAngleBracket   => '<',
            Self::CloseAngleBracket  => '>',
            Self::OpenCurlyBracket   => '{',
            Self::CloseCurlyBracket  => '}',
            Self::OpenSquareBracket  => '[',
            Self::CloseSquareBracket => ']',
            Self::At                 => '@',
            Self::Comma              => ',',
            Self::DoubleQuotes       => '"',
            Self::QuestionMark       => '?',
            Self::Equals             => '=',
            Self::Colon              => ':',
            Self::Semicolon          => ';',
            Self::ForwardSlash       => '/',
            Self::BackSlash          => '\\',
        }   
    }
}

impl TryFrom<char> for TokenSeparator {
    type Error = TypeError;
    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::OpenParenthesis),
            ')' => Ok(Self::CloseParenthesis),
            '<' => Ok(Self::OpenAngleBracket),
            '>' => Ok(Self::CloseAngleBracket),
            '{' => Ok(Self::OpenCurlyBracket),
            '}' => Ok(Self::CloseCurlyBracket),
            '[' => Ok(Self::OpenSquareBracket),
            ']' => Ok(Self::CloseSquareBracket),
            '@' => Ok(Self::At),
            ',' => Ok(Self::Comma),
            '"' => Ok(Self::DoubleQuotes),
            '?' => Ok(Self::QuestionMark),
            '=' => Ok(Self::Equals),
            ':' => Ok(Self::Colon),
            ';' => Ok(Self::Semicolon),
            '/' => Ok(Self::ForwardSlash),
            '\\' => Ok(Self::BackSlash),
            _ => Err(TypeError::InvalidSyperatorChar(c))
        }
    }
}

impl PartialEq<char> for TokenSeparator {
    fn eq(&self, other: &char) -> bool {
        Into::<char>::into(*self) == *other
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum LinearWhiteSpace {
    CarriageReturn,
    LineFeed,
    VerticalTab,
    HorizontalTab,
    Space,
}

impl Into<char> for LinearWhiteSpace {
    fn into(self) -> char {
        match self {
            Self::CarriageReturn => '\r',
            Self::LineFeed       => '\n',
            Self::VerticalTab    => LWS_VT,
            Self::HorizontalTab  => '\t',
            Self::Space          => ' '
        }   
    }
}

impl TryFrom<char> for LinearWhiteSpace {
    type Error = TypeError;
    fn try_from(c:char) -> std::result::Result<Self, Self::Error> {
        match c {
            '\r' =>   Ok(Self::CarriageReturn),
            '\n' =>   Ok(Self::LineFeed),
            LWS_VT => Ok(Self::VerticalTab),
            '\t' =>   Ok(Self::HorizontalTab),
            ' ' =>    Ok(Self::Space),
            _ => Err(TypeError::InvalidWhiteSpaceChar(c))
        } 
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AsciiChar { 
    Control(char),    // 0-31, 127
    Symbol(char),     // 32 - 37, 58-64, 91-96, 123-126
    Digit(char),      // 48 - 47
    UpperAlpha(char), // 65 - 90
    LowerAlpha(char)  // 97 - 122
}

enum AsciiToken {
    Seperator(TokenSeparator),
    Character(char),
    EmptySpace
}

impl TryFrom<char> for AsciiChar {
    type Error = TypeError;
    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '0'..='9' => Ok(Self::Digit(c)),
            'A'..='Z' => Ok(Self::UpperAlpha(c)),
            'a'..='z' => Ok(Self::LowerAlpha(c)),
            ' '..='/' |
            ':'..='@' |
            '['..='`' |
            '{'..='~' => Ok(Self::Symbol(c)),
            CTRL_NULL..=CTRL_US |
            CTRL_DEL => Ok(Self::Control(c)),
            _ => Err(TypeError::AsciiOutOfBounds(c as u8))
        }
    }
}

impl TryFrom<Octet> for AsciiChar {
    type Error = TypeError;
    fn try_from(b: Octet) -> std::result::Result<Self, Self::Error> {
        match b {
            b'0'..=b'9' => Ok(Self::Digit(b as char)),
            b'A'..=b'Z' => Ok(Self::UpperAlpha(b as char)),
            b'a'..=b'z' => Ok(Self::LowerAlpha(b as char)),
            b' '..=b'/' |
            b':'..=b'@' |
            b'['..=b'`' |
            b'{'..=b'~' => Ok(Self::Symbol(b as char)),
            0..=31 | 127 => Ok(Self::Control(b as char)),
            _ => Err(TypeError::AsciiOutOfBounds(b))
        }
    }
}

impl Into<char> for AsciiChar {
    fn into(self) -> char {
        match self {
            Self::Control(c) => c,
            Self::Digit(c) => c,
            Self::LowerAlpha(c) => c,
            Self::UpperAlpha(c) => c,
            Self::Symbol(c) => c
        }
    }
}

impl<'a> Into<&'a char> for &'a AsciiChar {
    fn into(self) -> &'a char {
        match self {
            AsciiChar::Control(c) => c,
            AsciiChar::Digit(c) => c,
            AsciiChar::LowerAlpha(c) => c,
            AsciiChar::UpperAlpha(c) => c,
            AsciiChar::Symbol(c) => c
        }
    }
}

impl Into<Octet> for AsciiChar {
    fn into(self) -> Octet {
        match self {
            Self::Control(c) => c as Octet,
            Self::Digit(c) => c as Octet,
            Self::LowerAlpha(c) => c as Octet,
            Self::UpperAlpha(c) => c as Octet,
            Self::Symbol(c) => c as Octet
        }
    }
}

impl AsciiChar {
    pub fn is_alpha(&self) -> bool {
        match self {
            Self::UpperAlpha(_) |
            Self::LowerAlpha(_) => true,
            _ => false
        }
    }

    pub fn is_digit(&self) -> bool {
        if let Self::Digit(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_ctl(&self) -> bool {
        if let Self::Control(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_text(&self) -> bool {
        if let Self::Control(_) = self {
            false
        } else {
            true
        }
    }

    pub fn is_lws(&self) -> bool {
        match self {
            Self::Control(char) =>
                *char == '\t' || *char == LWS_VT ||
                *char == '\r' || *char == '\n',
            Self::Symbol(char) => *char == ' ',
            _ => false
        }
    }

    pub fn is_separator(&self) -> bool {
        if let Self::Symbol(char) = self {
            *char == '(' || *char == ')' ||
            *char == '<' || *char == '>' ||
            *char == '{' || *char == '}' ||
            *char == '[' || *char == ']' ||
            *char == '@' || *char == ',' ||
            *char == '"' || *char == '?' ||
            *char == '=' || *char == ':' ||
            *char == ';' || *char == '/' ||
            *char == '\\'
        } else {
            false
        }
    }

    pub fn is_hex(&self) -> bool {
        match self {
            Self::Digit(_) => true,
            Self::LowerAlpha(c) => *c >= 'a' && *c <= 'z',
            Self::UpperAlpha(c) => *c >= 'A' && *c <= 'Z',
            _ => false
        }
    }

    pub fn cast_digit(&self) -> Result<u8, TypeError> {
        match self {
            Self::Digit(c) => Ok((*c as u8) - b'0'),
            _ => Err(TypeError::InvalidDigitChar(Into::<char>::into(*self)))
        }
    }

    pub unsafe fn cast_digit_unchecked(&self) -> u8 {
        (*Into::<&char>::into(self) as u8) - b'0'
    }

    pub fn tokenize(&self) -> AsciiToken {
        if let Self::Control(_) = *self {
            AsciiToken::EmptySpace
        } else {
            let value:char = (*self).into();

            if let Ok(sep) = TryInto::<TokenSeparator>::try_into(value) {
                AsciiToken::Seperator(sep)
            } else if value == ' ' {
                AsciiToken::EmptySpace
            } else {
                AsciiToken::Character(value)
            }
        }
    }

    pub fn encode(&self) -> Text {
        let b:Octet = (*self).into();
        Text::from_enc(&[b'%', hex_encode(b >> 4), hex_encode(b & 15)])
    }
}

impl PartialEq<char> for AsciiChar {
    fn eq(&self, other:&char) -> bool {
        *Into::<&char>::into(self) == *other
    }
}

impl<'a> PartialEq<AsciiChar> for &'a AsciiChar {
    fn eq(&self, other:&AsciiChar) -> bool {
        **self == *other
    }
}

impl PartialOrd<char> for AsciiChar {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        Into::<&char>::into(self).partial_cmp( other )
    }

    fn lt(&self, other: &char) -> bool {
        *Into::<&char>::into(self) < *other
    }

    fn gt(&self, other: &char) -> bool {
        *Into::<&char>::into(self) > *other
    }
}

#[derive(Eq, Clone)]
pub struct Text(Vec<AsciiChar>);

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
    fn from_str<'a>(value: &'a str) -> Self {
        let mut vec:Vec<_> = Vec::with_capacity(value.len());

        for c in value.chars() {
            vec.push(TryInto::<AsciiChar>::try_into(c).unwrap())
        }

        Self(vec)
    }

    fn from_enc<'a>(value:&'a [Octet]) -> Self {
        Self(
            value.iter().map(|b| -> AsciiChar {
                TryInto::<AsciiChar>::try_into(*b).unwrap()
            }).collect()
        )
    }

    pub fn tokenize(&self) -> Vec<Tokens> {
        let mut vec: Vec<_> = Vec::new();
        let mut str: Vec<AsciiChar> = Vec::new();

        for c in &self.0[..] {
            match c.tokenize() {
                AsciiToken::Character(c) => {
                    str.push(c.try_into().unwrap());
                },
                AsciiToken::Seperator(sep) => {
                    if str.len() > 0 {
                        vec.push(Tokens::from(str));
                        str = Vec::new();
                    }

                    vec.push(Tokens::Seperator(sep))
                },
                AsciiToken::EmptySpace => {
                    if str.len() > 0 {
                        vec.push(Tokens::from(str));
                        str = Vec::new();
                    }
                }
            }
        } //End For Loop

        if str.len() > 0 {
            vec.push(Tokens::from(str));
        }

        vec
    }

    pub fn split(&self, value:AsciiChar) -> Vec<Text>{
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

        let mut compare: VecDeque<&AsciiChar> = VecDeque::with_capacity(value_len);
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

    pub fn add(&mut self, c:AsciiChar) {
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

impl TryFrom<&str> for Text{
    type Error = TypeError;
    fn try_from(str:&str) -> std::result::Result<Self, Self::Error> {
        let mut vec:Vec<_> = Vec::new();

        for b in str.chars() {
            vec.push(b.try_into()?);
        }

        Ok(Self(vec))
    }
}

impl TryFrom<&[Octet]> for Text{
    type Error = TypeError;
    fn try_from(str:&[Octet]) -> std::result::Result<Self, Self::Error> {
        let mut vec:Vec<_> = Vec::new();

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

impl Into<String> for &Text {
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

pub enum Tokens {
    Seperator(TokenSeparator),
    Text(Text),
}

impl Tokens {
    fn from(value:Vec<AsciiChar>) -> Self {
        Self::Text(Text(value))
    }

    fn empty() -> Self {
        Self::Text(Text(Vec::with_capacity(0)))
    }

    pub fn seperator(&self) -> Result<TokenSeparator, TypeError> {
        match self {
            Self::Seperator(s) => Ok(*s),
            Self::Text(c) => Err(TypeError::InvalidSyperatorChar({
                let c = c.0.get(0);
                if c.is_some() {
                    (*c.unwrap()).into()
                } else {
                    '~'
                }
            }))
        }
    }

    pub fn is_seperator(&self) -> bool {
        if let Self::Seperator(_) = self {
            true
        } else {
            false
        }
    }

    pub fn text(&mut self) -> Result<Text, TypeError> {
        match self {
            Self::Text(t) => {
                Ok(std::mem::replace(t, Text(Vec::with_capacity(0))))
            },
            Self::Seperator(s) => Err(TypeError::InvalidAlphaChar((*s).into()))
        }
    }

    pub fn is_text(&self) -> bool {
        if let Self::Text(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_seperator(vec: &mut Vec<Tokens>, i:usize) -> Result<TokenSeparator, String> {
        match vec.get(i) {
            Some(t) => match t {
                Self::Seperator(_) => {
                    let v = vec.remove(i);
                    vec.insert(i, Self::empty());
                    Ok(v.seperator().unwrap())
                },
                Self::Text(t) => Err(format!("Unexpeted test \"{}\" at: {}!", Into::<String>::into(t), i)),
            },
            None => Err(format!("Missing seperator at: {}!", i))
        }
    }

    pub fn get_text(vec:&mut Vec<Tokens>, i:usize) -> Result<Text, String> {
        match vec.get(i) {
            Some(t) => match t {
                Self::Text(_) => {
                    let mut v = vec.remove(i);
                    vec.insert(i, Self::empty());
                    Ok(v.text().unwrap())
                },
                Self::Seperator(s) =>
                    Err(format!("Unexpected seperator '{}' at: {}!", Into::<char>::into(*s), i))
            },
            None => Err(format!("Missing text at: {}!", i))
        }
    }
}

pub trait ParseText where Self: Sized {
    type Error;
    fn parse(value:&Text) -> Result<Self, Self::Error>;
}