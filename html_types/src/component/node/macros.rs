macro_rules! DefaultChildrenAccess {
    () => {
        fn children(&self) -> Result<&LinkedList<Node>, NodeError> {
            Ok(&self.children)
        }

        fn children_mut(&mut self) -> Result<&mut LinkedList<Node>, NodeError> {
            Ok(&mut self.children)
        }

        fn add_child(&mut self, child:Node, index: Option<usize>) -> Result<(), NodeError> {
            if let Some(index) = index {
                let mut tail = self.children.split_off(index);
                self.children.push_back(child);
                self.children.append(&mut tail);
            } else {
                self.children.push_back(child);
            }

            Ok(())
        }

        fn set_children(&mut self, list: LinkedList<Node>) -> Result<(), NodeError> {
            self.children = list;
            Ok(())
        }

        fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
            self.children.remove(index);
            Ok(())
        }
    };
}

macro_rules! DefaultAttributeAccess {
    () => {
        fn attributes(&self) -> Option<&Vec<AttributeItem>> {
            Some(&self.attriubutes)
        }

        fn attributes_mut(&mut self) -> Option<&mut Vec<AttributeItem>> {
            Some(&mut self.attriubutes)
        }
    };
}

macro_rules! DefaultParrentAccess {
    () => {
        fn parrent(&self) -> Option<&Node> {
            self.parrent.as_ref()
        }

        fn set_parrent(&mut self, parrent:Option<&Node>) {
            self.parrent = parrent.map(|n|n.node());
        }
    };
}

macro_rules! StaticName {
    () => {
        StaticName!("");
    };
    ($name:literal) => {
        fn name(&self) -> &str {
            $name
        }
    };
}

macro_rules! NodeType {
    (
        $node_type:path = $struct_name:ident(
            $({ $($outer_impl_block:tt)* })?
            $( $outer_trait_name:ident:{ $($outer_impl_trait_block:tt)* }; )*
        );
        $inner_name:ident { $($inner_struct_block:tt)* }:(
            $( { $($inner_impl_block:tt)* }; )?
            $( $inner_trait_name:ident:{ $($inner_impl_trait_block:tt)* }; )*
        )
        
    ) => { paste::paste!{

        pub(crate) struct [<$struct_name $inner_name>] {
            $($inner_struct_block)*
        }

        $(
            impl [<$struct_name $inner_name>] {
                $($inner_impl_block)*
            }
        )?

        $(
            impl $inner_trait_name for [<$struct_name $inner_name>] {
                $($inner_impl_trait_block)*
            }
        )*

        pub struct $struct_name (
            pub(crate) std::rc::Rc<
                std::cell::RefCell<
                    [<$struct_name $inner_name>]
                >
            >
        );

        $(
            impl $struct_name {
                $($outer_impl_block)*
            }
        )?

        $(
            impl $outer_trait_name for $struct_name {
                $($outer_impl_trait_block)*
            }
        )*

        impl IntoNode for $struct_name {
            fn node(&self) -> Node {
                Node($node_type(self.0.clone()))
            }
        }
    }};
}

pub(crate) use DefaultChildrenAccess;
pub(crate) use DefaultParrentAccess;
pub(crate) use DefaultAttributeAccess;
pub(crate) use StaticName;
pub(crate) use NodeType;