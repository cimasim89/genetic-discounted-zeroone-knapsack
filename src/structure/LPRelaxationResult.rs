use crate::structure::ItemPreprocessing::ItemPreprocessing;

#[derive(Clone)]
pub struct LPRelaxationResult {
    pub(crate) f_0: Vec<(usize, usize)>,
    pub(crate) x_up: Vec<[f64; 3]>,
    pub(crate) x: Vec<[f64; 3]>,
    pub(crate) v_up: i64,
    pub(crate) v_low: i64,
    pub(crate) relaxed: Vec<[ItemPreprocessing; 3]>,
}