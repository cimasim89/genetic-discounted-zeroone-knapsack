use crate::genetic::{GeneticAlgorithm, KnapsackGeneticAlgorithm};
use crate::parser::*;
use crate::preprocessing::{PreprocessingResult, ProblemPreprocessor};
use crate::report::Report;
use crate::structure::configuration::ConfigurationByGenerations;
use crate::structure::problem::Problem;
use clap::Parser;
use env_logger::Env;
use log::{debug, info};
use std::time::SystemTime;
use uuid::Uuid;

mod parser;
mod structure;
mod utils;
mod genetic;
mod report;
mod generator;
mod preprocessing;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = 128)]
    no_upgrade_limit: u8,

    #[arg(short, long, default_value_t = 10)]
    initial_mutation_factor: u16,

    #[arg(short, long, default_value_t = 0)]
    seed: u64,

    #[arg(short, long, default_value = "")]
    result_file_name: String,

    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[arg(short, long, default_value = "false")]
    enhanced_enabled: bool,
}

fn parse_args() -> Args {
    Args::parse()
}

fn initialize_problem(file_path: &str) -> Problem {
    <Problem as ProblemParser>::parse_input(file_path.to_string())
}

fn initialize_preprocessor(problem: &Problem) -> ProblemPreprocessor {
    ProblemPreprocessor::new(problem)
}

fn initialize_configuration(args: &Args, seed: u64, population_size: u32, enhanced_enabled: bool) -> ConfigurationByGenerations {
    ConfigurationByGenerations {
        initial_mutation_factor: args.initial_mutation_factor,
        no_upgrade_limit: args.no_upgrade_limit,
        population_size,
        seed,
        enhanced_enabled,
    }
}

fn main() {
    let args = parse_args();
    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();
    let problem = initialize_problem(&args.file_path);

    let csv = report::CSV {
        path: if args.result_file_name.is_empty() { "metrics.csv".to_string() } else { args.result_file_name.clone() },
    };

    let start = SystemTime::now();
    let mut preprocessing_result = PreprocessingResult::empty();
    if args.enhanced_enabled {
        let mut preprocessor = initialize_preprocessor(&problem);
        preprocessing_result = preprocessor.process_problem();
    }

    let configuration = initialize_configuration(
        &args,
        args.seed,
        problem.size as u32 * 5,
        args.enhanced_enabled,
    );
    let mut executor = <KnapsackGeneticAlgorithm as GeneticAlgorithm>::init(
        problem.clone(),
        Box::new(configuration),
        &preprocessing_result,
    );
    let solution = executor.run();
    let elapsed = start.elapsed().unwrap();

    debug!("Solution: {:?}", solution);
    info!("Elapsed: {:.2?} best: {}", elapsed, solution.fitness);

    Report::generate(
        csv.clone(),
        Uuid::new_v4().to_string(),
        start,
        args.file_path.clone(),
        args.seed,
        args.no_upgrade_limit,
        problem.size as u32 * 5,
        &solution,
        elapsed,
        args.enhanced_enabled,
    );
}