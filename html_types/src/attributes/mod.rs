
mod aria;
pub mod types;
pub use types::*;

pub(crate) fn format_string(key: &str, value: &impl ToString) -> String {
    let value = value.to_string();

    let mut output = String::with_capacity(key.len() + value.len() + 3);

    output.push_str(key);
    output.push_str("=\"");
    output.push_str(value.as_str());
    output.push('"');

    output
}

macro_rules! MakeAttributeList {
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
            (class, "class"): SpaceSeperatedList,
            (content_editable, "contenteditable"): ContentEditable,
            (text_direction, "dir"): TextDirection,
            (draggable, "draggable"): Enumerable,
            (enter_keyhint, "enterkeyhint"): KeyHint,
            (export_parts, "exportparts"): SpaceSeperatedList,
            (hidden, "hidden"): Hidden,
            (id, "id"): String,
            (insert, "inert"): bool,
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
            (translate, "translate"): types::Translate
            $(, ($key_ident, $key_literal): $type:ty )*
        );
    };
    (
        $list_name:ident,
        $( ($key_ident: ident, $key_literal:literal): $type:ty ),+
    ) => {
        use $crate::attributes::*;

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
        $crate::attributes::MakeAttributeList!(GlobalAttributes: $name);
    };
}

pub(crate) use MakeAttributeList;