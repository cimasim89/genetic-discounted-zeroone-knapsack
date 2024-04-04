
#[derive(Debug, Clone)]
pub struct Chromosome {
    pub(crate) genes: Vec<usize>,
    pub(crate) fitness: i64,
    pub(crate) size: i32,
    pub(crate) age: i32,
}

impl Chromosome {
    pub fn init_chromosome(genes: Vec<usize>, size:i32) -> Self {
        Chromosome  {
            genes,
            fitness: 0,
            size,
            age: 0,
        }
    }

    pub fn evaluate_chromosome(self: &Self,fitness:i64, age:i32) -> Chromosome {
        Chromosome  {
            genes: self.genes.clone(),
            fitness,
            size: self.size,
            age,
        }
    }

    pub fn set_fitness(self: &Self, fitness: i64) -> Chromosome {
        Chromosome {
            genes: self.genes.clone(),
            fitness,
            size: self.size,
            age: self.age,
        }
    }

    pub fn set_size(self: &Self, size: i32) -> Chromosome {
        Chromosome {
            genes: self.genes.clone(),
            fitness: self.fitness,
            size,
            age: self.age,
        }
    }

    pub fn set_age(self: &Self, age: i32) -> Self {
        Chromosome {
            genes: self.genes.clone(),
            fitness: self.fitness,
            size: self.size,
            age,
        }
    }
}

