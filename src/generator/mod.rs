use crate::structure::chromosome::Chromosome;
use crate::structure::item::Item;
use crate::structure::problem::Problem;
use rand::prelude::SmallRng;
use rand::Rng;

#[derive(Clone)]
struct ItemP {
    a: i64,
    c: i64,
    e: f64,
    set_index: usize,   // Add set index field
    inner_index: usize, // Add inner index field
}

impl ItemP {
    fn new(a: i64, c: i64, e: f64, set_index: usize, inner_index: usize) -> Self {
        ItemP { a, c, e, set_index, inner_index }
    }
}


pub struct RandomChromosomeGenerator {
    problem: Problem,
    rng: SmallRng,
}

impl RandomChromosomeGenerator {
    pub fn new(problem: Problem, rng: SmallRng) -> Self {
        RandomChromosomeGenerator { problem, rng }
    }

    fn generate_chromosome(&mut self) -> Chromosome {
        let mut genes = vec![];
        for _ in 0..self.problem.size {
            genes.push(self.rng.gen_range(0..4));
        }
        Chromosome { genes, fitness: 0, age: 0 }
    }

    fn generate_chromosomes(&mut self, quantity: u32) -> Vec<Chromosome> {
        let mut chromosomes = vec![];
        for _ in 0..quantity {
            chromosomes.push(self.generate_chromosome());
        }
        chromosomes
    }
}


pub struct EnhancedChromosomeGenerator {
    problem: Problem,
    rng: SmallRng,
    processed_data: Vec<Item>,
}

impl EnhancedChromosomeGenerator {
    pub fn new(rng: SmallRng, problem: Problem) -> Self {
        let mut instance = EnhancedChromosomeGenerator { problem, rng, processed_data: vec![] };
        instance.process_problem();
        instance
    }

    fn process_problem(&mut self) {}

    fn generate_chromosome(&mut self) -> Chromosome {
        Chromosome {
            age: 0,
            genes: vec![],
            fitness: 0,
        }
    }

    fn is_dominant(dominant: &Item, to_check: &Item, ratio: f64) -> bool {
        let gain_difference = dominant.gain as f64 - to_check.gain as f64;
        let cost_difference = dominant.cost as f64 - to_check.cost as f64;
        (gain_difference / cost_difference) >= ratio
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

        let first_ratio = first.cost as f64 / first.gain as f64;

        // equation 13
        let is_second_dominated = EnhancedChromosomeGenerator::is_dominant(second, first, first_ratio);
        // equation 14
        let is_third_dominated = EnhancedChromosomeGenerator::is_dominant(third, first, first_ratio);

        is_second_dominated || is_third_dominated
    }

    // equation 15 or equation 16
    fn is_second_lp_dominated(&self, vec: Vec<Item>) -> bool {
        let first = &vec[0];
        let second = &vec[1];
        let third = &vec[2];

        let second_ratio = second.cost as f64 / second.gain as f64;

        let is_first_dominated = EnhancedChromosomeGenerator::is_ranged_dominated(third, first, second);

        let is_third_dominated = EnhancedChromosomeGenerator::is_dominant(third, second, second_ratio);

        is_third_dominated || is_first_dominated
    }


    fn lp_relaxation_sort_by_dominance(&self) -> (Vec<(usize, usize)>, Vec<ItemP>) {
        let data = self.problem.data.clone();
        let mut f_0: Vec<(usize, usize)> = vec![];
        let mut relaxed_vec: Vec<ItemP> = vec![];

        // step 1
        for index in 0..data.len() {
            let current_set = data[index].clone();
            let mut itemp: [ItemP; 3] = [
                ItemP::new(0, 0, 0.0, index, 0),
                ItemP::new(0, 0, 0.0, index, 1),
                ItemP::new(0, 0, 0.0, index, 2),
            ];

            // Check if item (i, 1) is LP-dominated
            if self.is_first_lp_dominated(current_set.clone()) {
                itemp[0].e = f64::MIN;
                f_0.push((index, 0));
            } else {
                itemp[0].c = current_set[0].gain;
                itemp[0].a = current_set[0].cost;
                itemp[0].e = itemp[0].c as f64 / itemp[0].a as f64;
            }

            // Check if item (i, 2) is LP-dominated
            if self.is_second_lp_dominated(current_set.clone()) {
                itemp[1].e = f64::MIN;
                f_0.push((index, 1));
                itemp[2].c = current_set[2].gain - itemp[0].c;
                itemp[2].a = current_set[2].cost - itemp[0].a;
                itemp[2].e = itemp[2].c as f64 / itemp[2].a as f64;
            } else {
                itemp[1].c = current_set[1].gain - itemp[0].c;
                itemp[1].a = current_set[1].cost - itemp[0].a;
                itemp[1].e = itemp[1].c as f64 / itemp[1].a as f64;
                itemp[2].c = current_set[2].gain - itemp[1].c;
                itemp[2].a = current_set[2].cost - itemp[1].a;
                itemp[2].e = itemp[2].c as f64 / itemp[2].a as f64;
            }

            relaxed_vec.extend_from_slice(&itemp);
        }

        relaxed_vec.sort_by(|a, b| b.e.partial_cmp(&a.e).unwrap());
        (f_0, relaxed_vec)
    }

