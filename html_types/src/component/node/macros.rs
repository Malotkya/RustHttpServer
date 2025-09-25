macro_rules! DefaultChildrenAccess {
    () => {
        fn children(&self) -> $crate::component::ChildIterator {
            $crate::component::ChildIterator::new(self.children.iter())
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

        fn set_children(&mut self, list: &[impl IntoNode]) -> Result<(), NodeError> {
            let mut new_list = LinkedList::new();

            while let Some(next) = self.children.pop_front() {
                if !next.is_visual_element() {
                    new_list.push_back(next);
                }
            }

            for new in list {
                new_list.push_back(new.node());
            }
            
            self.children = new_list;

            Ok(())
        }

        fn remove_child(&mut self, index:usize) -> Result<(), NodeError> {
            let mut tail = self.children.split_off(index);
            tail.pop_back();
            self.children.append(&mut tail);
            Ok(())
        }

        fn attributes(&self) -> $crate::component::AttributeIterator {
            $crate::component::AttributeIterator::new(self.children.iter())
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
        fn local_name(&self) -> &str {
            $name
        }
    };
}

macro_rules! NodeType {
    (
        $node_type:path = $struct_name:ident(
            $({ $($outer_impl_block:tt)* };)?
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
            pub(crate) $crate::component::document::DocumentItemRef<
                [<$struct_name $inner_name>]
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
                Node(self.0.downgrade())
            }
        }

        impl TryFrom<Node> for $struct_name {
            type Error = &'static str;

            fn try_from(value: Node) -> Result<Self, Self::Error> {
                TryInto::<Self>::try_into(&value)
            }
        }

        impl TryFrom<&Node> for $struct_name {
            type Error = &'static str;

            fn try_from(value: &Node) -> Result<Self, Self::Error> {
                match value.0.deref() {
                    $node_type(inner) => {
                        value.0.item.inc();

                        Ok(
                            Self(
                                $crate::component::document::DocumentItemRef::new (
                                    value.0.doc.clone(),
                                    value.0.item,
                                    inner
                                )
                            )
                        )
                    },
                    _ => Err("Unable to convert Node ot partular type!")
                }
            }
        }
    }};
}

pub(crate) use DefaultChildrenAccess;
pub(crate) use DefaultParrentAccess;
pub(crate) use StaticName;
pub(crate) use NodeType;