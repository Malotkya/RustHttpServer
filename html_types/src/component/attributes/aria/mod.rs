pub(crate) mod types;

macro_rules! MakeAriaAttributes {
    (GlobalAttributes) => {
        html_macros::attribute_functions!(
            aria_atomic: ("aria-atomic",
               $crate::component::attributes::types::Enumerable),
            aria_busy: ("aria-busy", 
               $crate::component::attributes::types::Enumerable),
            aria_controls: ("aria-controls", String),
            aria_current: ("aria-current", 
               $crate::component::attributes::types::Current),
            aria_described_by: ("aria-descripbedby", 
               $crate::component::attributes::types::SpaceSeperatedList),
            aria_description: ("aria-description", String),
            aria_details: ("aria-details", String),
            aria_disabled: ("aria-disabled", String),
            aria_drop_effect: ("aria-dropeffect", 
               $crate::component::attributes::types::DropEffect),
            aria_error_message: ("aria-errormessage", String),
            aria_flow_to: ("aria-flowto", 
               $crate::component::attributes::types::SpaceSeperatedList),
            aria_grabbed: ("aria-grabbed", 
               $crate::component::attributes::types::Enumerable),
            aria_has_popup: ("aria-haspopup", 
               $crate::component::attributes::types::PopUp),
            aria_hidden: ("aria-hidden", 
               $crate::component::attributes::types::Enumerable),
            aria_invalid: ("aria-invalid", 
               $crate::component::attributes::types::Enumerable),
            aria_key_shortcut: ("aria-keyshortcut", String),
            aria_label: ("aria-label", String),
            aria_labeled_by: ("aria-labedby", 
               $crate::component::attributes::types::SpaceSeperatedList),
            aria_live: ("aria-live", 
               $crate::component::attributes::types::Live),
            aria_owns: ("aria-owns", 
               $crate::component::attributes::types::SpaceSeperatedList),
            aria_relevant: ("aria-relevant", 
               $crate::component::attributes::types::Relevant),
            aria_role_description: ("aria-roledescription", String)
        );
    };
    (WidgetAttributes) => {
        html_macros::attribute_functions!(
            aria_autocomplete: ("aria-autocomplete", AutoComplete),
            aria_checked: ("aria-checked", Enumerable),
            aria_expanded: ("aria-expanded", Enumerable),
            aria_level: ("aria-level", usize),
            aria_modal: ("aria-modal", Enumerable),
            aria_multiline: ("aria-multiline", Enumerable),
            aria_multiselect: ("aria-multiselect", Enumerable),
            aria_orientation: ("aria-orientation", Orientation),
            aria_placeholder: ("aria-placeholder", String),
            aria_pressed: ("aria-pressed", Pressed),
            aria_readonly: ("aria-readonly", Enumerable),
            aria_required: ("aria-required", Enumerable),
            aria_selected: ("aria-selected", Enumerable),
            aria_sort: ("aria-sort", Sort),
            aria_value_max: ("aria-valuemax", Value),
            aria_value_min: ("aria-valuemin", Value),
            aria_value_now: ("aria-valuenow", Value),
            aria_value_text: ("aria-valuetext", String)    
        );
    };
    (LiveAttributes) => {
        //html_macros::attribute_functions!(
            //aria_atomic: (AriaAtomic, Enumerable),
            //aria_busy: (AriaBusy, Enumerable),
            //aria_live: (AriaLive, Live),
            //aria_relevant: (AriaRelevant, Relevant),
        //);
    };
    (RelationshipAttributes)=>{
        $html_macros::attribute_functions!(
            aria_active_descendant: ("aria-activedescendant", String),
            aria_col_count: ("aria-colcount", usize),
            aria_col_index: ( "aria-colindex", usize),
            aria_col_span: ("aria-colspan", usize),
            //aria_controls: (AriaControls, String),
            //aria_described_by: (AriaDescribedBy, SpaceSeperatedList),
            //aria_details: (AriaDetails, String),
            //aria_error_message: (AriaErrorMessage, String),
            //aria_flow_to: (AriaFlowTo, SpaceSeperatedList),
            aria_labelled_by: ("aria-labelledby", String),
            //aria_owns: (AriaOwns, SpaceSeperatedList),
            aria_pos_inset: ("aria-posinset", usize),
            aria_row_count: ("aria-rowcount", usize),
            aria_row_index: ("aria-rowindex", usize),
            aria_row_span: ("aria-rowspan", usize),
            aria_set_size: ("aria-setsize", usize)
        );
    };
}

pub(crate) use MakeAriaAttributes;
