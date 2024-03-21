use rand::Rng;
use rand::rngs::SmallRng;
use crate::structure::chromosome::Chromosome;
use crate::structure::item::Item;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::structure::configuration::{Configuration, ConfigurationByGenerations};
use crate::utils;

pub trait GeneticAlgorithm {
    fn run(problem: Problem, configuration: Box<dyn Configuration>) -> Solution;
}

impl GeneticAlgorithm for ConfigurationByGenerations {
    fn run(problem: Problem, configuration: Box<dyn Configuration>) -> Solution {
        println!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ", problem.capacity, problem.size);
        let mut rng = utils::make_rng(configuration.get_seed());
        let population = initialize_population(&problem, &configuration, &mut rng);
        let best = evolve(population, &problem, &configuration, &mut rng, 0);
        make_solution(&problem, &best)
    }
}

fn make_solution(problem: &Problem, chromosome: &Chromosome) -> Solution {
    let mut data: Vec<Item> = Vec::new();
    let mut cost = 0;
    for (i, gene) in chromosome.genes.iter().enumerate() {
        if *gene == 0 {
            continue;
        }
        data.push(problem.data[i][*gene - 1].clone());
        cost += problem.data[i][*gene - 1].cost;
    }
    Solution::make_solution(data, chromosome.fitness, cost)
}

fn initialize_population(problem: &Problem, configuration: &Box<dyn Configuration>, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Initializing population...");

    let mut population = Vec::new();

    for _ in 0..configuration.get_population_size(){
        let mut genes = Vec::new();
        for _ in 0..problem.size {
            // actually 0 is no selection
            genes.push(rng.gen_range(0..4));
        }
        population.push(Chromosome::init_chromosome(genes, problem.size));
    }

    population
}

fn fitness_func(chromosome: &Chromosome, problem: &Problem) -> i32 {
    let mut fitness = 0;
    let mut cost = 0;
    for (i, gene) in chromosome.genes.iter().enumerate() {

        // TODO use a better way to describe no selection
        if *gene == 0 {
            continue;
        }

        fitness += problem.data[i][*gene - 1].gain;
        cost += problem.data[i][*gene - 1].cost;
    }
    if cost > problem.capacity {
        fitness = 0;
    }
    fitness
}

fn evaluate(population: Vec<Chromosome>, problem: &Problem) -> Vec<Chromosome> {
    println!("evaluating population...");

    let mut evaluated = population.iter()
        .map(|c| Chromosome::evaluate_chromosome(c, fitness_func(c, problem), c.age + 1))
        .collect::<Vec<_>>();
    evaluated.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    evaluated.clone()
}

fn select(population: &Vec<Chromosome>, problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Selecting population...");
    roulette_wheel_selection(population, rng, population.len() as i32)
}

fn roulette_wheel_selection(population: &Vec<Chromosome>, rng: &mut SmallRng, n: i32) -> Vec<Chromosome> {
    let sum_fitness = population.iter().fold(0, |acc, c| acc + c.fitness);

    (0..n)
        .map(|_| {
            let mut slice = rng.gen_range(0..sum_fitness);
            let mut index = 0;
            for chromosome in population.iter() {
                slice -= chromosome.fitness;
                if slice <= 0 {
                    break;
                }
                index += 1;
            }
            population[index].clone()
        })
        .collect()
}

fn crossover(population: Vec<Chromosome>, problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Crossover population...");

    let mut new_population = Vec::new();

    for _ in 0..population.len() / 2 {
        let parent1 = &population[rng.gen_range(0..population.len())];
        let parent2 = &population[rng.gen_range(0..population.len())];

        let (child1, child2) = parent_crossover(parent1, parent2, problem, rng);

        new_population.push(child1);
        new_population.push(child2);
    }

    new_population
}

fn parent_crossover(parent1: &Chromosome, parent2: &Chromosome, problem: &Problem, rng: &mut SmallRng) -> (Chromosome, Chromosome) {
    let crossover_point = rng.gen_range(0..parent1.genes.len());

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

    let child1 = Chromosome::init_chromosome(child1_genes, problem.size);
    let child2 = Chromosome::init_chromosome(child2_genes, problem.size);

    (child1, child2)
}

fn mutate(population: Vec<Chromosome>,
          problem: &Problem,
          configuration: &Box<dyn Configuration>,
          rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Mutating population...");

    let mut new_population = Vec::new();

    population.iter().for_each(|c| {
        if rng.gen_range(0..1000) > 5 {
            new_population.push(c.clone());
            return;
        }
        let mut genes = c.genes.clone();
        let index = rng.gen_range(0..genes.len());
        let gene = rng.gen_range(0..4);
        genes[index] = gene;
        new_population.push(Chromosome::init_chromosome(genes, problem.size));
    });

    new_population
}


fn evolve<'a>(population: Vec<Chromosome>,
              problem: &'a Problem,
              configuration: &'a Box<dyn Configuration>,
              rng: &'a mut SmallRng,
              generation: u32) -> Chromosome {
    println!("Evolving population generation: {}", generation);

    let evaluated = evaluate(population, problem);
    let best_solution = evaluated.first().unwrap();

    if configuration.get_terminate_func()(best_solution, generation) {
        best_solution.clone()
    } else {
        let selection = select(&evaluated, problem, rng);
        let new_gen = crossover(selection, problem, rng);
        let mutated = mutate(new_gen, problem,configuration, rng);
        evolve(mutated, problem, configuration, rng, generation + 1)
    }
}
