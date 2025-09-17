use super::AttributeEnum;

AttributeEnum!(
    Enumerable,
    Boolean
);

AttributeEnum!(
    RefferPolicy,
    (No, "no-referrer"),
    (NoWhenDowngrade, "no_referrer-when-downgrade"),
    (Origin, "origin"),
    (OriginWhenCross, "origin-when-cross-origin"),
    (Unsafe, "unsafe-url")
);

AttributeEnum!(
    CrossOrigin,
    (Anonymous, "anonymous"),
    (UseCredentials, "use-credentials")
);

AttributeEnum!(
    Priority,
    (High, "high"),
    (Low, "low"),
    (Auto, "auto")
);

AttributeEnum!(
    LinkTarget,
    (_Self, "_self"),
    (Blank, "_blank"),
    (Parent, "_parrent"),
    (Top, "_top")
);

AttributeEnum!(
    AutoCapitalize,
    (None, "none"),
    (On, "on"),
    (Off, "off"),
    (Sentences, "sentences"),
    (Characters, "characters")
);

AttributeEnum!(
    ContentEditable,
    (PlaintTextOnly, "plaintext-only"),
    Boolean
);

AttributeEnum!(
    TextDirection,
    (LeftToRight, "ltr"),
    (RightToLeft, "rtl"),
    (Auto, "auto")
);

AttributeEnum!(
    KeyHint,
    (Enter, "enter"),
    (Done, "done"),
    (Go, "go"),
    (Next, "next"),
    (Previous, "previous"),
    (Search, "search"),
    (Send, "send")
);

AttributeEnum!(
    Hidden,
    (UntilFound, "until-found"),
    Boolean
);

AttributeEnum!(
    InputMode,
    (None, "none"),
    (Text, "text"),
    (Decimal, "decimal"),
    (Numeric, "numberic"),
    (Telephone, "tel"),
    (Search, "search"),
    (Email, "email"),
    (Url, "url")
);

AttributeEnum!(
    Role,
    (Toolbar, "toolbar"),
    (Feed, "feed"),
    (Math, "math"),
    (Presentation, "presentation"),
    (Note, "note"),
    (Scrollbar, "scrollbar"),
    (SearchBox, "searchbox"),
    (Separator, "separator"),
    (Slider, "slider"),
    (SpinButton, "spinbutton"),
    (Switch, "switch"),
    (Tab, "tab"),
    (TabPanel, "tabpanel"),
    (TreeItem, "treeitem"),
    (ComboBox, "combobox"),
    (Menu, "menu"),
    (Menubar, "menubar"),
    (Tablist, "tablist"),
    (Tree, "tree"),
    (TreeGrid, "treegrid")
);

AttributeEnum!(
    Translate,
    (Yes, "yes"),
    (No, "no")
);

impl From<bool> for Translate {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

AttributeEnum!(
    AnchorRelation,
    (Alternamte, "alternate"),
    (Author, "author"),
    (Bookmark, "bookmark"),
    (External, "eternal"),
    (Help, "help"),
    (Liscense, "liscense"),
    (Me, "me"),
    (Next, "next"),
    (NoFollow, "nofollow"),
    (NoOpener, "noopener"),
    (NoReferrer, "norefferer"),
    (Opener, "opener"),
    (Prev, "prev"),
    (PrivacyPolicy, "privacy-policy"),
    (Search, "search"),
    (Tag, "tag"),
    (TermsOfService, "terms-of-service")
);