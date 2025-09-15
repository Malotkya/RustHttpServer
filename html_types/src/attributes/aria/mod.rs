
pub(crate) mod types;
use types::*;
use super::types::{Value, SpaceSeperatedList, Enumerable};

macro_rules! MakeAriaAttributeList {
    (
        $list_name: ident
        $(, ($key_ident: ident, $key_literal:literal): $type:ty )*
    ) => {
        MakeAriaAttributeList!(
            Combine: $list_name,
            (atomic, "aria-atomic"): Enumerable,
            (busy, "aria-busy"): Enumerable,
            (controls, "aria-controls"): String,
            (current, "aria-current"): Current,
            (described_by, "aria-descripbedby"): SpaceSeperatedList,
            (description, "aria-description"): String,
            (details, "aria-details"): String,
            (disabled, "aria-disabled"): String,
            (drop_effect, "aria-dropeffect"): DropEffect,
            (error_message, "aria-errormessage"): String,
            (flow_to, "aria-flowto"): SpaceSeperatedList,
            (grabbed, "aria-grabbed"): Enumerable,
            (has_popup, "aria-haspopup"): PopUp,
            (hidden, "aria-hidden"): Enumerable,
            (invalid, "aria-invalid"): Enumerable,
            (key_shortcut, "aria-keyshortcut"): String,
            (label, "aria-label"): String,
            (labeled_by, "aria-labedby"): SpaceSeperatedList,
            (live, "aria-live"): Live,
            (owns, "aria-owns"): SpaceSeperatedList,
            (relevant, "aria-relevant"): Relevant,
            (role_description, "aria-roledescription": String),
            $( ($key_ident, $key_literal): $type:ty ),*
        );
    };
    (
        Combine: $list_name:ident,
        $( ($key_ident: ident, $key_literal:literal): $type:ty ),+
    ) => {
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
}

MakeAriaAttributeList!(
    WidgetAttributes,
    (autocomplete, "aria-autocomplete"): AutoComplete,
    (checked, "aria-checked"): Enumerable,
    (expanded, "aria-expanded"): Enumerable,
    (level, "aria-level"): Integer,
    (modal, "aria-modal"): Enumerable,
    (multiline, "aria-multiline"): Enumerable,
    (multiselect, "aria-multiselect"): Enumerable,
    (orientation, "aria-orientation"): Orientation,
    (placeholder, "aria-placeholder"): String,
    (pressed, "aria-pressed"): Pressed,
    (readonly, "aria-readonly"): Enumerable,
    (required, "aria-required"): Enumerable,
    (selected, "aria-selected"): Enumerable,
    (sort, "aria-sort"): Sort,
    (value_max, "aria-valuemax"): Value,
    (value_min, "aria-valuemin"): Value,
    (value_now, "aria-valuenow"): Value,
    (value_text, "aria-valuetext"): String    
);

MakeAriaAttributeList!(
    LiveAttributes,
    //pub busy: Option<Enumerable>, //Global
    (live, "aria-live"): Enumerable
    //pub relevant: Option<Relevant>,
    //pub atomic: Option<Enumerable> //Global
);

MakeAriaAttributeList(
    RelationshipAttributes,
    (active_descendant, "aria-activedescendant"): String,
    (col_count, "aria-colcount"): usize,
    (col_index, "aria-colindex"): usize,
    (col_span, "aria-colspan"): usize,
    //pub controls: Option<String>, //Global
    //pub described_by: Option<SpaceSeperatedList>, //Global
    //pub details: Option<String>, //Global
    //pub error_messsage: Option<String>,
    (flow_to, "aria-flowto"): SpaceSeperatedList,
    (labelled_by, "aria-labelledby"): String,
    //pub owns: Option<SpaceSeperatedList>, //Global
    (pos_inset, "aria-posinset"): usize,
    (row_count, "aria-rowcount"): usize,
    (row_index, "aria-rowindex"): usize,
    (row_span, "aria-rowspan"): usize,
    (set_size, "aria-setsize"): usize
);


