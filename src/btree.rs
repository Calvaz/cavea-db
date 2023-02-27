use std::{any::type_name, mem, str};

use crate::pager::{Cursor, MAX_PAGE_SIZE};

const ROOT_NODE_SIZE: usize = 1;
const PARENT_KEY_SIZE: usize = 4;
const NODE_KEY_SIZE: usize = 4;
const NODE_VALUE_SIZE: usize = 20;

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

pub struct BtreeNode {
    node_type: NodeType,
    key: u32,
    value: String,
}

impl BtreeNode {
    fn new(node_type: NodeType, key: u32, value: &str) -> Self {
        BtreeNode {
            key,
            node_type,
            value: String::from(value),
        }
    }

    fn get_header_size() -> usize {
        ROOT_NODE_SIZE + PARENT_KEY_SIZE
    }

    fn get_node_size() -> usize {
        NODE_KEY_SIZE + NODE_VALUE_SIZE
    }

    fn get_parent_key(bytes: [u8; MAX_PAGE_SIZE]) -> Option<u32> {
        let mut buf_key = [0; PARENT_KEY_SIZE];
        buf_key.copy_from_slice(&bytes[ROOT_NODE_SIZE..PARENT_KEY_SIZE]);
        let key = u32::from_be_bytes(buf_key);
        let mut result = Some(key);
        if key == 0 {
            result = None
        }
        result
    }

    pub fn get(keys: [u8; MAX_PAGE_SIZE]) -> Vec<BtreeNode> {
        let nodes_bytes = keys.split_at(Self::get_header_size()).1;

        let mut nodes = Vec::<BtreeNode>::new();
        for kv in nodes_bytes.chunks(Self::get_node_size()) {
            let node = kv.split_at(NODE_KEY_SIZE);

            // get key
            let key_bytes = node.0;
            println!("key: {:?}", key_bytes);
            let mut buf_key = [0; NODE_KEY_SIZE];
            buf_key.copy_from_slice(&key_bytes[0..NODE_KEY_SIZE]);
            let key = u32::from_be_bytes(buf_key);

            if (key != 0) {
                // get value
                let value_bytes = node.1;
                let mut buf_value = [0; NODE_VALUE_SIZE];
                buf_value.copy_from_slice(&value_bytes[0..NODE_VALUE_SIZE]);
                let value = str::from_utf8(&buf_value).unwrap();

                let btree_node = BtreeNode::new(NodeType::Leaf, key, value);
                nodes.push(btree_node);
            }
        }
        println!("found {} nodes", nodes.len());

        nodes
    }

    // fn add(root_node: BtreeNode) -> Result<BtreeNode> {}
}
