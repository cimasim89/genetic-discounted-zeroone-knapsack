use crate::structure::fix_result::UBFixResult;
use crate::structure::item::Item;
use crate::structure::item_preprocessing::ItemPreprocessing;
use crate::structure::problem::Problem;
use crate::structure::relaxation_result::LPRelaxationResult;
use rayon::prelude::*;
use std::sync::Mutex;

pub struct ProblemPreprocessor<'a> {
    problem: &'a Problem,
}

pub struct PreprocessingResult {
    pub relaxation_result: LPRelaxationResult,
    pub ub_fix_result: UBFixResult,
}

impl<'a> ProblemPreprocessor<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        ProblemPreprocessor {
            problem
        }
    }

    fn is_dominant(dominant: &Item, to_check: &Item) -> bool {
        let to_check_ratio = to_check.cost as f64 / to_check.gain as f64;
        let gain_difference = dominant.gain as f64 - to_check.gain as f64;
        let cost_difference = dominant.cost as f64 - to_check.cost as f64;
        (gain_difference / cost_difference) >= to_check_ratio
    }

    fn is_ranged_dominated(up: &Item, down: &Item, to_check: &Item) -> bool {
        let up_gain_difference = up.gain as f64 - to_check.gain as f64;
        let up_cost_difference = up.cost as f64 - to_check.cost as f64;

        let down_gain_difference = to_check.gain as f64 - down.gain as f64;
        let down_cost_difference = to_check.cost as f64 - up.cost as f64;
        (up_gain_difference / up_cost_difference) >= (down_gain_difference / down_cost_difference)
    }


    // equation 13 or equation 14
    fn is_first_lp_dominated(&self, vec: Vec<Item>) -> bool {
        let first = &vec[0];
        let second = &vec[1];
        let third = &vec[2];

        // equation 13
        let is_second_dominated = ProblemPreprocessor::is_dominant(second, first);
        // equation 14
        let is_third_dominated = ProblemPreprocessor::is_dominant(third, first);

        is_second_dominated || is_third_dominated
    }

    // equation 15 or equation 16
    fn is_second_lp_dominated(&self, vec: Vec<Item>) -> bool {
        let first = &vec[0];
        let second = &vec[1];
        let third = &vec[2];

        let is_first_dominated = ProblemPreprocessor::is_ranged_dominated(third, first, second);

        let is_third_dominated = ProblemPreprocessor::is_dominant(third, second);

        is_third_dominated || is_first_dominated
    }

    fn lp_relaxation_eliminate_by_dominance(&self) -> (Vec<(usize, usize)>, Vec<[ItemPreprocessing; 3]>) {
        let data = self.problem.data.clone();
        let f_0 = Mutex::new(vec![]);

        let relaxed_response: Vec<[ItemPreprocessing; 3]> = data.par_iter().enumerate().map(|(index, current_set)| {
            let mut itemp: [ItemPreprocessing; 3] = [
                ItemPreprocessing::new(0, 0, 0.0, index, 0),
                ItemPreprocessing::new(0, 0, 0.0, index, 1),
                ItemPreprocessing::new(0, 0, 0.0, index, 2),
            ];

            // Check if item (i, 1) is LP-dominated
            if self.is_first_lp_dominated(current_set.clone()) {
                itemp[0].ratio = f64::MIN;
                f_0.lock().unwrap().push((index, 0));
            } else {
                itemp[0].gain = current_set[0].gain;
                itemp[0].cost = current_set[0].cost;
                itemp[0].ratio = itemp[0].gain as f64 / itemp[0].cost as f64;
            }

            // Check if item (i, 2) is LP-dominated
            if self.is_second_lp_dominated(current_set.clone()) {
                itemp[1].ratio = f64::MIN;
                f_0.lock().unwrap().push((index, 1));
                itemp[2].gain = current_set[2].gain - itemp[0].gain;
                itemp[2].cost = current_set[2].cost - itemp[0].cost;
                itemp[2].ratio = itemp[2].gain as f64 / itemp[2].cost as f64;
            } else {
                itemp[1].gain = current_set[1].gain - itemp[0].gain;
                itemp[1].cost = current_set[1].cost - itemp[0].cost;
                itemp[1].ratio = itemp[1].gain as f64 / itemp[1].cost as f64;
                itemp[2].gain = current_set[2].gain - itemp[1].gain;
                itemp[2].cost = current_set[2].cost - itemp[1].cost;
                itemp[2].ratio = itemp[2].gain as f64 / itemp[2].cost as f64;
            }

            itemp
        }).collect();

        (f_0.into_inner().unwrap(), relaxed_response)
    }

    fn kp_greedy(&self, relaxed_original: Vec<[ItemPreprocessing; 3]>, f_0: Vec<(usize, usize)>) -> LPRelaxationResult {
        let m = self.problem.size;
        let mut remaining_capacity = self.problem.capacity as i64;
        let mut x = vec![[0.0; 3]; m as usize];
        let mut x_up = vec![[0.0; 3]; m as usize];
        let mut j: usize = 0;
        let mut v_up = 0.0;
        let mut v_low = 0.0;

        // order by e
        let mut relaxed: Vec<ItemPreprocessing> = relaxed_original.clone().into_iter().flat_map(|inner_vec| inner_vec.into_iter()).collect();
        relaxed.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

        while remaining_capacity > 0 && j < relaxed.len() {
            let i = relaxed[j].set_index;
            let k = relaxed[j].inner_index;
            if remaining_capacity > relaxed[j].cost {
                remaining_capacity -= relaxed[j].cost;
                v_up += relaxed[j].gain as f64;
                ProblemPreprocessor::reset_vectors(&mut x[i], &mut x_up[i]);
                x[i][k] = 1.0;
                x_up[i][k] = 1.0;
            } else {
                v_low = v_up;
                x_up[i][k] = remaining_capacity as f64 / relaxed[j].cost as f64;
                v_up += relaxed[j].gain as f64 * x_up[i][k];
                remaining_capacity = 0;
                ProblemPreprocessor::adjust_vectors(&mut x_up[i], &mut x[i], k);
            }
            j += 1;
        }

        while j <= ((3 * m) - (f_0.len() as i32)) as usize {
            let i = relaxed[j].set_index;
            let k = relaxed[j].inner_index;
            if remaining_capacity > relaxed[j].cost && ProblemPreprocessor::all_zero(&x[i]) {
                x[i][k] = 1.0;
                v_low += relaxed[j].gain as f64;
                remaining_capacity -= relaxed[j].cost;
            }
            j += 1;
        }


        LPRelaxationResult {
            f_0,
            x,
            x_up,
            v_up,
            v_low,
            relaxed: relaxed_original,
        }
    }

    fn ub_fix(&self, lp_relaxation_result: LPRelaxationResult) -> UBFixResult {
        let data = self.problem.data.clone();
        let x_up = lp_relaxation_result.x_up;
        let f_0: Vec<(usize, usize)> = lp_relaxation_result.f_0;
        let mut f_1: Vec<(usize, usize)> = vec![];
        let mut v_low_best = lp_relaxation_result.v_low;
        let mut x_best = lp_relaxation_result.x.clone();
        for index in 0..data.len() {
            let mut temp_relaxed = lp_relaxation_result.relaxed.clone();
            let current_set = data[index].clone();
            if x_up[index][2] == 1.0 {
                if f_0.contains(&(index, 1 as usize)) {
                    let first = &current_set[0];
                    let second = &current_set[1];

                    // equation 13
                    let is_dominated_by_second = ProblemPreprocessor::is_dominant(second, first);
                    if is_dominated_by_second {
                        temp_relaxed[index][0].gain = 0;
                        temp_relaxed[index][0].cost = 0;
                        temp_relaxed[index][0].ratio = f64::MIN;
                    } else {
                        temp_relaxed[index][0].gain = current_set[0].gain;
                        temp_relaxed[index][0].cost = current_set[0].cost;
                        temp_relaxed[index][0].ratio = temp_relaxed[index][0].gain as f64 / temp_relaxed[index][0].cost as f64;
                    }
                } else {
                    temp_relaxed[index][0].gain = current_set[0].gain;
                    temp_relaxed[index][0].cost = current_set[0].cost;
                    temp_relaxed[index][0].ratio = temp_relaxed[index][0].gain as f64 / temp_relaxed[index][0].cost as f64;
                    temp_relaxed[index][1].gain = current_set[1].gain - temp_relaxed[index][0].gain;
                    temp_relaxed[index][1].cost = current_set[1].cost - temp_relaxed[index][0].cost;
                    temp_relaxed[index][1].ratio = temp_relaxed[index][1].gain as f64 / temp_relaxed[index][1].cost as f64;
                }
                temp_relaxed[index][2].gain = 0;
                temp_relaxed[index][2].cost = 0;
                temp_relaxed[index][2].ratio = f64::MIN;

                let res_i = self.kp_greedy(temp_relaxed, f_0.clone());
                if res_i.v_low >= v_low_best {
                    v_low_best = res_i.v_low;
                    x_best = res_i.x;
                }
                if res_i.v_up.floor() <= v_low_best {
                    f_1.push((index, 1));
                }
            }
        }

        UBFixResult {
            f_1,
            x_best,
            v_best: v_low_best,
        }
    }

    fn reset_vectors(x: &mut [f64; 3], x_up: &mut [f64; 3]) {
        for inner in 0..3 {
            x[inner] = 0.0;
            x_up[inner] = 0.0;
        }
    }

    fn adjust_vectors(x_up: &mut [f64; 3], x: &mut [f64; 3], k: usize) {
        for inner_k in 0..3 {
            if inner_k != k && x_up[inner_k] == 1.0 {
                x_up[inner_k] = 1.0 - x_up[k];
                x[inner_k] = 0.0;
            }
        }
    }

    fn all_zero(x: &[f64; 3]) -> bool {
        x.iter().all(|&value| value == 0.0)
    }

    fn lp_relaxation(&self) -> LPRelaxationResult {
        // step 1
        let (f_0, relaxed) = self.lp_relaxation_eliminate_by_dominance();
        // step 2 and 3
        self.kp_greedy(relaxed, f_0)
    }

    pub(crate) fn process_problem(&mut self) -> PreprocessingResult {
        let relaxation_result = self.lp_relaxation();
        let ub_fix_result = self.ub_fix(relaxation_result.clone());
        PreprocessingResult {
            relaxation_result,
            ub_fix_result,
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::structure::make_item;

    fn make_problem() -> Problem {
        let capacity = 50;
        let data = vec![
            vec![make_item(10, 5), make_item(20, 10), make_item(30, 14)],
            vec![make_item(15, 7), make_item(25, 12), make_item(40, 15)],
        ];
        let size = data.len() as i32;

        Problem { capacity, data, size }
    }

    fn make_problem_low_capacity() -> Problem {
        let capacity = 20;
        let data = vec![
            vec![make_item(10, 5), make_item(20, 10), make_item(30, 14)],
            vec![make_item(15, 7), make_item(25, 12), make_item(45, 15)],
            vec![make_item(20, 1), make_item(35, 35), make_item(55, 35)],
            vec![make_item(11, 3), make_item(22, 15), make_item(33, 16)],
        ];
        let size = data.len() as i32;

        Problem { capacity, data, size }
    }

    #[test]
    fn test_lp_relaxation() {
        let problem = make_problem();
        let preprocessor = ProblemPreprocessor::new(&problem);
        let _result = preprocessor.lp_relaxation();
    }

    #[test]
    fn test_lp_relaxation_low_capacity() {
        let problem = make_problem_low_capacity();
        let preprocessor = ProblemPreprocessor::new(&problem);
        let _result = preprocessor.lp_relaxation();
    }


    #[test]
    fn test_process_problem_low_capacity() {
        let problem = make_problem_low_capacity();
        let mut preprocessor = ProblemPreprocessor::new(&problem);
        preprocessor.process_problem();
    }
}

