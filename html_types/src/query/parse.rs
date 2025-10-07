use super::{
    parts::*,
    Query, SubQuery, MatchOptions, find_next_combinator,
    QueryCombinator, QueryParts,
    PsudoClass, PsudoElement
};
use crate::{
    component::attributes::{
        AttributeMatchOperator,
        ToAttributeValue
    }
};
use std::{
    collections::VecDeque,
    iter::Peekable,
    fmt::Display
};

pub trait IntoQuery {
    fn parse(&self) -> Result<Query, QueryParseError>;

    #[inline]
    fn parse_default(&self) -> Query {
        self.parse()
            .ok()
            .unwrap_or(Query::default())
    }
}

impl<T: IntoQuery> IntoQuery for &T {
    fn parse(&self) -> Result<Query, QueryParseError> {
        (*self).parse()
    }
}

#[derive(Debug)]
pub enum QueryParseError {
    UnexpectedChar(usize, char),
    InvalidEscapeCode(usize),
    Other(String),
    UnexpectedEnd
}

impl Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEscapeCode(code) => write!(f, "Invalid escape code: {}!", *code),
            Self::UnexpectedChar(index, char) => write!(f, "Encountered invalid charater '{}' at {}!", *char, *index),
            Self::Other(string) => write!(f, "{}", string),
            Self::UnexpectedEnd => write!(f, "Unexpectedly hit the end when parsing!")
        }
    }
}

fn skip_whitespace(it: &mut impl Iterator<Item = (usize, char)>) {
    let mut peek = it.peekable();

    while let Some((_, char)) = peek.peek() && char.is_whitespace() {
        peek.next();
    }
}

