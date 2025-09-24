macro_rules! MakeAttributes {
    (GlobalAttributes) => {
        html_macros::attribute_functions!(
            access_key: ("accesskey", String),
            auto_capitalize: ("autocapitalize", 
                $crate::component::attributes::types::AutoCapitalize),
            auto_focus: ("autofocus", bool),
            //(auto_correct, "autocorrect") Javascript?
            //class: ("class",  
            //    $crate::component::attributes::types::SpaceSeperatedList),
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