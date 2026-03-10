use std::{
    string::ToString,
    iter::Iterator,
    slice::{Iter, IterMut}
};
use crate::{
    attributes::{Attribute, AttributeValue},
    node::Node
};
pub use html_macros::create_element;

macro_rules! all_element_function{
    ( mut: $( $func:item )+ ) => {
        impl Element {
            $( $func )+
        }

        impl<'a> ElementMutRef<'a> {
            $( $func )+
        }
    };
    ( $( $func:item )+ ) => {
        impl Element {
            $( $func )+
        }

        impl<'a> ElementRef<'a> {
           $( $func )+
        }

        impl<'a> ElementMutRef<'a> {
            $( $func )+
        }
    };
}

pub struct Element(pub(crate) Node);
pub struct ElementRef<'a>(&'a Node);
pub struct ElementMutRef<'a>(&'a mut Node);

impl Element {
    pub fn new(name:&str, attributes:Vec<Attribute>, children:Vec<Element>) -> Self {
        Self(Node::Element(
            name.to_string(),
            false,
            children.into_iter()
            .map(|child|child.0).chain(
                attributes.into_iter()
                    .map(|att|att.into())
            ).collect()
        ))
    }

    pub fn new_void(name:&str, attributes:Vec<Attribute>) -> Self {
        Self(Node::Element(
            name.to_string(),
            true,
            attributes.into_iter()
                .map(|att|att.into())
                .collect()
        ))
    }
}

all_element_function!(
    pub fn name(&self) -> &str {
        return self.0.name();
    }

    pub fn get_attribute(&self, name:&str) -> Option<&AttributeValue> {
        for child in self.0.children() {
            if let Node::Attribute(child_name, value) = child {
                if name == child_name {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn get_inner_text(&self) -> String {
        self.0.get_text()
            .collect::<Vec<&str>>()
            .join("")
    }

    pub fn get_content(&self) -> String {
        self.0.get_content()
            .collect::<Vec<&str>>()
            .join("")
    }

    pub fn children(&self) -> ChildIter<'_> {
        ChildIter(self.0.children())
    }

    pub fn stringify(&self) -> String {
        self.0.to_string()
    }
);

all_element_function!( mut:
    pub fn append_child<E:Into<Element>>(&mut self, child:E) {
        self.0.append(child.into().0);
    }

    pub fn remove_child<'b, R:Into<ElementRef<'b>>>(&mut self, child:R) {
        self.0.remove(child.into().0);
    }

    pub fn clear_children(&mut self) {
        self.0.clear();
    } 

    pub fn get_attribute_mut(&mut self, name:&str) -> Option<&mut AttributeValue> {
        for child in self.0.children_mut() {
            if let Node::Attribute(child_name, value) = child {
                if name == child_name {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn set_attribute<A:Into<Attribute>>(&mut self, value:A) {
        let Attribute{name, value} = value.into();

        if let Some(old_value) = self.get_attribute_mut(&name) {
            *old_value = value; 
        } else {
            self.0.append(Attribute{name, value});
        }
    }

    pub fn remove_attribute(&mut self, name:&str) {
        self.0.remove(&Attribute::new(name, false).into());
    }

    pub fn set_custom_attribute<V:Into<AttributeValue>>(&mut self, name:&str, value:V) {
        if let Some(old_value) = self.get_attribute_mut(name) {
            *old_value = value.into();
        } else {
            self.0.append(Attribute{name: String::from(name), value: value.into()})
        }
    }

    pub fn toggle_attribute(&mut self, name:&str, force:Option<bool>) {
        if let Some(old_value) = self.get_attribute_mut(name) {
            *old_value = AttributeValue::Boolean(
                force.unwrap_or(!old_value.is_truthy())
            )
        } else {
            self.0.append(Attribute{
                name: String::from(name),
                value: AttributeValue::Boolean(force.unwrap_or(true))
            })
        }
    }

    pub fn set_inner_text<S:ToString>(&mut self, value:S) {
        self.0.set_text(value);
    }

    pub fn set_content<S:ToString>(&mut self, value:S) {
        self.0.set_content(value);
    }

    pub fn children_mut(&mut self) -> ChildIterMut<'_>{
        ChildIterMut(self.0.children_mut())
    }
);

pub struct ChildIter<'a>(Iter<'a, Node>);

impl<'a> Iterator for ChildIter<'a> {
    type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.0.next() {
            if node.is_element() {
                return Some(ElementRef(node))
            }
        }

        None
    }
}

pub struct ChildIterMut<'a>(IterMut<'a, Node>);

impl<'a> Iterator for ChildIterMut<'a> {
    type Item = ElementMutRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.0.next() {
            if node.is_element() {
                return Some(ElementMutRef(node))
            }
        }

        None
    }
}

impl<T:ToString> From<T> for Element {
    fn from(value: T) -> Self {
        Self(Node::Text(value.to_string()))
    }
}

impl<'a> Into<ElementRef<'a>> for &'a Element {
    fn into(self) -> ElementRef<'a> {
        ElementRef(&self.0)
    }
}

