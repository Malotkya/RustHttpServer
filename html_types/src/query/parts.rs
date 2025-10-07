use crate::component::{
    attributes::{
        AttributeMatchOperator,
        ToAttributeValue,
        AttributeValue
    },
    element::Element
};
use super::{
    *,
    functions::match_attribute
};

#[derive(Clone)]
pub struct Id(pub String);

impl QueryFilter for Id {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node, 
            "id", 
            Some(MatchOptions { 
                ops: AttributeMatchOperator::WhitespaceValue, 
                value: &self.0 ,
                sensitive: false
            })
        )
    }
}

impl IntoQuery for Id {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: Some(self.clone()),
                class: None,
                attributes: Vec::new(),
                psudo_class: Vec::new(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}

#[derive(Clone)]
pub struct Class(pub String);

impl QueryFilter for Class {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node, 
            "class", 
            Some(MatchOptions { 
                ops: AttributeMatchOperator::WhitespaceValue, 
                value: &self.0,
                sensitive: false
            })
        )
    }
}

impl IntoQuery for Class {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: Some(self.clone()),
                attributes: Vec::new(),
                psudo_class: Vec::new(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}

#[derive(Clone)]
pub struct Name {
    pub namespace: Option<String>,
    pub tag_name: String
}

impl IntoQuery for Name {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: Some(self.clone()),
                id: None,
                class: None,
                attributes: Vec::new(),
                psudo_class: Vec::new(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}

impl QueryFilter for Name {
    fn filter(&self, node:&Element) -> bool {
        if let Some(match_namespace) = self.namespace.as_ref() {

            if let Some(elm_namespace) = node.prefix() {
                if &elm_namespace != match_namespace
                    && match_namespace != "*" {
                    return false
                }
            } else {
                return false;
            }
        }


        self.tag_name == "*" || self.tag_name == node.local_name()
    }
}

#[derive(Clone)]
pub struct MatchOptions<T:ToAttributeValue = AttributeValue> {
    pub ops: AttributeMatchOperator,
    pub value:T,
    pub sensitive: bool
}

impl<T:ToAttributeValue> MatchOptions<T> {
    fn as_ref(&self) -> MatchOptions<&T> {
        MatchOptions {
            ops: self.ops.clone(),
            value: &self.value,
            sensitive: self.sensitive
        }
    }
}

#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub ops: Option<MatchOptions>
}

impl QueryFilter for Attribute {
    fn filter(&self, node:&Element) -> bool {
        match_attribute(
            node,
            &self.name,
            self.ops.as_ref()
                .map(|v|v.as_ref()),
        )
    }
}

impl IntoQuery for Attribute {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: None,
                attributes: vec![self.clone()],
                psudo_class: Vec::new(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}

impl IntoQuery for &[Attribute] {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: None,
                attributes: self.iter()
                    .map(|a|a.clone())
                    .collect(),
                psudo_class: Vec::new(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}