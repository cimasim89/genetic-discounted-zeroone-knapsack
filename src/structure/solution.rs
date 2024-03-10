use crate::structure::item::Item;

#[derive(Debug)]
pub struct Solution {
    pub(crate) data: Vec<Item>,
}

impl Solution {
    pub fn make_solution(data: Vec<Item>) -> Solution {
        Solution {
            data,
        }
    }
}
