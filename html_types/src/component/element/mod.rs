use std::{
    collections::{
        LinkedList,
        HashMap
    },
    rc::Rc,
    cell::{RefCell, RefMut}
};
use super::{
    //node::*,
    attributes::{
        aria::MakeAriaAttributes,
        types::SpaceSeperatedList,
        AttributeValue,
        MakeAttributes,
        AttributeItem,
        AttributeName
    },
    node::*
};

//mod types;
//mod macros;
//pub(crate) use macros::BuildHtmlElement;

pub struct ElementData {
    name: AttributeName,
    attributes: Vec<AttributeItem>,
    parrent: Option<Node>,
    children: LinkedList<Node>
}

impl ElementData {
    fn class(&mut self) -> &mut SpaceSeperatedList {
        let mut pos:i64 = -1;
        for (index, att) in self.attributes.iter().enumerate() {
            if att.key() == "class" {
                pos = index as i64;
                break;
            }
        }

        if pos < 0 {
            pos = self.attributes.len() as i64;
            self.attributes.push(AttributeItem(
                AttributeName::Static("class"),
                AttributeValue::ClassList(SpaceSeperatedList::new())
            ));
        }

        self.attributes.iter_mut().nth(pos as usize).unwrap().coarse_list()
    }
}

impl PartialEq for ElementData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.attributes == other.attributes
            && self.children == other.children
    }
}

impl NodeInternalData for ElementData {
    DefaultParrentAccess!();

    fn name(&self) -> &str {
        self.name.value()
    }

    fn is_void(&self) -> bool {
        match self.name.value() {
            "area" | "base" | "br" | "col" |
            "embed" | "hr" | "img" | "input" | 
            "link" | "meta" | "param" | "source" | 
            "track" | "wbr" => true,
            _ => false
        }
    }

    fn children(&self) -> Result<&LinkedList<Node>, NodeError> {
        if self.is_void() {
            Err(NodeError::NoChildrenList)
        } else {
            Ok(&self.children)
        }
    }

    fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError> {
        if self.is_void() {
            Err(NodeError::NoChildrenList)
        } else {
            if let Some(index) = index {
                let mut tail = self.children.split_off(index);
                self.children.push_back(child);
                self.children.append(&mut tail);
            } else {
                self.children.push_back(child);
            }

            Ok(())
        }
    }

    fn set_children(&mut self, list: LinkedList<Node>) -> Result<(), NodeError> {
        if self.is_void() {
            Err(NodeError::NoChildrenList)
        } else {
            self.children = list;
            Ok(())
        }
    }
}

pub struct Element(pub(crate) Rc<RefCell<ElementData>>);

impl IntoNode for Element {
    fn node(&self) -> Node {
        Node::Element(self.0.clone())
    }
}

///// https://developer.mozilla.org/en-US/docs/Web/API/Element /////
impl Element {
    pub(crate) fn static_new(name: &'static str, attributes:Vec<AttributeItem>) -> Self {
        Self(
            Rc::new(RefCell::new(
                ElementData {
                    name: AttributeName::Static(name),
                    attributes,
                    children: LinkedList::new(),
                    parrent: None
                }
            ))
        )
    }

    MakeAriaAttributes!(GlobalAttributes);
    MakeAttributes!(GlobalAttributes);
    
    //fn assigned_slot(&self) -> HtmlSlotElement;

    fn attributes(&self) -> HashMap<String, AttributeValue> {
        let inner = self.0.borrow();
        inner.attributes.iter().map(|att|{
            (att.key().to_owned(), att.value().clone())
        }).collect()
    }

    fn child_elements(&self) -> Vec<Node> {
        let inner = self.0.borrow();
        inner.children.iter().filter_map(|node| {
            if node.is_visual_element() {
                Some(node.node())
            } else {
                None
            }
        }).collect()
    }

    fn child_element_count(&self) -> usize {
        let inner = self.0.borrow();
        
        let mut count:usize = 0;

        for node in &inner.children {
            if node.is_visual_element() {
                count += 1;
            }
        }

        count
    }

    fn class_list(&self) -> RefMut<'_, SpaceSeperatedList> {
        let inner = self.0.borrow_mut();
        RefMut::map(inner, |x: &mut ElementData |x.class())
    }

    fn class_name(&self) -> RefMut<'_, String> {
        let inner = self.0.borrow_mut();
        RefMut::map(inner, |x: &mut ElementData|x.class().inner())
    }

    //fn client_hight(&self) -> uszie;
    //fn client_left(&self) -> usize;
    //fn client_top(&self) -> usize;
    //fn client_width(&self) -> usize;
    //fn current_css_zoom(&self) -> f64;

    fn first_element_child(&self) -> Option<Node> {
        let inner = self.0.borrow();

        for value in &inner.children {
            if value.is_visual_element() {
                return Some(value.node())
            }
        }

        None
    }

    fn last_element_child(&self) -> Option<Node> {
        let inner = self.0.borrow();

        for value in inner.children.iter().rev() {
            if value.is_visual_element() {
                return Some(value.node())
            }
        }

        None
    }
}

