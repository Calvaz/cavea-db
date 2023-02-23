use std::mem;

use crate::pager::Pager;

pub struct Table {
    pub pager: Pager,
    pub num_pages: u32,
    pub root_node: u8,
}

#[derive(Debug)]
pub struct Row {
    pub(crate) value: String,
}