impl<'a> Into<ElementMutRef<'a>> for &'a mut Element {
    fn into(self) -> ElementMutRef<'a> {
        ElementMutRef(&mut self.0)
    }
}

impl<'a> Into<ElementRef<'a>> for ElementMutRef<'a> {
    fn into(self) -> ElementRef<'a> {
        ElementRef(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1:&'static str = "<ul class=\"float-list\" id=\"targetList\"><li class=\"float-item\">List Item 1</li><li class=\"float-item\">List Item 2</li></ul>";
    const TEST_2:&'static str = "<ul class=\"floatList\" id=\"targetList\"><li class=\"float-item\">List Item 1</li><li class=\"float-item\">List Item 2</li><li>List Item 3</li>Not a List Item!</ul>";
    const TEST_3:&'static str = "<ul id=\"targetList\"><li class=\"float-item\">List Item 2</li><li>List Item 3</li>Not a List Item!</ul>";
    const TEST_4:&'static str = "<ul id=\"targetList\"><input name=\"username\" id=\"username\" /></ul>";

    #[test]
    fn basic_element() {
        let li = Element::new("li", vec![Attribute::new("class", "float-item")], vec!["List Item 1".into()]);
        let li_ptr: *const Element = &li;

        let mut e = Element::new(
            "ul",
            vec![
                Attribute::new("class", "float-list"),
                Attribute::new("id", "targetList")
            ],
            vec![
                li,
                Element::new("li", vec![Attribute::new("class", "float-item")], vec!["List Item 2".into()])
            ]
        );

        assert_eq!(
            e.stringify().as_str(),
            TEST_1
        );

        e.set_custom_attribute("class", "floatList");
        e.append_child(Element::new("li", Vec::new(), vec!["List Item 3".into()]));
        e.append_child("Not a List Item!");
        
        assert_eq!(
            e.stringify().as_str(),
            TEST_2
        );

        e.remove_attribute("class");
        e.remove_child(unsafe{ &*li_ptr });

        assert_eq!(
            e.stringify().as_str(),
            TEST_3
        );

        e.clear_children();
        e.append_child(Element::new_void("input", vec![
            Attribute::new("name", "username"),
            Attribute::new("id", "username")
        ]));

        assert_eq!(
            e.stringify().as_str(),
            TEST_4
        )

    }
}

#[macro_export]
macro_rules! create_element {
    ("area" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "area",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("base" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "base",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("br" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "br",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("col" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "col",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("embed" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "embed",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("hr" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "hr",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("img" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "img",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("input" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "input",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("link" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "link",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("meta" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "meta",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("source" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "source",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("track" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "track",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    ("wbr" , { $($att_name:literal: $att_value:expr),* } ) => {
        Element::new_void(
            "wbr",
            $crate::create_attributes!( $($att_name: $att_value),* )
        )
    };
    (
        $name:literal,
        { $($att_name:literal: $att_value:expr),* }
        (, $child:expr )*
    ) => {
        Element::new(
            $name,
            $crate::create_attributes!($($att_name: $att_value),* )
            vec![ $($child),* ]
        )
    };
}



