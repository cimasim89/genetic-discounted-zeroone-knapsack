#[derive(Clone)]
pub struct ItemPreprocessing {
    pub(crate) cost: i64,
    pub(crate) gain: i64,
    pub(crate) ratio: f64,
    pub(crate) set_index: usize,   // Add set index field
    pub(crate) inner_index: usize, // Add inner index field
}


impl ItemPreprocessing {
    pub(crate) fn new(cost: i64, gain: i64, ratio: f64, set_index: usize, inner_index: usize) -> Self {
        ItemPreprocessing { cost, gain, ratio, set_index, inner_index }
    }
}