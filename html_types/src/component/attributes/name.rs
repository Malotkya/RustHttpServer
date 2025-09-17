
pub use super::aria::name::*;

pub trait AttributeName {
    fn value(&self) -> &str;
}

struct CustomAttributeName(pub(crate) String);

impl AttributeName for CustomAttributeName {
    fn value(&self) -> &str {
        &self.0
    }
}

macro_rules! AttributeNames {
    (
        $parrent:ident:
        $( ($struct:ident, $value:literal) ),+
    ) => { paste::paste! {
        $(
            pub struct [<$parent $struct>];

            impl crate::component::attributes::AttributeName 
            for [<$parent $struct>] {
                fn value(&self) -> &str {
                    $value
                }
            }
        )+
    }};
    ( $( ($struct:ident, $value:literal) ),+ ) => {
        $(
            pub struct $struct;

            impl crate::component::attributes::AttributeName for $struct {
                fn value(&self) -> &str {
                    $value
                }
            }
        )+
    };
}
pub(crate) use AttributeNames;

AttributeNames!(
    (AccessKey, "accesskey"),
    (AutoCapitalize, "autocapitalize"),
    (AutoFocus, "autofocus"),
    //(auto_correct, "autocorrect")?
    (Class, "class"),
    (ContentEditable, "contenteditable"),
    (TextDirection, "dir"),
    (Draggable, "draggable"),
    (EnterKeyHint, "enterkeyhint"),
    (ExportParts, "exportparts"),
    (Hidden, "hidden"),
    (Id, "id"),
    (Inert, "inert"),
    (InputMode, "inputmode"),
    //(Is: "is"),
    (ItemId, "itemid"),
    (ItemProp, "itemprop"),
    (ItemRef, "itemref"),
    (ItemScope, "itemscope"),
    (TtemType, "itemtype"),
    (Language, "lang"),
    (Nonce, "nonce"),
    (Part, "part"),
    (Popover, "popover"),
    (Role, "role"),
    (Slot, "slot"),
    (SpellCheck, "spellcheck"),
    (Style, "style"), /*TODO: Seperate Styling */
    (TabIndex, "tabindex"),
    (Title, "title"),
    (Translate, "translate"),
    (WritingSuggestions, "writingsuggestions")
);