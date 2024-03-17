use rand::Rng;
use rand::rngs::SmallRng;
use crate::structure::chromosome::Chromosome;
use crate::structure::problem::Problem;
use crate::structure::solution::Solution;
use crate::utils;

pub trait GeneticAlgorithm {
    fn run(problem: Problem, seed: u64) -> Solution;
}

impl GeneticAlgorithm for Problem {
    fn run(problem: Problem, seed: u64) -> Solution {
        println!("Running genetic algorithm for knapsack capacity: {}, selection size: {} ", problem.capacity, problem.size);
        let mut rng = utils::make_rng(seed);
        let population = initialize_population(&problem, &mut rng);
        let best = evolve(population, &problem, &mut rng, 0);
        Solution::make_solution(vec![])
    }
}

fn initialize_population(problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Initializing population...");

    let mut population = Vec::new();

    for _ in 0..100 {
        let mut genes = Vec::new();
        for _ in 0..problem.size {
            // actually 0 is no selection
            genes.push(rng.gen_range(0..4));
        }
        population.push(Chromosome::init_chromosome(genes, problem.size));
    }

    population
}

fn terminate(best: &Chromosome, generation: i32) -> bool {
    generation > 500
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

fn roulette_wheel_selection(population: &Vec<Chromosome>, rng: &mut SmallRng, n:i32) -> Vec<Chromosome> {

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

fn crossover(population: Vec<Chromosome>, problem: &Problem) -> Vec<Chromosome> {
    println!("Crossover population...");

    population
}

fn mutate(population: Vec<Chromosome>, problem: &Problem, rng: &mut SmallRng) -> Vec<Chromosome> {
    println!("Mutating population...");

    population
}


fn evolve<'a>(population: Vec<Chromosome>, problem: &'a Problem, rng: &'a mut SmallRng, generation: i32) -> Chromosome {
    println!("Evolving population generation: {}", generation);

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
