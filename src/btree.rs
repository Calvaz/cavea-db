use crate::pager::Cursor;

#[repr(u8)]
#[derive(Debug)]
pub enum NodeType {
    Root = 0,
    Internal = 1,
    Leaf = 2,
}

impl From<u8> for NodeType {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => NodeType::Root,
            0x1 => NodeType::Internal,
            0x2 => NodeType::Leaf,
            _ => NodeType::Leaf,
        }
    }
}

struct BtreeNode {
    node_type: NodeType,
    key: u32,
    value: String,
}

impl BtreeNode {
    fn new_leaf_node(cursor: Cursor, key: u32, value: &str) -> BtreeNode {
        BtreeNode {
            key,
            node_type: NodeType::Leaf,
            value: String::from(value),
        }
    }
}
