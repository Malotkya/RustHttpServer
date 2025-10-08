macro_rules! DefaultChildrenAccess {
    () => {
        fn children(&self) -> $crate::component::ChildIterator {
            $crate::component::ChildIterator::new(self.children.iter())
        }

        fn add_children(&mut self, list:&[Node], index:Option<usize>) -> Result<(), NodeError> {
            let length = self.children.len();
            let index = index.unwrap_or(length);

            let mut new_list:std::collections::LinkedList<Node> = list.iter()
                .map(|n|n.node())
                .collect();

            if index == 0 {
                new_list.append(&mut self.children);
                self.children = new_list;
            } else if index >= length {
                self.children.append(&mut new_list);
            } else {
                let mut tail = self.children.split_off(index);
                self.children.append(&mut new_list);
                self.children.append(&mut tail);
            }
            
            Ok(())
        }

        fn set_children(&mut self, list: &[Node]) -> Result<(), NodeError> {
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

        fn inner(&self) -> Option<&LinkedList<Node>> {
            Some(&self.children)
        }

        fn inner_mut(&mut self) -> Option<&mut LinkedList<Node>> {
            Some(&mut self.children)
        }
    };
}

macro_rules! DefaultParrentAccess {
    () => {
        fn parrent(&self) -> Option<&Node> {
            self.parrent.as_ref()
        }

        fn parrent_mut(&mut self) -> Option<&mut Node> {
            self.parrent.as_mut()
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
            pub(crate) $crate::component::document::DocumentItemRef,
            pub(crate) *const [<$struct_name $inner_name>]
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
                Node(self.0.clone())
            }
        }

        impl TryFrom<Node> for $struct_name {
            type Error = &'static str;

            fn try_from(value: Node) -> Result<Self, Self::Error> {
                match value.0.node_data() {
                    $node_type(inner) => {

                        Ok(
                            Self(
                                value.0.clone(),
                                inner as *const [<$struct_name $inner_name>]
                            )
                        )
                    },
                    _ => Err("Unable to convert Node to partular type!")
                }
            }
        }

        impl TryFrom<&Node> for $struct_name {
            type Error = &'static str;

            fn try_from(value: &Node) -> Result<Self, Self::Error> {
                match value.0.node_data() {
                    $node_type(inner) => {
                        value.0.item.inc();

                        Ok(
                            Self(
                                value.0.clone(),
                                inner as *const [<$struct_name $inner_name>]
                            )
                        )
                    },
                    _ => Err("Unable to convert Node to partular type!")
                }
            }
        }
    }};
}

pub(crate) use DefaultChildrenAccess;
pub(crate) use DefaultParrentAccess;
pub(crate) use StaticName;
pub(crate) use NodeType;