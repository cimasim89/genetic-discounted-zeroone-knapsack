use crate::generator::{EnhancedChromosomeGenerator, RandomChromosomeGenerator};
use crate::preprocessing::PreprocessingResult;
use crate::structure::chromosome::Chromosome;
use crate::structure::configuration::Configuration;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::utils;
use log::{debug, info};
use rand::rngs::SmallRng;
use rand::Rng;

pub trait GeneticAlgorithm<'a> {
    fn init(problem: Problem, configuration: Box<dyn Configuration>, preprocessing_result: &'a PreprocessingResult) -> Self;
    fn run(&mut self) -> Solution;
}

pub struct KnapsackGeneticAlgorithm<'a> {
    best: Chromosome,
    remain_no_improved_generations: u8,
    configuration: Box<dyn Configuration>,
    population: Vec<Chromosome>,
    problem: Problem,
    rng: SmallRng,
    mutation_factor: u16,
    preprocessing_result: &'a PreprocessingResult,
}

impl<'a> KnapsackGeneticAlgorithm<'a> {
    pub(crate) fn new(problem: Problem, configuration: Box<dyn Configuration>, preprocessing_result: &'a PreprocessingResult) -> Self {
        KnapsackGeneticAlgorithm {
            best: Chromosome::init_chromosome(vec![]),
            remain_no_improved_generations: configuration.get_no_upgrade_limit(),
            rng: utils::make_rng(configuration.get_seed()),
            mutation_factor: configuration.get_initial_mutation_factor(),
            population: vec![],
            problem,
            preprocessing_result,
            configuration,
        }
    }

    fn get_chromosome_summary(&self, chromosome: &Chromosome) -> (i64, i64) {
        let (mut gain, mut cost) = (0, 0);
        for (i, gene) in chromosome.genes.iter().enumerate() {
            if *gene != 0 {
                gain += self.problem.data[i][*gene - 1].gain;
                cost += self.problem.data[i][*gene - 1].cost;
            }
        }
        (gain, cost)
    }

    fn find_max_rate(&self, chromosome: &Chromosome) -> (usize, usize) {
        let mut max_rate = 0.0;
        let mut max_gene = 0;
        let mut max_value = 1;
        for (gene, value) in chromosome.genes.iter().enumerate() {
            if *value != 0 {
                let curr_rate = self.problem.data[gene][*value - 1].rate;
                if curr_rate > max_rate {
                    max_rate = curr_rate;
                    max_gene = gene;
                    max_value = *value;
                }
            }
        }
        (max_gene, max_value)
    }

    fn repair_chromosome(&self, chromosome: &Chromosome) -> Chromosome {
        let capacity = self.problem.capacity;
        let mut c = chromosome.clone();
        let mut genes = chromosome.genes.clone();
        let mut cost = self.get_chromosome_summary(&chromosome).1;

        while cost > capacity as i64 {
            let (high_rate_gene, value) = self.find_max_rate(&c);
            cost -= self.problem.data[high_rate_gene][value - 1].cost;
            if value > 1 {
                cost += self.problem.data[high_rate_gene][value - 2].cost;
            }
            genes[high_rate_gene] = value - 1;
            c = Chromosome::init_chromosome(genes.clone());
        }
        c
    }

    fn initialize_population(&mut self) {
        debug!("Initializing population...");
        let mut generated = self.configuration.get_population_size();
        let best_preprocess = self.preprocessing_result.ub_fix_result.x_best.clone();

        if self.configuration.is_enhanced_enabled() {
            if !best_preprocess.is_empty() {
                let mut chromosome = KnapsackGeneticAlgorithm::map_preprocessed_item_to_chromosome(best_preprocess);
                chromosome = self.repair_chromosome(&chromosome);
                self.population.push(chromosome);
                generated -= 1;
            }

            let mut enhanced_gen = EnhancedChromosomeGenerator::new(
                self.problem.clone(),
                self.configuration.get_seed(),
                self.preprocessing_result.relaxation_result.clone(),
                self.preprocessing_result.ub_fix_result.clone(),
            );

            while generated > (self.configuration.get_population_size() as f64 * 0.95).floor() as u32 {
                let mut chromosome = enhanced_gen.generate_chromosome_f0();
                chromosome = self.repair_chromosome(&chromosome);
                self.population.push(chromosome);
                generated -= 1;
            }

            while generated > (self.configuration.get_population_size() as f64 * 0.95).floor() as u32 {
                let mut chromosome = enhanced_gen.generate_chromosome_f1();
                chromosome = self.repair_chromosome(&chromosome);
                self.population.push(chromosome);
                generated -= 1;
            }

            while generated > (self.configuration.get_population_size() as f64 * 0.975).floor() as u32 {
                let mut chromosome = enhanced_gen.generate_chromosome_f0_and_f1();
                chromosome = self.repair_chromosome(&chromosome);
                self.population.push(chromosome);
                generated -= 1;
            }
        }
        while generated > 0 {
            let mut chromosome = RandomChromosomeGenerator::new(self.problem.clone(), self.configuration.get_seed()).generate_chromosome();
            chromosome = self.repair_chromosome(&chromosome);
            self.population.push(chromosome);
            generated -= 1;
        }
    }

