use crate::component::{
    attributes::{TextDirection, ToAttributeValue},
    element::Element,
    node::IntoNode,
};
use super::parts::MatchOptions;


pub(crate) fn match_attribute<T:ToAttributeValue>(node:&Element, name:&str, opts:Option<MatchOptions<T>>) -> bool {
    if let Some(att) = node.get_attribute(name) {
        match opts {
            None => true,
            Some(MatchOptions{ops, value, sensitive}) => {
                if sensitive {
                    att.compare(ops, &value)
                } else {
                    att.compare_insensitive(ops, &value)
                }
            }
        }
    } else {
        false
    }
}

pub(crate) fn browser_only(_:&Element) -> bool {
    false
}

pub(crate) fn any_link(node:&Element) -> bool {
    match node.tag_name() {
        "a" | "area" => {
            node.has_attribute("href")
        }
        _ => false
    }
}

pub(crate) fn checked(node:&Element) -> bool {
    match node.tag_name() {
        "input" => {
            if let Some(value) = node.get_attribute("type")
                && (value == "radio" || value == "checkbox") {

                if let Some(value) = node.get_attribute("checked"){
                    value.is_truthy()
                } else {
                    false
                }
            } else {
                false
            }
        },
        "option" => {
            todo!("Implement closest")
        },
        _ => false
    }
}

pub(crate) fn default(node:&Element) -> bool {
    match node.tag_name() {
        "option" => if let Some(value) = node.get_attribute("selected") {
            value.is_list()
        } else {
            false
        },
        "input" => if let Some(input_type) = node.get_attribute("type") {
            match input_type.as_str() {
                "checkbox" | "radio" => if let Some(checked) = node.get_attribute("checked"){
                    checked.is_truthy()
                } else {
                    false
                },
                "button" | "submit" | "image" => {
                    todo!("Detect if first of type in form")
                },
                _ => false
            }
        } else {
            false
        },
        "button" => {
            todo!("Detect if first of type in form")
        },
        _ => false
    }
}

pub(crate) fn defined(node:&Element) -> bool {
    todo!("Create list of custom elements and html elements")
}

pub(crate) fn direction(_node:&Element, dir:&TextDirection) -> bool {
    *dir == TextDirection::LeftToRight
    //ToDo: Use UserAgent from Reqeust
}

pub(crate) fn disabled(node:&Element) -> bool {
    if let Some(disabled) = node.get_attribute("disabled") {
        disabled.is_truthy()
    } else {
        false
    }
}

pub(crate) fn empty(node:&Element) -> bool {
    node.child_element_count() == 0
}

pub(crate) fn enabled(node:&Element) -> bool {
    if let Some(disabled) = node.get_attribute("disabled") {
        !disabled.is_truthy()
    } else {
        true
    }
}

pub(crate) fn first_child(node:&Element) -> bool {
    if let Some(parrent) = node.node().parrent() {
        if let Some(child) = parrent.first_child() {
            child.is_same_node(node)
        } else {
            false
        }
    } else {
        false
    }
}

pub(crate) fn first_of_type(node:&Element) -> bool {
    if let Some(parrent) = node.node().parrent() {
        for child in parrent.child_nodes() {
            if let Ok(child) = TryInto::<Element>::try_into(child) {
                if child.tag_name() == node.tag_name() {
                    return child.node().is_same_node(node)
                }
            }
        }
    }

    false
}