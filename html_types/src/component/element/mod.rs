use std::{
    collections::{
        LinkedList,
        HashMap,
        HashSet
    }
};

use crate::{
    component::ChildIterator, 
    query::{
        IntoQuery, QueryParseError,
        Id as QueryId,
        Name as QueryName,
        Class as QueryClass
    }
};

use super::{
    attributes::{
        aria::MakeAriaAttributes,
        types::SpaceSeperatedList,
        *
    },
    document::DocumentItemRef,
    node::*,
    other::DocumentFragment
};

mod internal;
pub use internal::*;
//mod types;
//mod macros;
//pub(crate) use macros::BuildHtmlElement;


pub struct Element(pub(crate) DocumentItemRef);

impl Element {
    fn get_attribute_helper(&self, name: &str, namespace:Option<&str>) -> Option<Attribute> {
        self.0.inner().map(|children|{
            for node in children {
                if let Ok(atr) = TryInto::<Attribute>::try_into(node) {
                    if atr.0.namespace() == namespace && atr.0.local_name() == name {
                        return Some(atr)
                    }
                }
            }

            None
        }).flatten()
    }

    fn remove_attribute_helper(&mut self, name:&str, namespace:Option<&str>) {
        if let Some(children) = unsafe{ self.0.borrow_mut() }.inner_mut() {
             let mut pos:Option<usize> = None;

            for (index, node ) in children.iter_mut().enumerate() {
                if let Ok(atr) = TryInto::<Attribute>::try_into(&*node) {
                    if atr.0.namespace() == namespace && atr.0.local_name() == name {
                        pos = Some(index);
                        break;
                    }
                }
            }

            if let Some(index) = pos {
                let mut tail = children.split_off(index);
                tail.pop_front();
                children.append(&mut tail);
            }
        }
    }

    fn toggle_attribute_helper(&mut self, name:&str, namespace:Option<&str>, value:bool) {
        if value {
            self.set_attribute_helper(name, namespace, true);
        } else {
            self.remove_attribute_helper(name, namespace);
        }
    }

    fn set_attribute_helper<T: ToAttributeValue>(&mut self, name:&str, namespace:Option<&str>, value:T) -> Option<AttributeValue>{
        if let Some(atr) = self.get_attribute_helper(name, namespace) {
           let old = unsafe {
                (*atr.1).set_value(value)
           };
           Some(old)
        } else {
            let mut atr = self.0.doc.create_attribute(
                "class",
                namespace,
                value
            );

            unsafe {
                atr.0.borrow_mut().set_parrent(Some(&self.node()));

                if let Some(list) = self.0.borrow_mut().inner_mut() {
                    list.push_back(atr.node());
                }
            }

            None
        }
    }

    fn class(&mut self) -> *mut SpaceSeperatedList {
        if let Some(value) = self.get_attribute_helper("class", None) {
            unsafe{ (*value.1).coarse_list() as *mut SpaceSeperatedList }
        } else {
            let mut atr = self.0.doc.create_attribute(
                "class",
                None,
                SpaceSeperatedList::new()
            );

            unsafe{
                let ptr = (*atr.1).coarse_list() as *mut SpaceSeperatedList;
                atr.0.borrow_mut().set_parrent(Some(&self.node()));

                if let Some(list) = self.0.borrow_mut().inner_mut() {
                    list.push_back(atr.node());
                }

                ptr
            }
        }
    }

    pub(crate) fn sibblings<'a>(&'a self) -> ChildIterator<'a> {
        if let Some(parrent) = self.0.parrent() {
            parrent.child_nodes().into()
        } else {
            ChildIterator::empty()
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl IntoNode for Element {
    fn node(&self) -> Node {
        Node(
            self.0.clone()
        )
    }
}

fn perform_try_clone(value:&Node, inc:bool) -> Result<Element, &'static str> {
    if !value.is_visual_element() {
        Err("Unable to convert to Element!")
    } else {
        if inc {
            value.0.item.inc();
        }

        Ok(Element(value.0.clone()))
    }
}  

impl TryFrom<Node> for Element {
    type Error = &'static str;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        perform_try_clone(&value, false)
    }
}

impl TryFrom<&Node> for Element {
    type Error = &'static str;

    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        perform_try_clone(value, true)
    }
}

///// https://developer.mozilla.org/en-US/docs/Web/API/Element /////
impl Element {
    MakeAriaAttributes!(GlobalAttributes);
    MakeAttributes!(GlobalAttributes);
    
    //pub fn assigned_slot(&self) -> HtmlSlotElement;

    pub fn attributes(&self) -> HashMap<String, AttributeValue> {
        self.0.attributes().map(|att|{
            (att.name().to_owned(), att.value().clone())
        }).collect()
    }

