use rand::prelude::SmallRng;
use rand::SeedableRng;

pub fn make_rng(seed: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed)
}