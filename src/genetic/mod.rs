use rand::rngs::SmallRng;
use rand::Rng;

use crate::structure::chromosome::Chromosome;
use crate::structure::configuration::Configuration;
use crate::structure::item::Item;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::utils;

pub trait GeneticAlgorithm {
    fn init(problem: Problem, configuration: Box<dyn Configuration>) -> Self;
    fn run(&mut self) -> Solution;
}

pub struct KnapsackGeneticAlgorithm {
    best_fitness: i64,
    remain_no_improved_generations: u8,
    configuration: Box<dyn Configuration>,
    population: Vec<Chromosome>,
    problem: Problem,
    rng: SmallRng,
    mutation_factor: u16,
}

impl KnapsackGeneticAlgorithm {
    pub(crate) fn new(problem: Problem, configuration: Box<dyn Configuration>) -> Self {
        KnapsackGeneticAlgorithm {
            best_fitness: 0,
            remain_no_improved_generations: configuration.get_no_upgrade_limit(),
            rng: utils::make_rng(configuration.get_seed()),
            configuration,
            population: vec![],
            problem,
            mutation_factor: 10,
        }
    }

    fn get_chromosome_summary(&self, chromosome: &Chromosome) -> (i64, i64) {
        let mut gain = 0;
        let mut cost = 0;
        for (i, gene) in chromosome.genes.iter().enumerate() {
            if *gene == 0 {
                continue;
            }

            gain += self.problem.data[i][*gene - 1].gain;
            cost += self.problem.data[i][*gene - 1].cost;
        }
        (gain, cost)
    }

    fn find_max_rate(&self, chromosome: &Chromosome) -> (usize, usize) {
        let mut max_rate = 0.0;
        let mut max_value: usize = 1;
        let mut max_gene: usize = 0;
        for (gene, value) in chromosome.genes.iter().enumerate() {
            if *value == 0 {
                continue;
            }

            let curr_rate = self.problem.data[gene][*value - 1].rate;

            if curr_rate > max_rate {
                max_rate = curr_rate;
                max_gene = gene;
                max_value = *value
            }
        }
        (max_gene, max_value)
    }


    fn repair_chromosome(&self, chromosome: &Chromosome, capacity: u32) -> Chromosome {
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
            c = Chromosome::init_chromosome(genes.clone())
        }
        c
    }


    fn initialize_population(&mut self) {
        println!("Initializing population...");

        let mut generated = self.configuration.get_population_size();

        while generated > 0 {
            let mut genes = Vec::new();
            for _ in 0..self.problem.size {
                // actually 0 is no selection
                genes.push(self.rng.gen_range(0..4));
            }
            let mut chromosome = Chromosome::init_chromosome(genes);
            chromosome = self.repair_chromosome(&chromosome, self.problem.capacity);
            self.population.push(chromosome);
            generated -= 1;
        }
    }


    fn make_solution(&mut self, chromosome: &Chromosome, generations: u32) -> Solution {
        let mut data: Vec<Item> = Vec::new();
        let mut cost = 0;
        for (gene, value) in chromosome.genes.iter().enumerate() {
            if *value == 0 {
                continue;
            }
            data.push(self.problem.data[gene][*value - 1].clone());
            cost += self.problem.data[gene][*value - 1].cost;
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
        println!("evaluating population...");

        let fitness: Vec<_> = self.population.iter().map(|c| {
            self.fitness_func(c)
        }).collect();

        for (i, chromosome) in self.population.iter_mut().enumerate() {
            chromosome.set_fitness(fitness[i]);
            chromosome.increase_age();
        }

        self.population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    }

    fn roulette_wheel_selection(&mut self) {
        let sum_fitness = self.population.iter().fold(0 as i64, |acc, c| acc + c.fitness);

        self.population = (0..self.population.len())
            .map(|_| {
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
            })
            .collect()
    }

    fn select(&mut self) {
        println!("Selecting population...");
        self.roulette_wheel_selection()
    }

    fn parent_crossover(&mut self, parent1: &Chromosome, parent2: &Chromosome) -> (Chromosome, Chromosome) {
        let crossover_point = self.rng.gen_range(0..parent1.genes.len());

        let mut child1_genes = Vec::new();
        let mut child2_genes = Vec::new();

        for i in 0..parent1.genes.len() {
            if i < crossover_point {
                child1_genes.push(parent1.genes[i]);
                child2_genes.push(parent2.genes[i]);
            } else {
                child1_genes.push(parent2.genes[i]);
                child2_genes.push(parent1.genes[i]);
            }
        }

        let mut child1 = Chromosome::init_chromosome(child1_genes);
        child1 = self.repair_chromosome(&child1, self.problem.capacity);
        let mut child2 = Chromosome::init_chromosome(child2_genes);
        child2 = self.repair_chromosome(&child2, self.problem.capacity);

        (child1, child2)
    }

    fn crossover(&mut self) {
        println!("Crossover population...");

        let mut new_population = Vec::new();

        for _ in 0..self.population.len() / 2 {
            let parent1 = self.population[self.rng.gen_range(0..self.population.len())].clone();
            let parent2 = self.population[self.rng.gen_range(0..self.population.len())].clone();

            let (child1, child2) = self.parent_crossover(&parent1, &parent2);

            new_population.push(child1.clone());
            new_population.push(child2.clone());
        }

        self.population = new_population;
    }

    fn mutate(&mut self) {
        println!("Mutating population...");

        self.population.iter_mut().for_each(|c| {
            if self.rng.gen_range(0..1000) > self.mutation_factor {
                return;
            }
            let index = self.rng.gen_range(0..c.genes.len());
            let gene = self.rng.gen_range(0..4);
            c.genes[index] = gene;
            c.fitness = 0;
            c.age = 0;
        });
    }

    fn check_is_end(&mut self, curr_fitness: i64) -> bool {
        if curr_fitness > self.best_fitness {
            self.best_fitness = curr_fitness;
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
        let mut condition = true;
        let mut best: Chromosome = match self.population.first() {
            None => { panic!("Population has not been initialized!") }
            Some(c) => {
                c.clone()
            }
        };

        while condition {
            println!("Evolving population generation: {} current best fitness: {}", generation, self.best_fitness);
            self.evaluate();
            best = match self.population.first() {
                None => { panic!("Problem occurs during evolution!") }
                Some(c) => {
                    c.clone()
                }
            };

            if self.check_is_end(best.fitness) {
                condition = false;
            } else {
                self.select();
                self.crossover();
                self.mutate();
                generation += 1;
            }
        }

        if (generation % 10 == 0 && self.mutation_factor > 1) {
            self.mutation_factor -= 1;
        }

        (best, generation)
    }
}


impl GeneticAlgorithm for KnapsackGeneticAlgorithm {
    fn init(problem: Problem, configuration: Box<dyn Configuration>) -> Self {
        let mut executor = KnapsackGeneticAlgorithm::new(problem, configuration);
        executor.initialize_population();
        executor
    }

    fn run(&mut self) -> Solution {
        println!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ",
                 self.problem.capacity,
                 self.problem.size);
        let (best, generations) = self.evolve();
        self.make_solution(&best, generations)
    }
}







