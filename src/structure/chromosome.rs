#[derive(Debug, Clone)]
pub struct Chromosome {
    pub(crate) genes: Vec<usize>,
    pub(crate) fitness: i64,
    pub(crate) age: i32,
}

impl Chromosome {
    pub fn init_chromosome(genes: Vec<usize>) -> Self {
        Chromosome {
            genes,
            fitness: 0,
            age: 0,
        }
    }

    pub fn set_fitness(self: &mut Self, fitness: i64) {
        self.fitness = fitness;
    }

    pub fn increase_age(self: &mut Self) {
        self.age = self.age + 1;
    }
}

