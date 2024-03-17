use rand::Rng;
use rand::rngs::SmallRng;
use crate::structure::chromosome::Chromosome;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::utils;

pub trait GeneticAlgorithm {
    fn run(problem: Problem, seed: u64 ) -> Solution;
}

impl GeneticAlgorithm for Problem {
    fn run(problem: Problem, seed: u64 ) -> Solution {
        println!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ", problem.capacity, problem.size);
        let mut rng = utils::make_rng(seed);
        let population = initialize_population(&problem, &mut rng);
        let best = evolve(population, &problem, &mut rng, 0);
        Solution::make_solution(vec![])
    }
}

fn initialize_population(problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Initializing population...");


    vec![]
}

fn terminate(best: &Chromosome, generation: i32) -> bool {
    generation > 500
}

fn fitness_func(chromosome: &Chromosome, problem: &Problem) -> i32 {
    let mut fitness = 0;
    let mut cost = 0;
    for (i, gene) in chromosome.genes.iter().enumerate() {
        fitness += problem.data[i][*gene].gain;
        cost += problem.data[i][*gene].cost;
    }
    if cost > problem.capacity {
        fitness = 0;
    }
    fitness
}

fn evaluate(population: Vec<Chromosome>, problem: &Problem) -> Vec<Chromosome> {
    println!("Initializing population...");

    let mut evaluated = population.iter()
        .map(|c| Chromosome::evaluate_chromosome(c, fitness_func(c, problem), c.age + 1))
        .collect::<Vec<_>>();
    evaluated.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    evaluated.clone()
}

fn select(population: &Vec<Chromosome>, problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Selecting population...");

    let mut selected = Vec::new();
    for _ in 0..population.len() {
        let index = rng.gen_range(0..population.len());
        selected.push(population[index].clone());
    }
    selected
}

fn crossover(population: Vec<Chromosome>, problem: &Problem) -> Vec<Chromosome> {
    println!("Crossover population...");

    population
}

fn mutate(population: Vec<Chromosome>, problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Mutating population...");

    population
}


fn evolve<'a>(population: Vec<Chromosome>, problem: &'a Problem, rng: &'a mut SmallRng, generation: i32) -> Chromosome {
    println!("Evolving population size: {}", population.capacity());

    let evaluated = evaluate(population, problem);
    let best_solution = evaluated.first().unwrap();

    if terminate(best_solution, generation) {
        best_solution.clone()
    } else {
        let selection = select(&evaluated, problem, rng);
        let new_gen = crossover(selection, problem);
        let mutated = mutate(new_gen, problem, rng);
        evolve(mutated, problem, rng, generation + 1)
    }
}
