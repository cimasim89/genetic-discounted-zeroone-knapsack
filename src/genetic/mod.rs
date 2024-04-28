use rand::Rng;
use rand::rngs::SmallRng;
use crate::structure::chromosome::Chromosome;
use crate::structure::item::Item;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::structure::configuration::Configuration;
use crate::utils;

pub trait OOPGeneticAlgorithm {
    fn init(&mut self);
    fn run(&mut self) -> Solution;
}

pub struct OOPGeneticAlgorithmStruct {
    configuration: Box<dyn Configuration>,
    population: Vec<Chromosome>,
    problem: Problem,
    rng: SmallRng
}

impl OOPGeneticAlgorithmStruct {

    pub(crate) fn new(problem: Problem, configuration: Box<dyn Configuration>) -> Box<dyn OOPGeneticAlgorithm> {
        Box::new(OOPGeneticAlgorithmStruct {
            rng:utils::make_rng(configuration.get_seed()),
            configuration,
            population: vec![],
            problem,
        })
    }


    fn initialize_population(&mut self) -> Vec<Chromosome> {
        println!("Initializing population...");
        let mut population = Vec::new();

        let mut generated = self.configuration.get_population_size();

        while generated > 0 {
            let mut genes = Vec::new();
            for _ in 0..self.problem.size {
                // actually 0 is no selection
                genes.push(self.rng.gen_range(0..4));
            }
            let chromosome = Chromosome::init_chromosome(genes, self.problem.size);
            let fitness = self.fitness_func(&chromosome);
            if fitness == 0 {
                continue
            }
            Chromosome::set_fitness(&chromosome,fitness);
            population.push(chromosome);
            generated -= 1;
        }

        population
    }



    fn make_solution(&mut self, chromosome: &Chromosome) -> Solution {
        let mut data: Vec<Item> = Vec::new();
        let mut cost = 0;
        for (i, gene) in chromosome.genes.iter().enumerate() {
            if *gene == 0 {
                continue;
            }
            data.push(self.problem.data[i][*gene - 1].clone());
            cost += self.problem.data[i][*gene - 1].cost;
        }
        Solution::make_solution(data, chromosome.fitness, cost)
    }

    fn fitness_func(&self,chromosome: &Chromosome) -> i64 {
        let mut fitness = 0;
        let mut cost = 0;
        for (i, gene) in chromosome.genes.iter().enumerate() {

            if *gene == 0 {
                continue;
            }

            fitness += self.problem.data[i][*gene - 1].gain;
            cost += self.problem.data[i][*gene - 1].cost;
        }
        if cost > self.problem.capacity as i64 {
            fitness = 0;
        }
        fitness
    }

    fn evaluate(&mut self) {
        println!("evaluating population...");

        let mut evaluated = self.population.iter()
            .map(|c| Chromosome::evaluate_chromosome(c, self.fitness_func(c), c.age + 1))
            .collect::<Vec<_>>();
        evaluated.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        self.population = evaluated
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

        let child1 = Chromosome::init_chromosome(child1_genes, self.problem.size);
        let child2 = Chromosome::init_chromosome(child2_genes, self.problem.size);

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

        let mut new_population = Vec::new();

        self.population.iter().for_each(|c| {
            if self.rng.gen_range(0..1000) > 5 {
                new_population.push(c.clone());
                return;
            }
            let mut genes = c.genes.clone();
            let index = self.rng.gen_range(0..genes.len());
            let gene = self.rng.gen_range(0..4);
            genes[index] = gene;
            new_population.push(Chromosome::init_chromosome(genes, self.problem.size));
        });

        self.population = new_population
    }

    fn evolve(&mut self) -> Chromosome {
        let mut generation:u32 = 0;
        let mut condition = true;
        let mut best: Chromosome = self.population.first_mut().unwrap().clone();

        while condition {
            println!("Evolving population generation: {}", generation);
            self.evaluate();
            best = self.population.first_mut().unwrap().clone();

            if generation >= 500 {
                condition = false;
            } else {
                self.select();
                self.crossover();
                self.mutate();
                generation += 1;
            }
        }

        best

    }
}


impl OOPGeneticAlgorithm for OOPGeneticAlgorithmStruct {

    fn init(&mut self) {
        self.population = self.initialize_population()
    }

    fn run(&mut self) -> Solution {
        println!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ",
                 self.problem.capacity,
                 self.problem.size);
        let best = self.evolve();
        self.make_solution(&best)
    }

}