/*
fn class_name(&self) -> String;
    
    fn local_name(&self) -> String;
    fn namespace_uri(&self) -> Option<String>;
    fn next_element_sibbling(&self) -> Option<Node>;
    fn outer_html(&self) -> String
    fn get_part(&self) -> Part;
    fn set_part(&mut self, value:Part);
    fn prefix(&self) -> String;
    fn previous_element_sibbling(&self);
    fn scroll_height(&self) -> usize;
    fn scroll_left(&self) -> usize;
    fn scroll_top(&self) -> usize;
    fn scroll_width(&self) -> usize;
    fn get_slot(&self) -> Slot;
    fn set_slot(&mut self, value:Slot);
    fn tag_name(&self) -> String;
    fn after(&mut self, list: &[Node]);
    fn animate(&mut self, keyframs, options);
    fn append(&mut self, list: &[Node]);
    fn before(&mut self, list: &[Node]);
    fn check_visibility(&self, options) -> bool;
    fn closest(&self) -> Option<Node>;
    fn get_animations(&self) -> Animations;
    pub fn get_attribute(&self, name:&str) -> Option<$crate::component::AttributeValue> {
            let interanl = self.0.borrow();
            for att in &interanl.attributes {
                if att.key() == name {
                    return Some(att.value().clone())
                }
            }

            None
        }
    fn get_attribute_names(&self) -> Vec<String>;
    fn get_attribute_node(&self, name:&str) -> Option<Node>;
    fn get_attribute_ns(&self, namespace:&str, name:&str) -> Option<Attribute>;
    fn get_bounding_client_rect(&self) -> ??;
    fn get_client_rects() -> ??;
    fn get_elements_by_class_name(&self, name:&str) -> LinkedList<Node>;
    fn get_elements_by_tag_name(&self, name:&str) -> LinkedList<Node>;
    fn get_html(&self) -> String;
    fn has_attribute(&self, name:&str) -> bool;
    fn has_attributes(&self, list:&[impl ToString]) -> bool;
    fn insert_adjasent_element(&mut self, pos:ENUM, element: Node) -> Result<(), ERROR>;
    fn insert_adjasent_html(&mut self, pos:ENUM, html: &str) -> Result<(), ERROR>;
    fn insert_adjasent_text(&mut self, pos:ENUM, text: &str) -> Result<(), ERROR>;
    fn matches(&self, query:&str) -> bool;
    fn move_before(&mut self, reference:&Node);
    fn prepend(&mut self, node:&Node);
    fn query_selector(&self, query:String) -> Option<Node>;
    fn query_selector_all(&self, query:String) -> LinkedList<Node>;
    fn remove(&mut self);
    fn replace_children(&mut self, list: &[&Node]);
    fn replace_with(&mut self, list:&[&Node]);
    fn request_full_screen(&self);
    fn request_pointer_lock(&self);
    fn scroll(&self);
    fn scroll_by(&self);
    fn scroll_into_view(&self);
    fn scroll_into_view_if_needed(&self);
    fn scroll_to(&self);
    pub fn set_attribute<T:$crate::component::attributes::ToAttributeValue>(&mut self, name:&str, value:T) -> Option<$crate::component::AttributeValue> {
            let mut interanl = self.0.borrow_mut();
            
            for att in &mut interanl.attributes {
                if att.key() ==  name {
                    return Some(
                        att.set_value(value)
                    )
                }
            }

            None
        }
    fn set_attribute_ns(&mut self, namespace:&str, name:&str, value:ToAttribute) -> Option<Attribute>;
    fn set_pointer_capture(&self);
    pub fn toggle_attribute(&mut self, name:&str, value:Option<bool>) -> Option<$crate::component::AttributeValue> {
            let value = !value.unwrap_or(
                self.get_attribute(name)
                    .map(|v|v.is_truthy())
                    .unwrap_or(false)
            );

            let mut interanl = self.0.borrow_mut();
            for att in &mut interanl.attributes {
                if att.key() ==  name {
                    let old_value = att.value().clone();
                    att.toggle_value(value);
                    return Some(
                        old_value
                    )
                }
            }

            None
        }
 */

pub trait HtmlElement {
    //https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement
    fn get_inner_text(&self) -> String;
    fn set_inner_text(&mut self, value:&str);
    fn get_outer_text(&self) -> String;
    fn offset_height(&self) -> usize;
    fn offset_left(&self) -> usize;
    fn offset_parrent(&self) -> usize;
    fn offset_top(&self) -> usize;
    fn offset_width(&self) -> usize;

    fn attach_internals(&self);
    fn blur(&self);
    fn click(&self);
    fn focus(&self);
    fn hide_popover(&self);
    fn show_popover(&self);
    fn toggle_popover(&self);
}