    pub fn child_elements<'a>(&'a self) -> ChildIterator<'a> {
        match self.0.inner() {
            Some(list) => ChildIterator::new(list.iter()),
            None => ChildIterator::empty()
        }
    }

    pub fn child_element_count(&self) -> usize {
        self.child_elements().count()
    }

    pub fn class_list(&mut self) -> &mut SpaceSeperatedList {
        unsafe{ &mut (*self.class()) }
    }

    pub fn class_name(&mut self) -> &str {
       unsafe{ (*self.class()).as_str() }
    }

    pub fn set_class_name(&mut self, value:&str) {
        unsafe{ (*self.class()) = value.into() }
    }

    pub fn client_hight(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }
    
    pub fn client_left(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }

    pub fn client_top(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }

    pub fn client_width(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }

    pub fn current_css_zoom(&self) -> f64 {
        0.0 //No actual rendering or calculating will be done
    }

    pub fn first_element_child(&self) -> Option<Element> {
        self.0.children().next()
    }

    pub fn last_element_child(&self) -> Option<Element> {
        self.0.children().last()
    }

    pub fn local_name(&self) -> &str {
        self.0.local_name()
    }

    /*pub fn namespace_uri(&self) -> Option<&str> {
        self.0.namespace()
    }*/

    pub fn next_element_sibbling(&self) -> Option<Element> {
        if let Some(parrent) = self.0.parrent() {
            let mut it = parrent.0.children();

            while let Some(next) = it.next() {
                if next.node().is_same_node(&self.node()) {
                    return it.next()
                }
            }
        }

        None
    }

    pub fn outer_html(&self) -> String {
        todo!("Compile html")
    }

    pub fn prefix(&self) -> Option<&str> {
        self.0.namespace()
    }

    pub fn previous_element_sibbling(&self) -> Option<Element> {
        if let Some(parrent) = self.0.parrent() {
            let mut prev:Option<Element> = None;
            let mut it = parrent.0.children();

            while let Some(next) = it.next() {
                if next.node().is_same_node(&self.node()) {
                    return prev
                } else {
                    prev = Some(next.into());
                }
            }
        }

        None
    }

    pub fn scroll_height(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }

    pub fn scroll_left(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }
    
    pub fn scroll_top(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }
    
    pub fn scroll_width(&self) -> usize {
        0 //No actual rendering or calculating will be done
    }
    
    pub fn shadow_root(&self) -> Option<DocumentFragment> {
        None //TODO: Return whole thing as shadow root ???
    }

    pub fn tag_name(&self) -> &str {
        self.0.local_name()
    }

    pub fn after(&mut self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.node().parrent() {
            append_parrent_helper(&mut parrent, list, true, Some(self))
        } else {
            Err(NodeError::NoParrent)
        }
        
    }

    //ToDo: pub fn animate(&mut self, keyframs, options);   

    fn append(&self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        append_parrent_helper(&mut self.node(), list, false, None)
    }

    fn before(&mut self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.node().parrent() {
            append_parrent_helper(&mut parrent, list, false, Some(self))
        } else {
            Err(NodeError::NoParrent)
        }
    }

     pub fn check_visibility(&self, _options:Option<VisibilityOptions>) -> bool {
        false //TODO: May Implement further at a later time.

        //Returns False When:
        // Css-Display for self or parrent is set to None or Contents.
        // Content-Visibility for self or parrent is set to hidden
        // Optional Checks:
        // If Content-Visibility is Auto and being skipped (default false)
        // If Opacity is set to 0 (default false)
        // If visibility is hidden or collapse and hidden (default false)
    }

    fn closest<T: IntoQuery>(&self, selector:&T) -> Result<Option<Element>, QueryParseError> {
        let inner_match = self.query_selector(selector)?;

        if inner_match.is_some() {
            Ok(inner_match)
        } else if let Some(node) = self.0.parrent() && let Ok(parrent) = Element::try_from(node) {
            parrent.closest(selector)
        } else {
            Ok(None)
        }
    }

    //ToDo:pub fn computed_style_map() -> CssStyleMap;

    //ToDo:pub fn get_animations(&self) -> Animations;

    pub fn get_attribute(&self, name:&str) -> Option<AttributeValue> {
        self.get_attribute_helper(name, None).map(|atr|atr.value().clone())
    }

    pub fn get_attribute_names(&self) -> impl Iterator<Item = String> {
        self.0.attributes().map(|atr|atr.name().to_string())
    }

    pub fn get_attribute_ns(&self, namespace:&str, name:&str) -> Option<AttributeValue> {
        self.get_attribute_helper(name, Some(namespace)).map(|atr|atr.value().clone())
    }

    //ToDo:pub fn get_bounding_client_rect(&self) -> ??;
    //ToDo:pub fn get_client_rects() -> ??;

    pub fn get_elements_by_class_name(&self, name:&str) -> Vec<Element> {
        self.query_selector_all(&QueryClass(
            name.to_string()
        )).unwrap()
    }

    pub fn get_elements_by_tag_name(&self, name:&str) -> Vec<Element> {
        self.query_selector_all(&QueryName{
            namespace: None,
            tag_name: name.to_string()
        }).unwrap()
    }

    //ToDo:pub fn get_html(&self, shadow_root:Option<bool>) -> String

    pub fn has_attribute(&self, name:&str) -> bool {
        for att in self.0.borrow().attributes() {
            if att.name() == name {
                return true;
            }
        }

        false
    }

    pub fn has_attributes(&self, list:&[impl ToString]) -> bool {
        let list: HashSet<String> = list.iter().map(|s|s.to_string()).collect();

        for att in self.0.borrow().attributes() {
            if !list.contains(att.name()) {
                return false
            }
        }

        true
    }

    //ToDo: pub fn has_pointer_capture(&self, pointer_id) -> bool;

    pub fn insert_adjasent_element(&mut self, pos:InsertPosition, element:&Element) -> Result<(), NodeError> {
        match pos {
            InsertPosition::BeforeBegin => self.before(&[element]),
            InsertPosition::AfterEnd => self.after(&[element]),
            InsertPosition::BeforeEnd => {
                self.append(&[element])
            },
            InsertPosition::AfterBegin => {
                self.prepend(&[element])
            }
        }
    }

    pub fn insert_adjasent_html(&mut self, pos:InsertPosition, html: &str) -> Result<(), NodeError> {
        todo!("Implment html")
    }

    pub fn insert_adjasent_text(&mut self, pos:InsertPosition, text: &str) -> Result<(), NodeError> {
        let text = self.0.doc.create_text_node(text);

        match pos {
            InsertPosition::BeforeBegin => self.before(&[text]),
            InsertPosition::AfterEnd => self.after(&[text]),
            InsertPosition::BeforeEnd => {
                self.append(&[text])
            },
            InsertPosition::AfterBegin => {
                self.prepend(&[text])
            }
        }
    }

    pub fn matches<T: IntoQuery>(&self, query:&T) -> bool {
        match query.parse() {
            Ok(query) => query.matches(self),
            Err(_) => false
        }
    }

    pub fn move_before(&self, node:&impl IntoNode, reference:&impl IntoNode) -> Result<(), NodeError> {
        append_parrent_helper(&mut node.node(), &[reference], false, Some(self))
    }

    pub fn prepend(&self, list:&[impl IntoNode]) -> Result<(), NodeError> {
        append_parrent_helper(&mut self.node(), list, false, None)
    }

    pub fn query_selector<T: IntoQuery>(&self, query:&T) -> Result<Option<Element>, QueryParseError> {
        Ok(
            query.parse()?
                .query(self.clone())
                .next()
        )
    }

    fn query_selector_all<T: IntoQuery>(&self, query:&T) -> Result<Vec<Element>, QueryParseError> {
        Ok(
            query.parse()?
                .query(self.clone())
                .collect()
        )
    }
}





