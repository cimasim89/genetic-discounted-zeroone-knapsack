#[derive(Clone)]
pub struct ItemPreprocessing {
    pub(crate) a: i64,
    pub(crate) c: i64,
    pub(crate) e: f64,
    pub(crate) set_index: usize,   // Add set index field
    pub(crate) inner_index: usize, // Add inner index field
}


impl ItemPreprocessing {
    pub(crate) fn new(a: i64, c: i64, e: f64, set_index: usize, inner_index: usize) -> Self {
        ItemPreprocessing { a, c, e, set_index, inner_index }
    }
}