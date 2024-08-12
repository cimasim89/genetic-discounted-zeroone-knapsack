use std::time::SystemTime;

use crate::genetic::{GeneticAlgorithm, KnapsackGeneticAlgorithm};
use crate::parser::*;
use crate::structure::configuration::ConfigurationByGenerations;
use crate::structure::problem::Problem;
use clap::Parser;
use env_logger::Env;
use log::{debug, info};
use uuid::Uuid;

mod parser;
mod structure;
mod utils;
mod genetic;
mod report;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = 100)]
    no_upgrade_limit: u8,

    #[arg(short, long, default_value_t = 500)]
    population_size: u32,

    #[arg(short, long, default_value_t = 1)]
    seed: u64,

    #[arg(short, long, default_value = "")]
    result_file_name: String,

    #[arg(short, long, default_value = "info")]
    log_level: String,
}


fn main() {
    let args = Args::parse();
    env_logger::Builder::from_env(Env::default().default_filter_or(args.log_level)).init();
    let problem = <Problem as ProblemParser>::parse_input(args.file_path.clone());
    let configuration = ConfigurationByGenerations {
        no_upgrade_limit: args.no_upgrade_limit,
        population_size: args.population_size,
        seed: args.seed,
    };
    let result_path = match args.result_file_name.as_str() {
        "" => "metrics.csv".to_string(),
        _ => args.result_file_name,
    };
    let csv = report::CSV {
        path: result_path,
    };


    let start = SystemTime::now();
    let mut executor = <KnapsackGeneticAlgorithm as GeneticAlgorithm>::init(problem, Box::new(configuration));
    let solution = executor.run();
    let elapsed = start.elapsed().unwrap();

    debug!("Solution: {:?}", solution);
    info!("Elapsed: {:.2?} best: {}", elapsed, solution.fitness);


    report::Report::generate(
        csv,
        Uuid::new_v4().to_string(),
        start,
        args.file_path.clone(),
        args.seed,
        args.no_upgrade_limit,
        args.population_size,
        &solution,
        elapsed,
    );
}

