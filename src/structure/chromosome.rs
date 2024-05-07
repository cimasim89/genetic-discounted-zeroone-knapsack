#[derive(Debug, Clone)]
pub struct Chromosome {
    pub(crate) genes: Vec<usize>,
    pub(crate) fitness: i64,
    pub(crate) size: i32,
    pub(crate) age: i32,
}

impl Chromosome {
    pub fn init_chromosome(genes: Vec<usize>, size: i32) -> Self {
        Chromosome {
            genes,
            fitness: 0,
            size,
            age: 0,
        }
    }

    pub fn evaluate_chromosome(self: &Self, fitness: i64, age: i32) -> Chromosome {
        Chromosome {
            genes: self.genes.clone(),
            fitness,
            size: self.size,
            age,
        }
    }

    pub fn set_fitness(self: &mut Self, fitness: i64) {
        self.fitness = fitness;
    }
}