    fn kp_greedy(&self, relaxed: Vec<ItemP>, f_0: Vec<(usize, usize)>) -> ((Vec<[f64; 3]>, i64), (Vec<[f64; 3]>, i64)) {
        let m = self.problem.size;
        let mut remaining_capacity = self.problem.capacity as i64;
        let mut x = vec![[0.0; 3]; m as usize];
        let mut x_up = vec![[0.0; 3]; m as usize];
        let mut j: usize = 0;
        let mut v_up = 0;
        let mut v_low = 0;

        while remaining_capacity > 0 && j < relaxed.len() {
            let i = relaxed[j].set_index;
            let k = relaxed[j].inner_index;
            if remaining_capacity > relaxed[j].a {
                remaining_capacity -= relaxed[j].a;
                v_up += relaxed[j].c;
                EnhancedChromosomeGenerator::reset_vectors(&mut x[i], &mut x_up[i]);
                x[i][k] = 1.0;
                x_up[i][k] = 1.0;
            } else {
                v_low = v_up;
                x_up[i][k] = remaining_capacity as f64 / relaxed[j].a as f64;
                v_up += relaxed[j].c * x_up[i][k] as i64;
                remaining_capacity = 0;
                EnhancedChromosomeGenerator::adjust_vectors(&mut x_up[i], &mut x[i], k);
            }
            j += 1;
        }

        while j <= ((3 * m) - (f_0.len() as i32)) as usize {
            let i = relaxed[j].set_index;
            let k = relaxed[j].inner_index;
            if remaining_capacity > relaxed[j].a && EnhancedChromosomeGenerator::all_zero(&x[i]) {
                x[i][k] = 1.0;
                v_low += relaxed[j].c;
                remaining_capacity -= relaxed[j].a;
            }
            j += 1;
        }

        ((x_up, v_up), (x, v_low))
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

    fn lp_relaxation(&self) -> ((Vec<[f64; 3]>, i64), (Vec<[f64; 3]>, i64)) {
        // step 1
        let (f_0, relaxed) = self.lp_relaxation_sort_by_dominance();
        // step 2 and 3
        self.kp_greedy(relaxed, f_0)
    }

    fn generate_chromosomes(&mut self, quantity: u32) -> Vec<Chromosome> {
        //TODO
        let mut chromosomes = vec![];
        for _ in 0..quantity {
            chromosomes.push(self.generate_chromosome());
        }
        chromosomes
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::generator::EnhancedChromosomeGenerator;
    use crate::structure::make_item;
    use rand::SeedableRng;

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
        ];
        let size = data.len() as i32;

        Problem { capacity, data, size }
    }

    #[test]
    fn test_lp_relaxation() {
        let problem = make_problem();

        let mut generator = EnhancedChromosomeGenerator::new(SmallRng::seed_from_u64(1),
                                                             problem);
        let ((x_up, v_up), (x, v_low)) = generator.lp_relaxation();
    }

    #[test]
    fn test_lp_relaxation_low_capacity() {
        let problem = make_problem_low_capacity();

        let mut generator = EnhancedChromosomeGenerator::new(SmallRng::seed_from_u64(1),
                                                             problem);
        let ((x_up, v_up), (x, v_low)) = generator.lp_relaxation();
    }
}



/*
fn lp_relaxation(&self) {
    let data = self.problem.data.clone();
    let size = data.len();
    let mut f_0: Vec<(usize, usize)> = vec![];
    let c_vec: Vec<[i64; 3]> = vec![[0; 3]; size];
    let a_vec: Vec<[i64; 3]> = vec![[0; 3]; size];
    let e_vec: Vec<[i64; 3]> = vec![[0; 3]; size];

    // step 1
    for index in (0..data.len()) {
        let current_set = data[index].clone();
        let mut c: [i64; 3] = [0; 3];
        let mut a: [i64; 3] = [0; 3];
        let mut e: [i64; 3] = [0; 3];
        /* Check if item (i, 1) is LP-dominated */
        if self.is_first_lp_dominated(current_set.clone()) {
            // implement row 4
            e[0] = i64::MIN;
            f_0.push((index, 1))
        } else {
            // implement row 5
            c[0] = current_set[1].gain;
            a[0] = current_set[1].cost;
            e[0] = c[0] / a[0];
        }

        /* Check if item (i, 2) is LP-dominated */
        if self.is_second_lp_dominated(current_set.clone()) {
            // implement row 7
            e[1] = i64::MIN;
            f_0.push((index, 2));

            // implement row 8
            c[2] = current_set[3].gain - current_set[1].gain;
            a[2] = current_set[3].cost - current_set[1].cost;
            e[2] = c[2] / a[2];
        } else {
            // implement row 9
            c[1] = current_set[2].gain - c[0];
            a[1] = current_set[2].cost - a[0];
            e[1] = c[1] / a[1];

            // implement row 10
            c[2] = current_set[3].gain - c[1];
            a[2] = current_set[3].cost - a[1];
            e[2] = c[2] / a[2];
        }
        // assign values to result vectors
        c_vec[index] = c;
        a_vec[index] = a;
        e_vec[index] = e;
    }

    let mut combined: Vec<([i64; 3], [i64; 3], [i64; 3])> = c_vec.into_iter().zip(a_vec).zip(e_vec).map(|((c, a), e)| (c, a, e)).collect();
    combined.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
}


 */

