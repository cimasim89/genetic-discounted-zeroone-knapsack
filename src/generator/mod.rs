use crate::structure::chromosome::Chromosome;
use crate::structure::fix_result::UBFixResult;
use crate::structure::problem::Problem;
use crate::structure::relaxation_result::LPRelaxationResult;
use crate::utils::make_rng;
use rand::prelude::SmallRng;
use rand::Rng;

pub struct RandomChromosomeGenerator {
    problem: Problem,
    rng: SmallRng,
}

impl RandomChromosomeGenerator {
    pub fn new(problem: Problem, seed: u64) -> Self {
        RandomChromosomeGenerator {
            problem,
            rng: make_rng(seed),
        }
    }

    pub(crate) fn generate_chromosome(&mut self) -> Chromosome {
        let mut genes = vec![];
        for _ in 0..self.problem.size {
            genes.push(self.rng.gen_range(0..4));
        }
        Chromosome { genes, fitness: 0, age: 0 }
    }

    pub(crate) fn generate_chromosomes(&mut self, quantity: u32) -> Vec<Chromosome> {
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
    relaxation_result: LPRelaxationResult,
    ub_fix_result: UBFixResult,
}

impl EnhancedChromosomeGenerator {
    pub fn new(
        problem: Problem,
        seed: u64,
        relaxation_result: LPRelaxationResult,
        ub_fix_result: UBFixResult,
    ) -> Self {
        EnhancedChromosomeGenerator {
            problem,
            rng: make_rng(seed),
            relaxation_result,
            ub_fix_result,
        }
    }

    pub(crate) fn generate_chromosome_f0(&mut self) -> Chromosome {
        let mut genes = vec![];
        for i in 0..self.problem.size {
            let mut gene = 0;
            if !self.ub_fix_result.f_1.contains(&(i as usize, 1)) {
                gene = self.rng.gen_range(0..4);
                if gene == 1 && self.relaxation_result.f_0.contains(&(i as usize, 0)) {
                    gene = 2;
                }
                if gene == 2 && self.relaxation_result.f_0.contains(&(i as usize, 1)) {
                    gene = 3;
                }
                if gene == 3 && self.relaxation_result.f_0.contains(&(i as usize, 2)) {
                    gene = 0;
                }
            }
            genes.push(gene);
        }

        Chromosome {
            age: 0,
            genes,
            fitness: 0,
        }
    }

    pub(crate) fn generate_chromosome_f1(&mut self) -> Chromosome {
        let mut genes = vec![];
        for i in 0..self.problem.size {
            let mut gene = 0;
            if !self.ub_fix_result.f_1.contains(&(i as usize, 1)) {
                gene = self.rng.gen_range(0..4);
            }
            genes.push(gene);
        }

        Chromosome {
            age: 0,
            genes,
            fitness: 0,
        }
    }
}
