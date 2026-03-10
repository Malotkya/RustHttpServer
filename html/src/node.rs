use std::{
    fmt,
    string::{String, ToString},
    str::SplitWhitespace,
    slice::{Iter, IterMut},
    iter::Iterator
};
use crate::attributes::AttributeValue;

#[derive(Eq)]
pub(crate) enum NodeData {
    Comment(String),
    Text(String),
    Element(String, bool, Vec<NodeData>),
    Attribute(String, AttributeValue)
}



impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Attribute(lhs, _) => if let Self::Attribute(rhs, _) = other {
                lhs == rhs
            } else {
                false
            },
            Self::Text(lhs) => if let Self::Text(rhs) = other {
                lhs == rhs
            } else {
                false
            },
            _ => std::ptr::eq(self, other)
        }
    }
}

impl NodeData {
    pub fn name(&self) -> &str {
        match self {
            Self::Comment(_) => "!",
            Self::Text(_) => "",
            Self::Element(name, _, _) => name,
            Self::Attribute(name, _) => name
        }
    }

    pub fn is_element(&self) -> bool {
        match self {
            Self::Element(_, _, _) => true,
            _ => false
        }
    }

    pub fn is_attribute(&self) -> bool {
        match self {
            Self::Attribute(_, _) => true,
            _ => false
        }
    }

    pub fn append<N:Into<NodeData>>(&mut self, value:N) {
        let value:NodeData = value.into();

        match self {
            Self::Comment(text) => text.push_str(
                &value.get_text()
                    .collect::<Vec<&str>>()
                    .join("")
            ),
            Self::Text(text) => text.push_str(
                &value.get_text()
                    .collect::<Vec<&str>>()
                    .join("")
            ),
            Self::Attribute(_, _) => {},
            Self::Element(_, _, list) => {
                list.push(value)
            }
        }
    }

    pub fn remove(&mut self, node:&NodeData) {
        if let NodeData::Element(_, _, children) = self {
            children.retain(|n|n.ne(node))
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Text(str) => str.clear(),
            Self::Comment(str) => str.clear(),
            Self::Attribute(_, value) => value.clear(),
            Self::Element(_, _, children) => children.retain(|node|node.is_attribute())
        }
    }

    pub fn children(&self) -> Iter<'_, NodeData> {
        match self{
            Self::Element(_, _, list) => list.iter(),
            _ => Default::default()
        }
    }

    pub fn children_mut(&mut self) -> IterMut<'_, NodeData> {
        match self {
            Self::Element(_, _, list) => list.iter_mut(),
            _ => Default::default()
        }
    }

    pub fn get_text(&self) -> TextIter<'_> {
        match self {
            Self::Comment(text) => TextIter::Single(text.split_whitespace(), Some("")),
            Self::Text(text) => TextIter::Single(text.split_whitespace(), Some("")),
            Self::Attribute(_, _) => TextIter::None,
            Self::Element(_, is_void, children) => if *is_void {
                TextIter::None
            } else {
                TextIter::Multiple(children.iter(), None, None)
            }
        }
    }

    pub fn set_text<S:ToString>(&mut self, value:S) {
        match self {
            Self::Comment(text) => *text = value.to_string(),
            Self::Text(text) => *text = value.to_string(),
            Self::Attribute(_, _) => {},
            Self::Element(_, is_void, children) => {
                if *is_void {
                    return;
                }

                children.retain(|node|node.is_attribute());
                children.push(NodeData::Text(value.to_string()));
            }
        }
    }

    pub fn get_content(&self) -> ContentIter<'_> {
        match self {
            Self::Comment(str) => ContentIter::Single(Some(str)),
            Self::Text(str) => ContentIter::Single(Some(str)),
            Self::Attribute(_, _) => ContentIter::None,
            Self::Element(_, is_void, children) => if *is_void {
                ContentIter::None
            } else {
                ContentIter::List(children.iter(), None)
            }
        }
    }

    pub fn set_content<S:ToString>(&mut self, value:S) {
        self.set_text(value);
    }

}

pub(crate) enum TextIter<'a> {
    None,
    Single(SplitWhitespace<'a>, Option<&'a str>),
    Multiple(Iter<'a, NodeData>, Option<SplitWhitespace<'a>>, Option<&'a str>)
}

