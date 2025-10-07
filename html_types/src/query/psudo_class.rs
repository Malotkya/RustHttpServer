use crate::component::{
    attributes::TextDirection,
    element::Element
};
use super::{
    functions::*,
    *
};

pub enum Pattern {
    Odd,
    Even,
    Function(String) //An+B
}

macro_rules! BuildPsudoClass {
    (
        $(
            $name:ident
            $( ( $($arg_name:ident : $arg_type:ty),+ ) )?
            :$func_name:path = $str_lit:literal
        ),+
    ) => {

        #[derive(Clone)]
        pub enum PsudoClass{
            $(
                $name
                $( ($($arg_type),+) )?
            ),+
        }

        impl TryFrom<&str> for PsudoClass {
            type Error = String;

            fn try_from(value:&str) -> Result<Self, Self::Error> {
                
                if value.is_empty() {
                    return Err("An empty string cannot be a PsudoClass!".to_string())
                }
                $(else if value == $str_lit {
                    Ok(
                        Self::$name
                        $(
                            ($( <$arg_type>::default() ),+)
                        )?
                    )
                }$( else if let Some(index) = value.find($str_lit) && index == 0{
                    let start = $str_lit.len();
                    let end = value.len() - 1;

                    let mut chars = value.chars();

                    let open = chars.nth(start);
                    if open.is_none() || open.unwrap() != '(' {
                        return Err(format!("Missing open bracket at {} for PsudoClass {}!", start, $str_lit));
                    }
                    
                    let close = chars.nth(end);
                    if close.is_none() || close.unwrap() != ')' {
                        return Err(format!("Missing closing bracket at {} for PsudoClass {}!", end, $str_lit));
                    }

                    let mut args = value[start..=end].split(",");

                    $(
                        let $arg_name = <$arg_type>::from(args.next().unwrap_or("").trim());
                    )+

                    Ok(
                        Self::$name(
                            $($arg_name),+
                        )
                    )
                })?
                )+
                else {
                    Err(
                        format!("{} is not a valid PsudoClass!", value)
                    )
                }
            }
        }

        impl QueryFilter for PsudoClass {
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

BuildPsudoClass!(
    Active: browser_only = "active",
    ActiveViewTransition: browser_only = "active-view-transition",
    AnyLink: any_link = "active-link",
    AutoFill: browser_only = "autofill",
    Blank: blank = "blank",
    Buffering: browser_only = "buffering",
    Checked: checked = "checked",
    Current: browser_only = "current",
    Default: default = "default",
    Defined: defined = "defined",
    Dir(dir:TextDirection): direction = "dir"
    /*Disabled: disabled,
    Empty: empty,
    FirstChild: first_child,
    FirstOfType: first_of_type,
    Focus: browser_only,
    FocusVisible: browser_only,
    FocusWithin: browser_only,
    FullScreen: browser_only,
    //Has(/*Selector*/String),
    Hover: browser_only
    InRange,
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

impl IntoQuery for PsudoClass {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: None,
                attributes: Vec::new(),
                psudo_class: vec![self.clone()],
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}

impl IntoQuery for &[PsudoClass] {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: None,
                attributes: Vec::new(),
                psudo_class: self.iter()
                    .map(|class|class.clone())
                    .collect(),
                psudo_element: None
            }]
        });
        Ok(Query{queue})
    }
}