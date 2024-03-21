use crate::structure::item::Item;
use crate::structure::problem::Problem;

pub(crate) mod problem;
pub(crate) mod item;
pub(crate) mod solution;
pub(crate) mod chromosome;
pub(crate) mod configuration;


pub fn make_problem(capacity: i32,
                    data: Vec<Vec<Item>>,
                    size: i32, ) -> Problem {
    Problem {
        capacity,
        data,
        size,
    }
}

pub fn make_item(gain: i64,
                 cost: i64, ) -> Item {
    Item {
        gain,
        cost,
    }
}



