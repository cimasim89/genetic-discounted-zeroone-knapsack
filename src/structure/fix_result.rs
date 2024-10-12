#[derive(Clone)]
pub(crate) struct UBFixResult {
    pub(crate) f_1: Vec<(usize, usize)>,
    pub(crate) x_best: Vec<[f64; 3]>,
    pub(crate) v_best: f64,
}

impl UBFixResult {
    pub fn new(f_1: Vec<(usize, usize)>, x_best: Vec<[f64; 3]>, v_best: f64) -> Self {
        UBFixResult {
            f_1,
            x_best,
            v_best,
        }
    }

    pub fn empty() -> Self {
        UBFixResult {
            f_1: vec![],
            x_best: vec![],
            v_best: 0.0,
        }
    }
}
