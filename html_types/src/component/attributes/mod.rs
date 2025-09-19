
pub mod aria;
pub mod name;
pub use name::*;
pub mod types;
pub use types::*;
pub mod value;
pub use value::*;

#[derive(Clone, PartialEq)]
pub(crate) struct AttributeItem(AttributeName, AttributeValue);

impl AttributeItem {
    pub fn key(&self) -> &str {
        self.0.value()
    }

    pub fn value(&self) -> &AttributeValue {
        &self.1
    }

    pub fn set_value<T: ToAttributeValue>(&mut self, value:T) -> AttributeValue {
        let old_value = self.1.clone();
        self.1 = value.into_value();
        old_value
    }

    pub fn toggle_value(&mut self, value:bool) {
        self.1 = AttributeValue::Boolean(value)
    }
}

impl ToString for AttributeItem {
    fn to_string(&self) -> String {
        match &self.1 {
            AttributeValue::Boolean(b) => if *b {
                self.0.value().to_owned()
            } else {
                String::new()
            },
            AttributeValue::String(value) => {
                let key = self.0.value();

                let mut output = String::with_capacity(key.len() + value.len() + 3);
                output.push_str(key);
                output.push_str("=\"");
                output.push_str(value);
                output.push('"');
                output
            }
        }
    }
}

macro_rules! MakeAttributes {
    (GlobalAttributes) => {
        html_macros::attribute_functions!(
            access_key: ("accesskey", String),
            auto_capitalize: ("autocapitalize", 
                $crate::component::attributes::types::AutoCapitalize),
            auto_focus: ("autofocus", bool),
            //(auto_correct, "autocorrect") Javascript?
            class: ("class",  
                $crate::component::attributes::types::SpaceSeperatedList),
            content_editable: ("contenteditable",  
                $crate::component::attributes::types::ContentEditable),
            text_direction: ("dir",  
                $crate::component::attributes::types::TextDirection),
            draggable: ("draggable",  
                $crate::component::attributes::types::Enumerable),
            enter_key_hint: ("enterkeyhint",  
                $crate::component::attributes::types::KeyHint),
            export_parts: ("exportparts",  
                $crate::component::attributes::types::SpaceSeperatedList),
            hidden: ("hidden",  
                $crate::component::attributes::types::Hidden),
            id: ("id", String),
            inert: ("inert", bool),
            input_mode: ("inputmode",  
                $crate::component::attributes::types::InputMode),
            //(Is: "is"),
            item_id: ("itemid", String),
            item_prop: ("itemprop", String),
            item_ref: ("itemref", String),
            item_scope: ("itemscope", String),
            item_type: ("itemtype", String),
            language: ("lang", String),
            nonce: ("nonce", String),
            part: ("part",  
                $crate::component::attributes::types::SpaceSeperatedList),
            popover: ("popover", bool),
            role: ("role",  
                $crate::component::attributes::types::Role),
            slot: ("slot", String),
            spell_check: ("spellcheck", bool),
            style: ("style", String), /*TODO: Seperate Styling */
            tab_index: ("tabindex", String/*usize*/),
            title: ("title", String),
            translate: ("translate",  
                $crate::component::attributes::types::Translate),
            writing_suggestions: ("writingsuggestions", bool)
        );
    };
    (
        $( $func_name:ident: ($key: literal, $value:ty) ),+
    ) => {

        html_macros::attribute_functions!(
            $( $func_name: ($key, $value) ),+
        );
    };
}

pub(crate) use MakeAttributes;