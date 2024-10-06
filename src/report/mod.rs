use crate::structure::solution::Solution;
use csv::Writer;
use std::env;
use std::fs::OpenOptions;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) struct Metric {
    architecture: String,
    best_fitness: i64,
    elapsed: Duration,
    file_name: String,
    generation: u32,
    instance: String,
    no_upgrade_limit: u8,
    os_description: String,
    population_size: u32,
    seed: u64,
    starting_time: SystemTime,
}

pub(crate) trait Exporter {
    fn export(&self, row: Metric);
}


#[derive(Clone)]
pub(crate) struct CSV {
    pub(crate) path: String,
}

impl CSV {
    fn row_to_record(&self, row: Metric) -> Vec<String> {
        vec![
            row.instance,
            format!("{}", row.starting_time.duration_since(UNIX_EPOCH).unwrap().as_secs()),
            row.file_name,
            row.seed.to_string(),
            row.no_upgrade_limit.to_string(),
            row.population_size.to_string(),
            format!("{}", row.elapsed.as_nanos()),
            row.generation.to_string(),
            row.best_fitness.to_string(),
            row.os_description,
            row.architecture,
        ]
    }
}


impl Exporter for CSV {
    fn export(&self, row: Metric) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.path.to_string())
            .unwrap();
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(self.row_to_record(row)).unwrap();
        wtr.flush().unwrap();
    }
}


pub(crate) struct Report;

impl Report {
    pub(crate) fn generate<T: Exporter>(g: T,
                                        instance: String,
                                        starting_time: SystemTime,
                                        file_name: String,
                                        seed: u64,
                                        no_upgrade_limit: u8,
                                        population_size: u32,
                                        solution: &Solution,
                                        duration: Duration) {
        let metric = Metric {
            architecture: env::consts::ARCH.to_string(),
            best_fitness: solution.fitness,
            elapsed: duration,
            file_name,
            generation: solution.generations,
            instance,
            no_upgrade_limit,
            os_description: env::consts::OS.to_string(),
            population_size,
            seed,
            starting_time,
        };

        g.export(metric);
    }
}