use std::collections::LinkedList;
use crate::component::{
    element::Element,
    node::IntoNode,
    ChildIterator, shift_lifetime
};
use super::*;

enum CombinatorIteratorType<'a> {
    Child(ChildIterator<'a>),
    NextSibling(Option<Element>),
    Descendant(LinkedList<ChildIterator<'a>>)
}

#[derive(Clone)]
pub enum QueryCombinator {
    Child,
    SubsequentSibling,
    NextSibling,
    Descendant
}

impl IntoQuery for QueryCombinator {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: self.clone(),
                name: None,
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

pub(crate) struct CombinatorIterator<'a>(CombinatorIteratorType<'a>);

impl<'a> CombinatorIterator<'a> {
    pub fn new(combinator: &QueryCombinator, start:Element) -> CombinatorIterator<'a> {
        let it_type = match combinator {
            QueryCombinator::Child =>
                CombinatorIteratorType::Child(unsafe{
                    shift_lifetime(start.child_elements())
                }),
            QueryCombinator::Descendant => {
                let mut queue = LinkedList::new();
                queue.push_back(unsafe{
                    shift_lifetime(start.child_elements())
                });
                CombinatorIteratorType::Descendant(queue)
            },
            QueryCombinator::NextSibling =>
                CombinatorIteratorType::NextSibling(start.next_element_sibbling()),
            QueryCombinator::SubsequentSibling => {
                let mut it = unsafe{
                    shift_lifetime(start.sibblings())
                };

                while let Some(next) = it.next() {
                    if next.node().is_same_node(&start) {
                        break;
                    }
                }

                CombinatorIteratorType::Child(it)
            }
        };

        Self(it_type)
    }
}

impl<'a> Iterator for CombinatorIterator<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            CombinatorIteratorType::Child(it) => it.next(),
            CombinatorIteratorType::NextSibling(next) => next.take(),
            CombinatorIteratorType::Descendant(queue) => {
                let mut output: Option<Element> = None;
                
                while let Some(mut it) = queue.pop_front() {
                    if let Some(next) = it.next() {
                        queue.push_front(it);
                        queue.push_back(
                            unsafe{ shift_lifetime(next.child_elements()) }
                        );
                        output = Some(next);
                        break;
                    }
                }

                output
            },
            
            
        }
    }
}

pub(crate) fn find_next_combinator(value:&str) -> Option<(QueryCombinator, usize, usize)> {
    let mut it = value.chars().enumerate();
    let mut start: Option<usize> = None;
    let mut combinator: Option<QueryCombinator> = None;
    while let Some((index, char)) = it.next() {
        match char {
            '+' => if combinator.is_some() {
                break;
            } else {
                combinator = Some(QueryCombinator::NextSibling)
            },
            '>' => if combinator.is_some() {
                break;
            } else {
                combinator = Some(QueryCombinator::Child)
            },
            '~' => if combinator.is_some() {
                break;
            } else {
                combinator = Some(QueryCombinator::SubsequentSibling)
            },
            _ => {
                if char.is_whitespace() {
                    if start.is_none() {
                        start = Some(index)
                    }
                } else {
                    break;
                }
            }
        }
    }

    match start {
        Some(start) => {
            let end = it.next()
                .map(|(index, _)|index)
                .unwrap_or(value.len());

            Some((
                combinator.unwrap_or(QueryCombinator::Descendant),
                start,
                end
            ))
        },
        None => None
    }
}