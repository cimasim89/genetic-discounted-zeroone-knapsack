mod parser;
mod structure;
use clap::Parser;
use rand::prelude::*;
use rand::SeedableRng;


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
    let mut rng = make_rng(args.seed);

    let _times = rng.gen_range(0..100);
    let _problem = parser::parse_input(args.file_path);
}


fn make_rng(seed: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed)
}