/*
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
    pub fn set_attribute<T:ToAttributeValue>(&mut self, name:&str, value:T) -> Option<AttributeValue> {
        self.0.set_attribute(name, None, value)
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

fn append_parrent_helper(parrent:&mut Node, list: &[impl IntoNode], after:bool, child:Option<&Element>) -> Result<(), NodeError> {
    let mut index:Option<usize>;
    
    if let Some(child) = child{
        index = None;

        for (i, node) in parrent.0.children().enumerate() {
            if node.node().is_same_node(child) {
                if after {
                    index = Some(i+1);
                } else {
                    index = Some(i);
                }
                break;
            }
        }
    } else {

        if after {
            index = Some(parrent.0.children().len())
        } else {
            index = Some(0)
        }
    }
        
    
    
    if let Some(index) = index {
        unsafe {
            parrent.0.borrow_mut().add_children(
                &list.iter()
                    .map(|n|n.node())
                    .collect::<Vec<Node>>(),
                Some(index)
            )?
        }

        Ok(())
    } else {
        Err(NodeError::InvalidChild)
    }
}

pub struct VisibilityOptions {
    content_visibility_auto: Option<bool>,
    opacity_property: Option<bool>,
    visibility_property: Option<bool>
}

pub enum InsertPosition {
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd
}

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