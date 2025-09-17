use crate::component::attributes::AttributeEnum;

AttributeEnum!(
    AutoComplete,
    (None, "none"),
    (Inline, "inline"),
    (List, "list"),
    (Both, "both")
);


AttributeEnum!(
    PopUp,
    (Menu, "menu"),
    (Listbox, "listbox"),
    (Tree, "tree"),
    (Grid, "grid"),
    (Dialog, "dialog"),
    Boolean
);

AttributeEnum!(
    Orientation,
    (Horizontal, "horizontal"),
    (Vertical, "vertical")
);

AttributeEnum!(
    Pressed,
    (Mixed, "mixed"),
    Boolean
);

AttributeEnum!(
    Sort,
    (Ascending, "ascending"),
    (Descending, "descending"),
    (Other, "other"),
    (None, "none")
);

AttributeEnum!(
    Live,
    (Assertive, "assertive"),
    (Polite, "polite"),
    (Off, "off")
);

AttributeEnum!(
    Relevant,
    (Additions, "additions"),
    (All, "all"),
    (Removals, "removals"),
    (Text, "text")
);

AttributeEnum!(
    DropEffect,
    (Copy, "copy"),
    (Execute, "execute"),
    (Link, "link"),
    (Move, "move"),
    (Popup, "popup")
);

AttributeEnum!(
    Current,
    (Page, "page"),
    (Step, "step"),
    (Location, "location"),
    (Date, "date"),
    (Time, "time"),
    Boolean
);
