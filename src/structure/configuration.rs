
pub trait Configuration {
    fn get_no_upgrade_limit(self: &Self) -> u8;
    fn get_mutation_factor(self: &Self) -> u8;
    fn get_population_size(self: &Self) -> u32;
    fn get_seed(self: &Self) -> u64;
}

pub struct ConfigurationByGenerations {
    pub(crate) no_upgrade_limit: u8,
    pub(crate) mutation_factor: u8,
    pub(crate) population_size: u32,
    pub(crate) seed:u64,
}

impl Configuration for ConfigurationByGenerations {
    fn get_no_upgrade_limit(self: &Self) -> u8 {
        self.no_upgrade_limit
    }
    fn get_mutation_factor(self: &Self) -> u8 {
        self.mutation_factor
    }
    fn get_population_size(self: &Self) -> u32 {
        self.population_size
    }

    fn get_seed(self: &Self) -> u64 {
        self.seed
    }
}






