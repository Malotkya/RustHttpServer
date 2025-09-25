use std::{
    collections::{
        LinkedList,
        HashMap
    },
    rc::Rc,
    cell::{RefCell, RefMut}
};

use super::{
    attributes::{
        aria::MakeAriaAttributes,
        types::SpaceSeperatedList,
        *
    },
    document::DocumentItemRef,
    node::*,
    other::{DocumentFragment, Text}
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
        if let Some(mut atr) = self.get_attribute_helper(name, namespace) {
           let old = unsafe {
                (*atr.1).set_value(value)
           };
           Some(old)
        } else {
            let atr = self.0.doc.create_attribute(AttributeData{
                namespace: namespace.map(|s|s.to_string()),
                name: AttributeName::Static("class"),
                parrent: Some(Node(self.0.clone())),
                value: value.into_value()
            });

            unsafe {
                if let Some(list) = self.0.borrow_mut().inner_mut() {
                    list.push_back(atr.node());
                }
            }

            None
        }
    }

    fn class(&mut self) -> *mut SpaceSeperatedList {
        if let Some(mut value) = self.get_attribute("class", None) {
            unsafe{ value.0.borrow_mut().coarse_list() as *mut SpaceSeperatedList }
        } else {
            let mut attribute = Attribute(self.doc.create_attribute(AttributeData{
                namespace: None,
                name: AttributeName::Static("class"),
                parrent: Some(Node::new_helper(self)),
                value: AttributeValue::ClassList(SpaceSeperatedList::new())
            }));

            unsafe{
                let ptr = attribute.0.borrow_mut().coarse_list() as *mut SpaceSeperatedList;
                self.borrow_mut().children.push_back(attribute.node());
                ptr
            }
        }
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
    
    //pub fn assigned_slot(&self) -> HtmlSlotElement;

    pub fn attributes(&self) -> HashMap<String, AttributeValue> {
        let inner = self.0.borrow();
        inner.attributes.iter().map(|att|{
            (att.key().to_owned(), att.value().clone())
        }).collect()
    }

    pub fn child_elements(&self) -> Vec<Node> {
        let inner = self.0.borrow();
        inner.children.iter().filter_map(|node| {
            if node.is_visual_element() {
                Some(node.node())
            } else {
                None
            }
        }).collect()
    }

    pub fn child_element_count(&self) -> usize {
        let inner = self.0.borrow();
        
        let mut count:usize = 0;

        for node in &inner.children {
            if node.is_visual_element() {
                count += 1;
            }
        }

        count
    }

    pub fn class_list(&self) -> RefMut<'_, SpaceSeperatedList> {
        let inner = self.0.borrow_mut();
        RefMut::map(inner, |x: &mut ElementData |x.class())
    }

    pub fn class_name(&self) -> RefMut<'_, String> {
        let inner = self.0.borrow_mut();
        RefMut::map(inner, |x: &mut ElementData|x.class().inner())
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

    pub fn first_element_child(&self) -> Option<Node> {
        let inner = self.0.borrow();

        for value in &inner.children {
            if value.is_visual_element() {
                return Some(value.node())
            }
        }

        None
    }

    pub fn last_element_child(&self) -> Option<Node> {
        let inner = self.0.borrow();

        for value in inner.children.iter().rev() {
            if value.is_visual_element() {
                return Some(value.node())
            }
        }

        None
    }

    pub fn local_name(&self) -> String {
        let inner = self.0.borrow();
        let name = inner.name.value();

        if let Some(index) = name.rfind(":") {
            name[index+1..].to_string()
        } else {
            name.to_string()
        }
    }

    pub fn namespace_uri(&self) -> Option<String> {
        None
    }

    pub fn next_element_sibbling(&self) -> Option<Node> {
        let inner = self.0.borrow();

        if let Some(parrent) = inner.parrent() {
            let parrent = parrent.0.borrow();

            if let Ok(list) = parrent.children() {
                let mut it = list.iter();

                while let Some(next) = it.next() {
                    if next.is_same_node(self) {
                        break;
                    }
                }

                while let Some(next) = it.next() {
                    if next.is_visual_element() {
                        return Some(next.node())
                    }
                }
            }
        }

        None
    }

    pub fn outer_html(&self) -> String {
        todo!("Compile html")
    }

    pub fn prefix(&self) -> Option<String> {
        let inner = self.0.borrow();
        let name = inner.name.value();

        if let Some(index) = name.rfind(":") {
            Some(name[..index].to_string())
        } else {
            None
        }
    }

    pub fn previous_element_sibbling(&self) -> Option<Node> {
        let inner = self.0.borrow();

        if let Some(parrent) = inner.parrent() {
            let parrent = parrent.0.borrow();

            if let Ok(list) = parrent.children() {
                let mut prev: Option<Node> = None;
                let mut it = list.iter();

                while let Some(next) = it.next() {
                    if next.is_same_node(self) {
                        return prev;
                    } else if next.is_visual_element() {
                        prev = Some(next.node())
                    }
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

    pub fn tag_name(&self) -> String {
        self.0.borrow().name().to_owned()
    }

    pub fn after(&self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        append_parrent_helper(self, list, true)
    }

    //ToDo: pub fn animate(&mut self, keyframs, options);   

    fn append(&self, list: &[impl IntoNode]) {
        let mut inner = self.0.borrow_mut();
        inner.children.append(&mut list.iter()
            .map(|n|n.node())
            .collect()
        )
    }

    fn before(&self, list: &[impl IntoNode]) -> Result<(), NodeError> {
        append_parrent_helper(self, list, false)
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

    fn closest(&self, selector:&str) -> Option<Node> {
        todo!("Css style selectors implementation")
    }

    //ToDo:pub fn computed_style_map() -> CssStyleMap;

    //ToDo:pub fn get_animations(&self) -> Animations;

    pub fn get_attribute(&self, name:&str) -> Option<AttributeValue> {
        self.get_attribute_helper(name, None).map(|atr|atr.value().clone())
    }

    pub fn get_attribute_names(&self) -> Vec<String> {
        self.0.borrow().attributes.iter().map(|item|{
            item.key().to_string()
        }).collect()
    }

    pub fn get_attribute_node(&self, name:&str) -> Option<Attribute> {
        let interanl = self.0.borrow();
        for att in &interanl.attributes {
            if att.key() == name {
                return Some(
                    Attribute::new(att, Some(self))
                )
            }
        }

        None
    }

    pub fn get_attribute_ns(&self, _namespace:&str, name:&str) -> Option<AttributeValue> {
        self.get_attribute(name)
    }

    //ToDo:pub fn get_bounding_client_rect(&self) -> ??;
    //ToDo:pub fn get_client_rects() -> ??;

    pub fn get_elements_by_class_name(&self, name:&str) -> Vec<Node> {
        self.0.borrow().children.iter().filter_map(|node|{
            let mut inner = node.0.borrow_mut();
            
            if let Some(list) = inner.attributes_mut() {
                for att in list {
                    if att.key() == "class" && att.coarse_list().as_str() == name {
                        return Some(node.node())
                    }
                }

                None
            } else {
                None
            }
        }).collect()
    }

    pub fn get_elements_by_tag_name(&self, name:&str) -> Vec<Node> {
        self.0.borrow().children.iter().filter_map(|node|{
            if node.node_name() == name {
                Some(node.node())
            } else {
                None
            }
        }).collect()
    }

    //ToDo:pub fn get_html(&self, shadow_root:Option<bool>) -> String

    pub fn has_attribute(&self, name:&str) -> bool {
        for att in &self.0.borrow().attributes {
            if att.key() == name {
                return true;
            }
        }

        false
    }

    pub fn has_attributes(&self, list:&[impl ToString]) -> bool {
        for into in list {
            if !self.has_attribute(&into.to_string()) {
                return false;
            }
        }

        true
    }

    //ToDo: pub fn has_pointer_capture(&self, pointer_id) -> bool;

    pub fn insert_adjasent_element(&self, pos:InsertPosition, element:&Element) -> Result<(), NodeError> {
        match pos {
            InsertPosition::BeforeBegin => self.before(&[element]),
            InsertPosition::AfterEnd => self.after(&[element]),
            InsertPosition::BeforeEnd => {
                self.append(&[element]);
                Ok(())
            },
            InsertPosition::AfterBegin => {
                self.prepend(&[element]);
                Ok(())
            }
        }
    }

    pub fn insert_adjasent_html(&self, pos:InsertPosition, html: &str) -> Result<(), NodeError> {
        todo!("Implment html")
    }

    pub fn insert_adjasent_text(&self, pos:InsertPosition, text: &str) -> Result<(), NodeError> {
        match pos {
            InsertPosition::BeforeBegin => self.before(&[Text::new(text)]),
            InsertPosition::AfterEnd => self.after(&[Text::new(text)]),
            InsertPosition::BeforeEnd => {
                self.append(&[Text::new(text)]);
                Ok(())
            },
            InsertPosition::AfterBegin => {
                self.prepend(&[Text::new(text)]);
                Ok(())
            }
        }
    }

    pub fn matches(&self, query:&str) -> bool {
        todo!("Implement css-query string")
    }

    pub fn move_before(&self, node:&impl IntoNode, reference:&impl IntoNode) {
        
    }

    pub fn prepend(&self, list:&[impl IntoNode]) {
        let mut inner = self.0.borrow_mut();
        let mut new_list: LinkedList<Node> = list.iter()
            .map(|n|n.node())
            .collect();

        new_list.append(&mut inner.children);
        inner.children = new_list
    }

    
}





/*
    
    
    
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

fn append_parrent_helper(child:&Element, list: &[impl IntoNode], after:bool) -> Result<(), NodeError> {
    let inner = child.0.borrow_mut(); 

    if let Some(parrent) = inner.parrent() {
        let mut parrent = parrent.0.borrow_mut();
        let children = parrent.children_mut().unwrap();

        let mut index:Option<usize> = None;
        for (i, node) in children.iter().enumerate() {
            if node.is_same_node(child) {
                index = Some(i);
                break;
            }
        }
        let mut index = index.expect("Unable to find self in parrent!");
        if after {
            index += 1;
        }

        let mut tail = children.split_off(index as usize);
        children.append(&mut list.iter()
            .map(|n|n.node())
            .collect()
        );
        children.append(&mut tail);

        Ok(())
    } else {
        Err(NodeError::NoParrent)
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