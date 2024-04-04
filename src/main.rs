mod parser;
mod structure;
mod utils;
mod genetic;

use clap::Parser;
use crate::genetic::GeneticAlgorithm;
use crate::parser::*;
use crate::structure::chromosome::Chromosome;
use crate::structure::configuration::ConfigurationByGenerations;
use crate::structure::problem::Problem;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = 500)]
    generations: u32,

    #[arg(short, long, default_value_t = 5)]
    mutation_factor: u8,

    #[arg(short, long, default_value_t = 500)]
    population_size: u32,

    #[arg(short, long, default_value_t = 1)]
    seed: u64,
}

fn generate_term_func (termination_generations: u32) -> Box<dyn Fn(&Chromosome, u32) -> bool> {
    Box::new(move |best: &Chromosome, generation: u32| -> bool {
        generation >= termination_generations
    })
}


fn main() {
    let args = Args::parse();
    let problem = <Problem as ProblemParser>::parse_input(args.file_path);
    let configuration = ConfigurationByGenerations {
        mutation_factor: args.mutation_factor,
        population_size: args.population_size,
        seed: args.seed,
        terminate_func: generate_term_func(args.generations),
    };
    let solution =  <ConfigurationByGenerations as GeneticAlgorithm>::run(problem, Box::new(configuration));
    println!("Solution: {:?}", solution);
}

