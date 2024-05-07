use std::time::Instant;

use clap::Parser;

use crate::genetic::{OOPGeneticAlgorithm, OOPGeneticAlgorithmStruct};
use crate::parser::*;
use crate::structure::configuration::ConfigurationByGenerations;
use crate::structure::problem::Problem;

mod parser;
mod structure;
mod utils;
mod genetic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = 100)]
    no_upgrade_limit: u8,

    #[arg(short, long, default_value_t = 5)]
    mutation_factor: u16,

    #[arg(short, long, default_value_t = 500)]
    population_size: u32,

    #[arg(short, long, default_value_t = 1)]
    seed: u64,
}


fn main() {
    let args = Args::parse();
    let problem = <Problem as ProblemParser>::parse_input(args.file_path);
    let configuration = ConfigurationByGenerations {
        no_upgrade_limit: args.no_upgrade_limit,
        mutation_factor: args.mutation_factor,
        population_size: args.population_size,
        seed: args.seed,
    };
    let now = Instant::now();
    let mut executor = <OOPGeneticAlgorithmStruct as OOPGeneticAlgorithm>::init(problem, Box::new(configuration));
    let solution = executor.run();
    println!("Solution: {:?}", solution);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

