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
                match value {
                    $(
                        $str_lit => Ok(Self::$name),
                    )+
                    _ => Err(
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
    AnyLink: any_link = "active-link"
    /*AutoFill: browser_only,
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