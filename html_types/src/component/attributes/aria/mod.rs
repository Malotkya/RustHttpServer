pub(crate) mod name;
pub(crate) mod types;

macro_rules! MakeAriaAttributes {
    () => {
        $crate::component::attributes::aria::MakeAriaAttributes(GlobalAttributes);
    };
    ($($name:ident),+ ) => {
        $(
            $crate::component::attributes::aria::MakeAriaAttributes($name);
        )+
    };
    (GlobalAttributes) => {
        $crate::component::attributes::AddAttributes(
            aria_atomic: (AriaAtomic, Enumerable),
            aria_busy: (AriaBusy, Enumerable),
            aria_controls: (AriaControls, String),
            aria_current: (AriaCurrent, Current),
            aria_described_by: (AriaDescribedBy, SpaceSeperatedList),
            aria_description: (AriaDescription, String),
            aria_details: (AriaDetails, String),
            aria_disabled: (AriaDisabled, String),
            aria_drop_effect: (AriaDropEffect, DropEffect),
            aria_error_message: (AriaErrorMessage, String),
            aria_flow_to: (AriaFlowTo, SpaceSeperatedList),
            aria_grabbed: (AriaGrabbed, Enumerable),
            aria_has_popup: (AriaHasPopup, PopUp),
            aria_hidden: (AriaHidden, Enumerable),
            aria_invalid: (AriaInvalid, Enumerable),
            aria_key_shortcut: (AriaKeyShortcut, String),
            aria_label: (AriaLabel, String),
            aria_labeled_by: (AriaLabeledBy, SpaceSeperatedList),
            aria_live: (AriaLive, Live),
            aria_owns: (AriaOwns, SpaceSeperatedList),
            aria_relevant: (AriaRelevant, Relevant),
            aria_role_description: (AriaRoleDescription, String)
        );
    };
    (WidgetAttributes) => {
        $crate::component::attributes::AddAttributes(
            aria_autocomplete: (AriaAutoComplete, AutoComplete),
            aria_checked: (AriaChecked, Enumerable),
            aria_expanded: (AriaExpanded, Enumerable),
            aria_level: (AriaLevel, usize),
            aria_modal: (AriaModal, Enumerable),
            aria_multiline: (AriaMultiline, Enumerable),
            aria_multiselect: (AriaMultiselect, Enumerable),
            aria_orientation: (AriaOrientation, Orientation),
            aria_placeholder: (AriaPlaceholder, String),
            aria_pressed: (AriaPressed, Pressed),
            aria_readonly: (AriaReadonly, Enumerable),
            aria_required: (AriaRequired, Enumerable),
            aria_selected: (AriaSelected, Enumerable),
            aria_sort: (AriaSort, Sort),
            aria_value_max: (AriaValueMax, Value),
            aria_value_min: (AriaValueMin, Value),
            aria_value_now: (AriaValueNow, Value),
            aria_value_text: (AriaValueText, String)    
        );
    };
    (LiveAttributes) => {
        //$crate::component::attributes::AddAttributes(
            //aria_atomic: (AriaAtomic, Enumerable),
            //aria_busy: (AriaBusy, Enumerable),
            //aria_live: (AriaLive, Live),
            //aria_relevant: (AriaRelevant, Relevant),
        //);
    };
    (RelationshipAttributes)=>{
        $crate::component::attributes::AddAttributes(
            aria_active_descendant: (AriaActiveDescendant, String),
            aria_col_count: (AriaColCount, usize),
            aria_col_index: (AriaColIndex, usize),
            aria_col_span: (AriaColSpan, usize),
            //aria_controls: (AriaControls, String),
            //aria_described_by: (AriaDescribedBy, SpaceSeperatedList),
            //aria_details: (AriaDetails, String),
            //aria_error_message: (AriaErrorMessage, String),
            aria_flow_to: (AriaFlowTo, SpaceSeperatedList),
            aria_labelled_by: (AriaLabelledBy, String),
            //aria_owns: (AriaOwns, SpaceSeperatedList),
            aria_pos_inset: (AriaPosInset, usize),
            aria_row_count: (AriaRowCount, usize),
            aria_row_index: (AriaRowIndex, usize),
            aria_row_span: (AriaRowSpan, usize),
            aria_set_size: (AriaSetSize, usize)
        );
    };
}

pub(crate) use MakeAriaAttributes;
