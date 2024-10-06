use crate::structure::item_preprocessing::ItemPreprocessing;

#[derive(Clone)]
pub struct LPRelaxationResult {
    pub(crate) f_0: Vec<(usize, usize)>,
    pub(crate) x_up: Vec<[f64; 3]>,
    pub(crate) x: Vec<[f64; 3]>,
    pub(crate) v_up: f64,
    pub(crate) v_low: f64,
    pub(crate) relaxed: Vec<[ItemPreprocessing; 3]>,
}

impl LPRelaxationResult {
    pub(crate) fn new() -> Self {
        LPRelaxationResult {
            f_0: vec![],
            x_up: vec![],
            x: vec![],
            v_up: 0.0,
            v_low: 0.0,
            relaxed: vec![],
        }
    }
}