    fn map_preprocessed_item_to_chromosome(best_preprocess: Vec<[f64; 3]>) -> Chromosome {
        let genes = best_preprocess.iter().map(|x| {
            let mut value = 3;
            for v in x.iter().rev() {
                if *v == 1.0 {
                    break;
                }
                value -= 1;
            }
            value
        }).collect();
        Chromosome::init_chromosome(genes)
    }

    fn make_solution(&mut self, chromosome: &Chromosome, generations: u32) -> Solution {
        let mut data = Vec::new();
        let mut cost = 0;
        for (gene, value) in chromosome.genes.iter().enumerate() {
            if *value != 0 {
                data.push(self.problem.data[gene][*value - 1].clone());
                cost += self.problem.data[gene][*value - 1].cost;
            }
        }
        Solution::make_solution(data, chromosome.fitness, cost, generations)
    }

    fn fitness_func(&self, chromosome: &Chromosome) -> i64 {
        let (gain, cost) = self.get_chromosome_summary(&chromosome);
        if cost > self.problem.capacity as i64 {
            return 0;
        }
        gain
    }

    fn evaluate(&mut self) {
        debug!("Evaluating population...");
        let fitness: Vec<_> = self.population.iter().map(|c| self.fitness_func(c)).collect();
        for (i, chromosome) in self.population.iter_mut().enumerate() {
            chromosome.set_fitness(fitness[i]);
            chromosome.increase_age();
        }
        self.population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    }

    fn roulette_wheel_selection(&mut self) {
        let sum_fitness = self.population.iter().map(|c| c.fitness).sum();
        self.population = (0..self.population.len()).map(|_| {
            let mut slice = self.rng.gen_range(0..sum_fitness);
            let mut index = 0;
            for chromosome in self.population.iter() {
                slice -= chromosome.fitness;
                if slice <= 0 {
                    break;
                }
                index += 1;
            }
            self.population[index].clone()
        }).collect();
    }

    fn select(&mut self) {
        debug!("Selecting population...");
        self.roulette_wheel_selection();
    }

    fn parent_crossover(&mut self, parent1: &Chromosome, parent2: &Chromosome) -> (Chromosome, Chromosome) {
        let crossover_point = self.rng.gen_range(0..parent1.genes.len());
        let (child1_genes, child2_genes): (Vec<_>, Vec<_>) = parent1.genes.iter().zip(&parent2.genes)
            .enumerate()
            .map(|(i, (g1, g2))| if i < crossover_point { (*g1, *g2) } else { (*g2, *g1) })
            .unzip();

        let mut child1 = Chromosome::init_chromosome(child1_genes);
        child1 = self.repair_chromosome(&child1);
        let mut child2 = Chromosome::init_chromosome(child2_genes);
        child2 = self.repair_chromosome(&child2);

        (child1, child2)
    }

    fn crossover(&mut self) {
        debug!("Crossover population...");
        let mut new_population = Vec::new();
        for _ in 0..self.population.len() / 2 {
            let parent1 = self.population[self.rng.gen_range(0..self.population.len())].clone();
            let parent2 = self.population[self.rng.gen_range(0..self.population.len())].clone();
            let (child1, child2) = self.parent_crossover(&parent1, &parent2);
            new_population.push(child1);
            new_population.push(child2);
        }
        self.population = new_population;
    }

    fn mutate(&mut self) {
        debug!("Mutating population...");
        self.population.iter_mut().for_each(|c| {
            if self.rng.gen_range(0..1000) <= self.mutation_factor {
                let index = self.rng.gen_range(0..c.genes.len());
                let gene = self.rng.gen_range(0..4);
                c.genes[index] = gene;
                c.fitness = 0;
                c.age = 0;
            }
        });
    }

    fn check_is_end(&mut self, new_chromosome: Chromosome) -> bool {
        if new_chromosome.fitness > self.best.fitness {
            self.best = new_chromosome;
            self.remain_no_improved_generations = self.configuration.get_no_upgrade_limit();
            return false;
        }
        if self.remain_no_improved_generations > 0 {
            self.remain_no_improved_generations -= 1;
            return false;
        }
        true
    }

    fn evolve(&mut self) -> (Chromosome, u32) {
        let mut generation: u32 = 0;
        self.evaluate();
        while !self.check_is_end(self.population.first().cloned().unwrap_or_else(|| panic!("Problem occurs during evolution!"))) {
            info!("Evolving population generation: {} current best fitness: {}", generation, self.best.fitness);
            self.select();
            self.crossover();
            self.mutate();
            generation += 1;
            if generation % 10 == 0 && self.mutation_factor > 1 {
                self.mutation_factor -= 1;
            }
            self.evaluate();
        }
        (self.best.clone(), generation)
    }
}

impl<'a> GeneticAlgorithm<'a> for KnapsackGeneticAlgorithm<'a> {
    fn init(problem: Problem, configuration: Box<dyn Configuration>, preprocessing_result: &'a PreprocessingResult) -> Self {
        let mut executor = KnapsackGeneticAlgorithm::new(problem, configuration, preprocessing_result);
        executor.initialize_population();
        executor
    }

    fn run(&mut self) -> Solution {
        info!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ", self.problem.capacity, self.problem.size);
        let (best, generations) = self.evolve();
        self.make_solution(&best, generations)
    }
}