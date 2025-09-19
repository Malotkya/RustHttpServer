macro_rules! DefaultChildrenAccess {
    () => {
        fn children(&self) -> Result<&LinkedList<Node>, NodeError> {
            Ok(&self.children)
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

pub(crate) use DefaultChildrenAccess;
pub(crate) use DefaultParrentAccess;
pub(crate) use DefaultAttributeAccess;
pub(crate) use StaticName;