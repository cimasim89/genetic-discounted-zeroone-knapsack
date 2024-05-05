use crate::structure::item::Item;

#[derive(Debug)]
pub struct Solution {
    pub(crate) cost: i64,
    pub(crate) data: Vec<Item>,
    pub(crate) fitness: i64,
}


impl Solution {
    pub fn make_solution(data: Vec<Item>, fitness: i64, cost: i64) -> Solution {
        Solution {
            cost,
            data,
            fitness,
        }
    }
}
