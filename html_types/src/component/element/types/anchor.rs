use crate::component::{
    attributes::{
        AddAttributes,
        types::*
    },
    node::Node
};

struct Anchor(Node);

impl Anchor {
    AddAttributes!(
        AriaAttributes(GlobalAttributes);
        GlobalAttributes;
        attribution_src:( "attributionsrc", BoolOrString),
        download:( "download", BoolOrString),
        href:( "href", String),
        href_language:( "hreflang", String),
        ping:( "ping", SpaceSeperatedList),
        referrer_policy:( "referrerpolicy", RefferPolicy),
        rel:( "ref", AnchorRelation),
        target:( "target", LinkTarget),
        types:( "type", String)
    );
}