fn parse_escaped_char(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<char, QueryParseError> {
    let mut unicode:u32;
    let start_index:usize;
    if let Some( (start, mut first)) = it.next() {

        if first == '\\' {
            if let Some((start, next_first)) = it.next() {
                first = next_first;
                start_index = start;
            } else {
                return Err(QueryParseError::UnexpectedEnd)
            }
        } else {
            start_index = start;
        }
        
        match first {
            'a'..='f' | 'A'..='F'
            | '0'..='9' => {
                unicode = first.to_digit(16).unwrap();
            },
            _ => return Ok(first)
        }

        
    } else {
        return Err(QueryParseError::UnexpectedEnd);
    }

    
    for _ in 0..6 {
        if let Some((_, char)) = it.next() {
            match char.to_digit(16) {
                Some(value) => {
                    unicode *= 16;
                    unicode += value;
                },
                None => break
            }
        }
    }

    char::from_u32(unicode)
        .ok_or_else(||QueryParseError::InvalidEscapeCode(start_index))
}

fn parse_name(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<Option<Name>, QueryParseError> {
    let mut namespace: Option<Option<String>> = None;
    let mut value = String::new();

    while let Some((index, char)) = it.peek() {

        match char {
            '|' => if namespace.is_some() {
                return Err(QueryParseError::UnexpectedChar(*index, *char))
            } else {
                if value.is_empty() {
                    namespace = Some(None);
                } else {
                    namespace = Some(Some(value.clone()))
                }
                value.clear();
            },
            '.' | '#' | '[' | ':' => {
                if value.is_empty() {
                    return Ok(None)
                } else {
                    return Ok(Some(
                        Name{
                            namespace: namespace.flatten(),
                            tag_name: value
                        }
                    ))
                }
            },
            '\\' => {
                value.push(parse_escaped_char(it)?)
            }
            _ => if char.is_whitespace() {
                return Ok(Some(
                    Name{
                        namespace: namespace.flatten(),
                        tag_name: value
                    }
                ));
            } else {
                value.push(*char);
            }
        }

        it.next();
    }

    Err(QueryParseError::UnexpectedEnd)
}

fn parse_class(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<Class, QueryParseError> {
    let mut value = String::new();
    if let Some( (mut index, mut first)) = it.next() {
        if first != '.' {
            if let Some((new_index, new_first)) = it.next() {
                index = new_index;
                first = new_first
            } else {
                return Err(QueryParseError::UnexpectedChar(index, first))
            }
        }
        
        if first == '\\' {
            value.push(parse_escaped_char(it)?);
        } else if first.is_whitespace() {
            return Err(QueryParseError::UnexpectedChar(index, first))
        } else {
            value.push(first)
        }
    } else {
        return Err(QueryParseError::UnexpectedEnd)
    }


    while let Some((_, char)) = it.peek()  {
        match char {
            '#' | '[' | ':' => {
                return Ok(Class(value))
            },
            '\\' => {
                value.push(parse_escaped_char(it)?)
            },
            _ => if char.is_whitespace() {
                return Ok(Class(value))
            } else {
                value.push(*char)
            }
        }

        it.next();
    }

    Err(QueryParseError::UnexpectedEnd)
} 

fn parse_id(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<Id, QueryParseError> {
    let mut value = String::new();
    if let Some( (mut index, mut first)) = it.next() {
        if first != '#' {
            if let Some((new_index, new_first)) = it.next() {
                index = new_index;
                first = new_first
            } else {
                return Err(QueryParseError::UnexpectedChar(index, first))
            }
        }
        
        if first == '\\' {
            value.push(parse_escaped_char(it)?);
        } else if first.is_whitespace() {
            return Err(QueryParseError::UnexpectedChar(index, first))
        } else {
            value.push(first)
        }

    } else {
        return Err(QueryParseError::UnexpectedEnd)
    }


    while let Some((_, char)) = it.peek()  {
        match char {
            '.' | '[' | ':' => {
                return Ok(Id(value))
            },
            '\\' => {
                value.push(parse_escaped_char(it)?)
            },
            _ => if char.is_whitespace() {
                return Ok(Id(value))
            } else {
                value.push(*char)
            }
        }

        it.next();
    }

    Err(QueryParseError::UnexpectedEnd)
} 

fn parse_attriute_selector(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<Attribute, QueryParseError> {
    let mut name = String::new();
    
    if let Some( (mut index, mut first)) = it.next() {
        if first != '[' {
            if let Some((new_index, new_first)) = it.next() {
                index = new_index;
                first = new_first
            } else {
                return Err(QueryParseError::UnexpectedChar(index, first))
            }
        }
        
        if first == '\\' {
            name.push(parse_escaped_char(it)?);
        } else if first.is_whitespace() {
            return Err(QueryParseError::UnexpectedChar(index, first))
        } else {
            name.push(first)
        }

    } else {
        return Err(QueryParseError::UnexpectedEnd)
    }

    let mut operator: Option<AttributeMatchOperator> = None;
    while let Some((_index, char)) = it.next() {
        match char {
            '=' => {
                operator = Some(AttributeMatchOperator::Exact);
                break;
            },
            '~' => {
                match it.next() {
                    None => return Err(QueryParseError::UnexpectedEnd),
                    Some((index, char)) => if char != '=' {
                        return Err(QueryParseError::UnexpectedChar(index, char))
                    }
                }
                operator = Some(AttributeMatchOperator::WhitespaceValue);
                break;
            },
            '|' => {
                match it.next() {
                    None => return Err(QueryParseError::UnexpectedEnd),
                    Some((index, char)) => if char != '=' {
                        return Err(QueryParseError::UnexpectedChar(index, char))
                    }
                }
                operator = Some(AttributeMatchOperator::HyphinMatch);  //Dont have implement
                break;
            },
            '^' => {
                match it.next() {
                    None => return Err(QueryParseError::UnexpectedEnd),
                    Some((index, char)) => if char != '=' {
                        return Err(QueryParseError::UnexpectedChar(index, char))
                    }
                }
                operator = Some(AttributeMatchOperator::Prefix);
                break;
            },
            '$' => {
                match it.next() {
                    None => return Err(QueryParseError::UnexpectedEnd),
                    Some((index, char)) => if char != '=' {
                        return Err(QueryParseError::UnexpectedChar(index, char))
                    }
                }
                operator = Some(AttributeMatchOperator::Suffix);
                break;
            },
            '*' => {
                match it.next() {
                    None => return Err(QueryParseError::UnexpectedEnd),
                    Some((index, char)) => if char != '=' {
                        return Err(QueryParseError::UnexpectedChar(index, char))
                    }
                }
                operator = Some(AttributeMatchOperator::Contains);
                break;
            },
            ']' => {
                return Ok(Attribute { name, ops: None })
            },
            _ => if !char.is_whitespace() {
                 name.push(char);
            }
        }
    }

    let operator = if let Some(value) = operator {
        value
    } else {
        return Err(QueryParseError::UnexpectedEnd)
    };

    let value = if let Some((_, next)) = it.peek()
        && (*next == '"' || *next == '\'') {
            parse_string(it)?
    } else {
        let mut value = String::new();

        while let Some((_, char)) = it.next() {
            if char.is_whitespace() {
                break;
            } else {
                value.push(char);
            }
        }

        value
    };

    parse_whitespace(it);

    let mut sensitive = false;
    if let Some((index, last)) = it.next() {
        match last {
            's'|'S' => sensitive = true,
            'i'|'I' => {},
            ']' => return Ok(
                Attribute { name, ops: Some(MatchOptions{
                    ops: operator,
                    value: value.into_value(),
                    sensitive
                }) }
            ),
            _ => return Err(QueryParseError::UnexpectedChar(index, last))
        }
    }

    parse_whitespace(it);
    if let Some((index, last)) = it.next() {
        if last == ']' {
            Ok(
                Attribute { name, ops: Some(MatchOptions{
                    ops: operator,
                    value: value.into_value(),
                    sensitive
                })
            })
        } else {
             Err(QueryParseError::UnexpectedChar(index, last))
        }
    } else {
        Err(QueryParseError::UnexpectedEnd)
    }
}

fn parse_string(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) -> Result<String, QueryParseError> {
    let double_quotes: bool;
    if let Some((index, char)) = it.next() {
        if char == '\'' {
            double_quotes = false;
        } else if char == '"' {
            double_quotes = true;
        } else {
            return Err(QueryParseError::UnexpectedChar(index, char));
        }
    } else {
        return Err(QueryParseError::UnexpectedEnd)
    }

    let mut value = String::new();

    while let Some((_, char)) = it.next() {
        if char == '\'' && !double_quotes {
            return Ok(value)
        } else if char == '"' && double_quotes {
            return Ok(value)
        } else {
            value.push(char)
        }
    }

    Err(QueryParseError::UnexpectedEnd)
}

fn parse_whitespace(it: &mut Peekable<impl Iterator<Item = (usize, char)>>) {
    while let Some((_, char)) = it.peek() {
        if !char.is_whitespace() {
            break;
        } else {
            it.next();
        }
    }
}

fn is_valid_psudo_char(value:&char) -> bool {
    value.is_alphanumeric() || *value == '-' || *value == '_'
}

enum PsudoType {
    Class(PsudoClass),
    Element(PsudoElement)
}

fn parse_psudo_type(it: &mut Peekable<impl Iterator<Item = (usize, char)>>)  -> Result<PsudoType, QueryParseError> {
    let mut value = String::new();
    let mut count:usize = 0;

    while let Some((index, first)) = it.next() {
        if first == ':' {
            count += 1;
        } else if is_valid_psudo_char(&first) {
            value.push(first);
            break;
        } else {
            return Err(QueryParseError::UnexpectedChar(index, first)); 
        }
    }

    while let Some((index, next)) = it.peek() {
        if next.is_whitespace() {
            break;
        } else if is_valid_psudo_char(next) {
            value.push(*next);
        } else {
            return Err(QueryParseError::UnexpectedChar(*index, *next))
        }

        it.next();
    }

    Ok(if count < 2 {
        PsudoType::Class(
            TryInto::<PsudoClass>::try_into(value.as_str())
                .map_err(|str|QueryParseError::Other(str))?
        )
    } else {
        PsudoType::Element(
            TryInto::<PsudoElement>::try_into(value.as_str())
                .map_err(|str|QueryParseError::Other(str))?
        )
    })
}

fn parse_parts(combinator:QueryCombinator, string:&str) -> Result<QueryParts, QueryParseError> {
    let mut it = string.chars().enumerate().peekable();
    
    skip_whitespace(&mut it);

    let name = parse_name(&mut it)?;
    let mut class: Option<Class> = None;
    let mut id: Option<Id> = None;
    let mut attributes: Vec<Attribute> = Vec::new();
    let mut psudo_class: Vec<PsudoClass> = Vec::new();
    let mut psudo_element: Option<PsudoElement> = None;

    while let Some((index, next)) = it.peek() {
        //Move out of borrow
        let index = index.clone();
        let next = next.clone();

        match next {
            '#' => if id.is_none() {
                id = Some(parse_id(&mut it)?);
            } else {
                return Err(
                    QueryParseError::UnexpectedChar(index, next)
                )
            },
            '.' => if class.is_none() {
                class = Some(parse_class(&mut it)?);
            } else {
                return Err(
                    QueryParseError::UnexpectedChar(index, next)
                )
            },
            '[' => {
                attributes.push(
                    parse_attriute_selector(&mut it)?
                )
            },
            ':' => {
                 match parse_psudo_type(&mut it)? {
                    PsudoType::Class(c) => psudo_class.push(c),
                    PsudoType::Element(e) => if psudo_element.is_none() {
                        psudo_element = Some(e);
                    } else {
                        return Err(
                            QueryParseError::UnexpectedChar(index, next)
                        )
                    }
                 }
            },
            _ => if next.is_whitespace() {
                parse_whitespace(&mut it)
            } else {
                return Err(
                    QueryParseError::UnexpectedChar(index, next)
                )
            }
        }
    }

    Ok(QueryParts{
        combinator,
        name, id, class,
        attributes, psudo_class, psudo_element
    })
}

impl TryFrom<String> for SubQuery {
    type Error = QueryParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryInto::<Self>::try_into(value.as_str())
    }
}

impl TryFrom<&str> for SubQuery {
    type Error = QueryParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = Vec::new();
        let mut combinator = QueryCombinator::Descendant;
        let mut sub_start:usize = 0;

        while let Some((next, sub_end, next_start)) = find_next_combinator(&value[sub_start..]) {
            parts.push(
                parse_parts(combinator.clone(), &value[sub_start..sub_end])
                    .map_err(|mut err|{
                        if let QueryParseError::UnexpectedChar(pos, _) = &mut err {
                            *pos += sub_start;
                        }
                        err
                    })?
            );
            combinator = next;
            sub_start = next_start;
        }

        Ok(Self {
            parts
        })
    }
}

impl TryFrom<String> for Query {
    type Error = QueryParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryInto::<Self>::try_into(value.as_str())
    }
}

impl IntoQuery for String {
    fn parse(&self) -> Result<Query, QueryParseError> {
        IntoQuery::parse(&self.as_str())
    }
}

impl TryFrom<&str> for Query {
    type Error = QueryParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut queue: VecDeque<SubQuery> = VecDeque::new();

        for sub_str in value.split(",") {
            queue.push_front(
                sub_str.try_into()
                    .map_err(|mut err: QueryParseError|{
                        if let QueryParseError::UnexpectedChar(pos, _) = &mut err {
                            *pos += value.find(sub_str).unwrap_or(0)
                        }
                        err
                    })?
            );
        }

        Ok(Self{queue})
    }
}

impl IntoQuery for &str {
    fn parse(&self) -> Result<Query, QueryParseError> {
        (*self).try_into()
    }
}
