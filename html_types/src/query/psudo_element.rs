use super::*;
use crate::{component::element::Element, query::QueryCombinator};

macro_rules! BuildPsudoElement {
    (
        $(
            $name:ident = $str_lit:literal
        ),+
    ) => {
        
        #[derive(Clone)]
        pub enum PsudoElement{
            $(
                $name
            ),+
        }

        impl TryFrom<&str> for PsudoElement {
            type Error = String;

            fn try_from(value:&str) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $str_lit => Ok(Self::$name),
                    )+
                    _ => Err(
                        format!("{} is not a valid PsudoElement!", value)
                    )
                }
            }
        }

        impl QueryFilter for PsudoElement {
            fn filter(&self, _:&Element) -> bool {
                false
            }
        }
    };
}

BuildPsudoElement!(
    After = "after",
    BackDrop = "backdrop",
    Before = "before",
    Column = "column",
    Checkmark = "checkmark",
    Cue = "cue",
    DetailsContent = "details-content",
    FileSelectorButton = "file-selector-button",
    FirstLetter = "first-letter",
    FirstLine = "first-line",
    GrammarError = "grammar-error",
    Highlight = "highlight()",
    Marker = "marker",
    Part = "part()",
    Picker = "picker()",
    PickerIcon = "picker-icon",
    Placeholder = "placeholder",
    ScrollButton = "scroll-button()",
    ScrollMarker = "scroll-marker",
    ScrollMarkerGroup = "scroll-marker-group",
    Selection = "selection",
    Slotted = "slotted()",
    SpellingError = "spelling-error",
    TargetText = "target-text",
    ViewTransition = "view-transition",
    ViewTransitionImagePair = "view-transition-image-pair()",
    ViewTransitionGroup = "view-transition-group",
    ViewTransitionNew = "view-transition-new",
    ViewTransitionOld = "view-transition-old"
);

impl IntoQuery for PsudoElement {
    fn parse_query(&self) -> Result<Query, QueryParseError> {
        let mut queue = VecDeque::new();
        queue.push_front(SubQuery {
            parts: vec![QueryParts {
                combinator: QueryCombinator::Descendant,
                name: None,
                id: None,
                class: None,
                attributes: Vec::new(),
                psudo_class: Vec::new(),
                psudo_element: Some(self.clone())
            }]
        });
        Ok(Query{queue})
    }
}