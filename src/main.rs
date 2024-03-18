mod parser;
mod structure;
mod utils;
mod genetic;

use clap::Parser;
use crate::genetic::GeneticAlgorithm;
use crate::parser::*;
use crate::structure::problem::Problem;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = 1)]
    seed: u64,
}

fn main() {
    let args = Args::parse();
    let problem = <Problem as ProblemParser>::parse_input(args.file_path);
    let solution =  <Problem as GeneticAlgorithm>::run(problem, args.seed);
    println!("Solution: {:?}", solution);
}

