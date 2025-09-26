 use crate::{
    component::element::Element,
};
use std::{
    collections::vec_deque::{Iter, VecDeque}, 
    iter::Filter,
};

mod combinator;
pub use combinator::*;
mod functions;
mod parts;
pub use parts::*;
mod selector;
pub use selector::Selector;
mod into;
pub use into::*;

pub(crate) trait QueryFilter {
    fn filter(&self, node:&Element) -> bool;
}

impl<T:QueryFilter> QueryFilter for Vec<T> {
    fn filter(&self, node:&Element) -> bool {
        for sel in self {
            if !sel.filter(node) {
                return false;
            }
        }

        true
    }
}

impl<T:QueryFilter> QueryFilter for VecDeque<T> {
    fn filter(&self, node:&Element) -> bool {
        for sel in self {
            if !sel.filter(node) {
                return false;
            }
        }

        true
    }
}

impl<T:QueryFilter> QueryFilter for Option<T> {
    fn filter(&self, node:&Element) -> bool {
        match self.as_ref() {
            Some(t) => t.filter(node),
            None => true
        }
    }
}

impl<T:QueryFilter> QueryFilter for &T {
    fn filter(&self, node:&Element) -> bool {
        (*self).filter(node)
    }
}

#[derive(Clone)]
pub struct QueryParts {
    pub combinator: QueryCombinator,
    pub name: Option<Name>,
    pub id: Option<Id>,
    pub class: Option<Class>,
    pub attributes: Vec<Attribute>,
    pub selectors: Vec<Attribute>
}

type QueryPartsIterator<'a> = Filter<CombinatorIterator<'a>, Box<dyn FnMut(&Element) -> bool + 'a>>;

impl QueryFilter for QueryParts {
    fn filter(&self, node:&Element) -> bool {
        QueryFilter::filter(&self.name, node)
            && QueryFilter::filter(&self.id, node)
            && QueryFilter::filter(&self.class, node)
            && self.attributes.filter(node)
            && self.selectors.filter(node)
    }
}

impl QueryParts {
    pub fn query<'a>(&'a self, element:Element) -> QueryPartsIterator<'a> {
        CombinatorIterator::new(&self.combinator, element)
            .filter(Box::new(|e|self.filter(e)))
    }
}

#[derive(Clone)]
pub struct SubQuery {
    pub parts: Vec<QueryParts>
}

impl SubQuery {
    fn query(&self, element:Element) -> SubQueryIterator {
        SubQueryIterator::new(&self.parts, element)
    }
}

struct SubQueryIterator<'a> {
    parts: &'a Vec<QueryParts>,
    stack: Vec<(usize, QueryPartsIterator<'a>)>
}

impl<'a> SubQueryIterator<'a> {
    fn new(parts:&'a Vec<QueryParts>, element:Element) -> Self{
        let stack = match parts.get(0) {
            Some(part) => vec![(1, part.query(element))],
            None => Vec::new()
        };

        Self {
            parts, stack
        }
    }
}

impl<'a> Iterator for SubQueryIterator<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item>{
        while let Some((index, mut it)) = self.stack.pop() {
            if let Some(element) = it.next() {
                self.stack.push((index, it));

                match self.parts.get(index) {
                    Some(next) => {
                        self.stack.push((
                            index+1,
                            next.query(element)
                        ))
                    },
                    None => {
                        return Some(element)
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct Query {
    queue: VecDeque<SubQuery>
}

impl Query {
    pub fn query(&self, element:Element) -> QueryIterator {
        let mut it = self.queue.iter();
        let current = it.next().map(|sub_query|{
            sub_query.query(element.clone())
        });

        QueryIterator {
            it, current, element
        }
    }
}

pub struct QueryIterator<'a> {
    it: Iter<'a, SubQuery>,
    current: Option<SubQueryIterator<'a>>,
    element: Element
}

impl<'a> Iterator for QueryIterator<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = &mut self.current {
            if let Some(next) = current.next() {
                return Some(next)
            }

            self.current = self.it.next().map(|sub_query|{
                sub_query.query(self.element.clone())
            });
        }

        None
    }
}