fn next_helper<'a, I:Iterator<Item = &'a str>>(it:&mut I, prev: &mut Option<&'a str>) -> Option<&'a str> {
    while let Some(next) = it.next() {
        if next.len() > 0 {
            *prev = Some(next);
            return Some(next);
        }

        if prev.is_some() {
            *prev = None;
            return Some(" ");
        }
    }

    None
}

impl<'a> Iterator for TextIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::Single(it, prev) => next_helper(it, prev),
            Self::Multiple(it, opt, prev) => {
                if let Some(it) = opt {
                    let next = next_helper(it, prev);

                    if next.is_some() {
                        return next;
                    }
                }

                while let Some(next_node) = it.next() {
                    if let NodeData::Text(str) = next_node {
                        *opt = Some(str.split_whitespace());
                        *prev = Some(" ");
                        return self.next();
                    }
                }

                None
            }
        }
    }
}

pub(crate) enum ContentIter<'a> {
    None,
    Single(Option<&'a str>),
    List(Iter<'a, NodeData>, Option<Box<ContentIter<'a>>>)
}

impl<'a> Iterator for ContentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::Single(opt) => opt.take(),
            Self::List(it, curr_opt) => {
                if let Some(curr_node) = curr_opt {
                    let next = curr_node.next();

                    if next.is_some() {
                        return next;
                    }
                }

                if let Some(next_node) = it.next() {
                    *curr_opt = Some(Box::new(next_node.get_content()));
                }

                None
            }
        }
    }

}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attribute(name, value) => write!(f, "{}=\"{}\"", name, value),
            Self::Text(text) => write!(f, "{}", text),
            Self::Comment(text) => write!(f, "<!--{}-->", text),
            Self::Element(name, is_void, list) => {
                let mut attributes: Vec<String> = Vec::with_capacity(list.len());
                let mut children: Vec<String> = Vec::with_capacity(list.len());

                for node in list {
                    if node.is_attribute() {
                        attributes.push(node.to_string());
                    } else {
                        children.push(node.to_string());
                    }
                }

                if *is_void {
                    write!(f, "<{}", name)?;
                    if attributes.len() > 0 {
                        write!(f, " {}", attributes.join(" "))?;
                    }
                    write!(f, "/>")
                    
                } else {
                    write!(f, "<{}", name)?;
                    if attributes.len() > 0 {
                        write!(f, " {}>{}", attributes.join(" "), children.join(""))?;
                    } else {
                        write!(f, ">{}", children.join(""))?;
                    }
                    write!(f, "</{}>", name)
                }
            }
        }
    }
}

impl<T:Into<AttributeValue>> From<T> for NodeData {
    fn from(value: T) -> Self {
        NodeData::Text(value.into().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_node_string() {
        let string = "Hello World!";
        let n:Node = string.into();

        assert_eq!(
            n.to_string().as_str(),
            string
        )
    }

    #[test]
    fn text_node_integer() {
        let n:Node = 525600.into();

        assert_eq!(
            n.to_string().as_str(),
            "525600"
        )
    }

    #[test]
    fn text_node_number() {
        let n:Node = 420.69.into();

        assert_eq!(
            n.to_string().as_str(),
            "420.69"
        )
    }

    #[test]
    fn text_node_boolean() {
        let n:Node = false.into();

        assert_eq!(
            n.to_string().as_str(),
            "false"
        )
    }
    
    #[test]
    fn comment_node() {
        let n = Node::Comment("Hello World".to_string());

        assert_eq!(
            n.to_string().as_str(),
            "<!--Hello World-->"
        )
    }

    #[test]
    fn void_element_node() {
        let n = Node::Element("br".to_string(), true, Vec::new());

        assert_eq!(
            n.to_string().as_str(),
            "<br/>"
        )
    }

    #[test]
    fn element_node() {
        let n = Node::Element(
            "p".to_string(),
            false,
            vec![
                Node::Comment("This is a message element.".to_string()),
                Node::Attribute("class".to_string(), "message".into()),
                Node::Text("Hello World!".to_string()),
                Node::Element("br".to_string(), true, Vec::new()),
                Node::Attribute("style".to_string(), "color: red;".into())
            ]
        );

        assert_eq!(
            n.to_string().as_str(),
            "<p class=\"message\" style=\"color: red;\"><!--This is a message element.-->Hello World!<br/></p>"
        )
    }
}