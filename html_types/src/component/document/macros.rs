macro_rules! GenerateNodeFunctions {
    ( $( ($func:ident, $name:ident) ),+ ) => {  $( paste::paste!{
        pub fn $func (self: Rc<Self>, mut data:[<$name Data>]) -> DocumentItemRef<[<$name Data>]> {
            let list = &self.all_nodes as *const NodeArray as *mut NodeArray;
            
            let ptr_data = std::ptr::from_mut(&mut data);
            let ptr_item = NodeArray::add(list, NodeData::$name(data));

            DocumentItemRef {
                doc: self.clone(),
                item: ptr_item,
                data: ptr_data
            }
        }
    })+ };
}

pub(crate) use GenerateNodeFunctions;