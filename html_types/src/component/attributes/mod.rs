
pub mod aria;
pub(crate) use aria::MakeAriaAttributes;
pub mod name;
pub use name::*;
pub mod types;
pub use types::*;
pub mod value;
pub use value::*;

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

macro_rules! AddAttributes {
    () => {
        $crate::component::attributes::AddAttributes!(
            AriaAttributes(GlobalAttributes);
            GlobalAttributes;
        );
    };
    (
        $( AriaAttributes($( $aria_name:ident ),+); )?
        GlobalAttributes;
        $( $func_name:ident: ($key: literal, $value:ty) ),*
    ) => {
        $(
            $crate::component::attributes::MakeAriaAttributes!($($aria_name),+);
        )?
        //$crate::component::attributes::AddAttributes!(
        html_macros::attribute_functions!(
            $( $func_name: ($key, $value), )*
            access_key: ("accesskey", String),
            auto_capitalize: ("autocapitalize", AutoCapitalize),
            auto_focus: ("autofocus", bool),
            //(auto_correct, "autocorrect") Javascript?
            class: ("class", SpaceSeperatedList),
            content_editable: ("contenteditable", ContentEditable),
            text_direction: ("dir", TextDirection),
            draggable: ("draggable", Enumerable),
            enter_key_hint: ("enterkeyhint", KeyHint),
            export_parts: ("exportparts", SpaceSeperatedList),
            hidden: ("hidden", Hidden),
            id: ("id", String),
            inert: ("inert", bool),
            input_mode: ("inputmode", InputMode),
            //(Is: "is"),
            item_id: ("itemid", String),
            item_prop: ("itemprop", String),
            item_ref: ("itemref", String),
            item_scope: ("itemscope", String),
            item_type: ("itemtype", String),
            language: ("lang", String),
            nonce: ("nonce", String),
            part: ("part", SpaceSeperatedList),
            popover: ("popover", bool),
            role: ("role", Role),
            slot: ("slot", String),
            spell_check: ("spellcheck", bool),
            style: ("style", String), /*TODO: Seperate Styling */
            tab_index: ("tabindex", String/*usize*/),
            title: ("title", String),
            translate: ("translate", Translate),
            writing_suggestions: ("writingsuggestions", bool)
        );
    };
    (
        $( $func_name:ident: ($key: literal, $value:ty) ),+
    ) => {

        html_macros::attribute_functions!(
            $( $func_name: ($key, $value) ),+
        );

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

        pub fn get_attribute(&self, name:&str) -> Option<$crate::component::AttributeValue> {
            let interanl = self.0.borrow();
            for att in &interanl.attributes {
                if att.key() == name {
                    return Some(att.value().clone())
                }
            }

            None
        }
    };
}

pub(crate) use AddAttributes;