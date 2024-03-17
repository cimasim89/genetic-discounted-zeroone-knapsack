use crate::structure::item::Item;

#[derive(Debug)]
pub struct Solution {
    pub(crate) cost: i32,
    pub(crate) data: Vec<Item>,
    pub(crate) fitness: i32,
}

impl Solution {
    pub fn make_solution(data: Vec<Item>, fitness: i32, cost: i32) -> Solution {
        Solution {
            cost,
            data,
            fitness,
        }
    }
}
