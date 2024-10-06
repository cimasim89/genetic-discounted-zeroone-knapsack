use crate::structure::chromosome::Chromosome;
use crate::structure::fix_result::UBFixResult;
use crate::structure::problem::Problem;
use crate::structure::relaxation_result::LPRelaxationResult;
use rand::prelude::SmallRng;
use rand::Rng;

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
    relaxation_result: LPRelaxationResult,
    ub_fix_result: UBFixResult,
}

impl EnhancedChromosomeGenerator {
    pub fn new(rng: SmallRng,
               problem: Problem,
               relaxation_result: LPRelaxationResult,
               ub_fix_result: UBFixResult,
    ) -> Self {
        EnhancedChromosomeGenerator {
            problem,
            rng,
            relaxation_result,
            ub_fix_result,
        }
    }

    fn generate_chromosome(&mut self) -> Chromosome {
        Chromosome {
            age: 0,
            genes: vec![],
            fitness: 0,
        }
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
