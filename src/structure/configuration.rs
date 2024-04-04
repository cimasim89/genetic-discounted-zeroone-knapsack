use crate::structure::chromosome::Chromosome;

pub trait Configuration {
    fn get_mutation_factor(self: &Self) -> u8;
    fn get_population_size(self: &Self) -> u32;
    fn get_seed(self: &Self) -> u64;
    fn get_terminate_func(self: &Self) -> &Box<dyn Fn(&Chromosome, u32) -> bool>;
}

pub struct ConfigurationByGenerations {
    pub(crate) mutation_factor: u8,
    pub(crate) population_size: u32,
    pub(crate) seed:u64,
    pub(crate) terminate_func: Box<dyn Fn(&Chromosome, u32) -> bool>,
}

impl Configuration for ConfigurationByGenerations {
    fn get_mutation_factor(self: &Self) -> u8 {
        self.mutation_factor
    }
    fn get_population_size(self: &Self) -> u32 {
        self.population_size
    }

    fn get_seed(self: &Self) -> u64 {
        self.seed
    }

    fn get_terminate_func(self: &Self) -> &Box<dyn Fn(&Chromosome, u32) -> bool> {
        &self.terminate_func
    }
}






