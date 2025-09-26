use crate::component::{
    attributes::TextDirection,
    element::Element
};
use super::{
    functions::*,
    QueryFilter
};

pub enum Pattern {
    Odd,
    Even,
    Function(String) //An+B
}

macro_rules! BuildSelector {
    (
        $(
            $name:ident
            $( ( $($arg_name:ident : $arg_type:ty),+ ) )?
            :$func_name:path
        ),+
    ) => {
        pub enum Selector{
            $(
                $name
                $( ($($arg_type),+) )?
            ),+
        }

        impl QueryFilter for Selector {
            fn filter(&self, value:&Element) -> bool {
                match self {
                    $(
                        Self::$name$(
                            ( $($arg_name),+ )
                        )? => $func_name(&value,
                            $( $($arg_name),+ )?
                        )
                    ),+
                }
            }
        }
    };
}

BuildSelector!(
    Active: browser_only,
    AnyLink: any_link,
    AutoFill: browser_only,
    Buffering: browser_only,
    Checked: checked,
    Default: default,
    Defined: defined,
    Dir(dir:TextDirection): direction,
    Disabled: disabled,
    Empty: empty,
    FirstChild: first_child,
    FirstOfType: first_of_type,
    Focus: browser_only,
    FocusVisible: browser_only,
    FocusWithin: browser_only,
    FullScreen: browser_only,
    //Has(/*Selector*/String),
    Hover: browser_only
    /*InRange,
    Indeterminate,
    Invalid,
    Is(/*Selector*/String),
    Language(String),
    LastChild,
    LastOfType,
    Link,
    LocalLink,
    Matches(/*Selector*/String),
    Modal,
    Muted,
    Not(/*Selector*/String),
    NthChild(Pattern),
    NthOfType(Pattern),
    NthLastChild(Pattern),
    NthLastOfType(Pattern),
    OnlyChild,
    OnlyOfType,
    Open,
    OutOfRange,
    Past,
    Paused,
    PictureInPicture,
    PlaceholderShown,
    Playing,
    PopoverOpen,
    ReadOnly,
    ReadWrite,
    Required,
    Root,
    Scope,
    Seeking,
    Stalled,
    Target,
    UserInvalid,
    UswerValid,
    Valid,
    Visited,
    VolumeLocked,
    Where(Pattern)*/
);
