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
