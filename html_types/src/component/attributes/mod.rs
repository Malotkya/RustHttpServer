
pub mod aria;
pub mod name;
pub use name::*;
pub mod types;
pub use types::*;
pub mod value;
pub use value::*;

struct AttributeItem(Box<dyn AttributeName>, AttributeValue);

impl AttributeItem {
    pub fn key(&self) -> &str {
        self.0.value()
    }

    pub fn value(&self) -> &AttributeValue {
        &self.1
    }

    pub fn set_value<T: ToString>(&mut self, value:T) {
        self.1 = AttributeValue::String(value.to_string())
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
    (
        GlobalAttributes:
        $list_name: ident
        $( ,($key_ident: ident, $key_literal:literal): $type:ty )*
    ) => {
        $crate::attributes::MakeAttributeList!(
            $list_name,
            (access_key, "accesskey"): String,
            (auto_capitalize, "autocapitalize"): AutoCapitalize,
            (auto_focus, "autofocus"): bool,
            //(auto_correct, "autocorrect") Javascript?
            (class, "class"): SpaceSeperatedList,
            (content_editable, "contenteditable"): ContentEditable,
            (text_direction, "dir"): TextDirection,
            (draggable, "draggable"): Enumerable,
            (enter_key_hint, "enterkeyhint"): KeyHint,
            (export_parts, "exportparts"): SpaceSeperatedList,
            (hidden, "hidden"): Hidden,
            (id, "id"): String,
            (inert, "inert"): bool,
            (input_mode, "inputmode"): InputMode,
            //(Is: "is"),
            (item_id, "itemid"): String,
            (item_prop, "itemprop"): String,
            (item_ref, "itemref"): String,
            (item_scope, "itemscope"): String,
            (item_type, "itemtype"): String,
            (language, "lang"): String,
            (nonce, "nonce"): String,
            (part, "part"): SpaceSeperatedList,
            (popover, "popover"): bool,
            (role, "role"): Role,
            (slot, "slot"): String,
            (spell_check, "spellcheck"): bool,
            (style, "style"): String, /*TODO: Seperate Styling */
            (tab_index, "tabindex"): usize,
            (title, "title"): String,
            (translate, "translate"): types::Translate,
            (writing_suggestions, "writingsuggestions"): bool
            $(, ($key_ident, $key_literal): $type:ty )*
        );
    };
    (
        $list_name:ident,
        $( ($key_ident: ident, $key_literal:literal): $type:ty ),+
    ) => {
        use $crate::component::attributes::*;

        pub struct $list_name {
            $($key_ident: Option<$type>), +
        }

        impl $list_name {
            pub fn new() -> Self {
                Self{
                    $($key_ident: None), +
                }
            }
        }

        impl ToString for $list_name {
            fn to_string(&self) -> String {
                let count:usize = ${count($key_ident)};
                let mut output:String = String::with_capacity(count * 24); //Just a guess 24 = [char; 10] '=' '"' [char; 10] '"' ' '

                $(
                    if self.$key_ident.is_some() {
                        output.push_str(&format_string($key_literal, self.$key_ident.as_ref().unwrap()));
                        output.push(' ');
                    }
                )+

                output
            }
        }
    };
    ($name: ident) => {
        $crate::attributes::AddAttributes!(GlobalAttributes: $name);
    };
}

pub(crate) use AddAttributes;