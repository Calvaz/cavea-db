use crate::pager::Pager;

pub struct Table {
    pub pager: Pager,
}

#[derive(Debug)]
pub struct Row {
    pub(crate) value: String,
}
