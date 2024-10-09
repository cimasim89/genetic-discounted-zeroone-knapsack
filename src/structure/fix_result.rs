#[derive(Clone)]
pub(crate) struct UBFixResult {
    pub(crate) f_1: Vec<(usize, usize)>,
    pub(crate) x_best: Vec<[f64; 3]>,
    pub(crate) v_best: f64,
}
