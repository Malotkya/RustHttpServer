
pub(crate) mod types;
use types::*;
use super::types::{Value, SpaceSeperatedList, Enumerable, Integer};

pub struct WidgetAttributes {
    pub autocomplete: Option<AutoComplete>,
    pub checked: Option<Enumerable>,
    //pub disabled: Option<Enumerable>, Global
    //pub error_message: Option<String>, Global
    pub expanded: Option<Enumerable>,
    //pub has_popup: Option<Enumerable>, Global
    //pub hidden: Option<Enumerable>, Global
    //pub invalid: Option<Enumerable>, Global
    // pub label: Option<String>,
    pub level: Option<usize>,
    pub modal: Option<Enumerable>,
    pub multiline: Option<Enumerable>,
    pub multiselect: Option<Enumerable>,
    pub orientation: Option<Orientation>,
    pub placeholder: Option<String>,
    pub pressed: Option<Pressed>,
    pub readonly: Option<Enumerable>,
    pub required: Option<Enumerable>,
    pub selected: Option<Enumerable>,
    pub sort: Option<Sort>,
    pub value_max: Option<Value>,
    pub value_min: Option<Value>,
    pub value_now: Option<Value>,
    pub value_text: Option<String>
}

pub struct LiveAttributes {
    //pub busy: Option<Enumerable>, Global
    pub live: Option<Enumerable>,
    //pub relevant: Option<Relevant>,
    //pub atomic: Option<Enumerable> Global
}

pub struct RelationshipAttributes {
    pub activedescendat: Option<String>,
    pub col_count: Option<Integer>,
    pub col_index: Option<Integer>,
    pub col_span: Option<Integer>,
    //pub controls: Option<String>, Global
    //pub described_by: Option<SpaceSeperatedList>, Global
    //pub details: Option<String>, Global
    pub error_messsage: Option<String>,
    pub flow_to: Option<SpaceSeperatedList>,
    pub labelled_by: Option<String>,
    //pub owns: Option<SpaceSeperatedList>, Global
    pub posinset: Option<Integer>,
    pub row_count: Option<Integer>,
    pub row_index: Option<Integer>,
    pub row_span: Option<Integer>,
    pub setsize: Option<Integer>
}

pub struct GlobalAttributes {
    pub atomic: Option<Enumerable>,
    pub busy: Option<Enumerable>,
    pub controls: Option<String>,
    pub current: Option<Current>,
    pub described_by: Option<SpaceSeperatedList>,
    pub description: Option<String>,
    pub details: Option<String>,
    pub disabled: Option<Enumerable>,
    pub drop_effect: Option<DropEffect>,
    pub error_message: Option<String>,
    pub flow_to: Option<SpaceSeperatedList>,
    pub grabbed: Option<Enumerable>,
    pub has_popup: Option<PopUp>,
    pub hidden: Option<Enumerable>,
    pub invalid: Option<Enumerable>,
    pub key_shortcut: Option<String>,
    pub label: Option<String>,
    pub labeled_by: Option<SpaceSeperatedList>,
    pub live: Option<Live>,
    pub owns: Option<SpaceSeperatedList>,
    pub relevant: Option<Relevant>,
    pub role_description: Option<String>
}