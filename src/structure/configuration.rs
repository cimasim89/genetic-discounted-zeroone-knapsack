pub trait Configuration {
    fn get_no_upgrade_limit(self: &Self) -> u8;
    fn get_population_size(self: &Self) -> u32;
    fn get_initial_mutation_factor(self: &Self) -> u16;
    fn get_seed(self: &Self) -> u64;
    fn is_enhanced_enabled(self: &Self) -> bool;
}

pub struct ConfigurationByGenerations {
    pub(crate) no_upgrade_limit: u8,
    pub(crate) population_size: u32,
    pub(crate) initial_mutation_factor: u16,
    pub(crate) seed: u64,
    pub(crate) enhanced_enabled: bool,
}

impl Configuration for ConfigurationByGenerations {
    fn get_no_upgrade_limit(self: &Self) -> u8 {
        self.no_upgrade_limit
    }
    fn get_population_size(self: &Self) -> u32 {
        self.population_size
    }

    fn get_initial_mutation_factor(self: &Self) -> u16 {
        self.initial_mutation_factor
    }

    fn get_seed(self: &Self) -> u64 {
        self.seed
    }
    fn is_enhanced_enabled(self: &Self) -> bool {
        self.enhanced_enabled
    }
}






