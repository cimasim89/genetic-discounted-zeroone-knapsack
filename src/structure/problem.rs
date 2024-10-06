use crate::structure::item::Item;

#[derive(Debug)]
#[derive(Clone)]
pub struct Problem {
    pub(crate) capacity: u32,
    pub(crate) data: Vec<Vec<Item>>,
    pub(crate) size: i32,
}
