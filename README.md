# Genetic - discounted [0-1] knapsack

This is a proposal solution of discounted [0-1] knapsack problem using genetic algorithm

Example input datasets are available at https://oae.uphf.fr/content/UVHC/paAEJHcO0
After download unzip datasets and copy instances to main directory

## Run

This command will execute program with default attributes

```shell
cargo run -- --f <path_to_file>
```

## Help

This command will provide information about available attributes

```shell
cargo run -- -h
```

## Arguments

| **name**         | **arg**               | **Type** | **default** | **required** | **Description**                               |
|------------------|-----------------------|----------|-------------|--------------|-----------------------------------------------|
| file-path        | -f --file-path        | string   |             | true         | Input file path                               |
| seed             | -s --seed             | integer  | 1           | false        | Seed used for random initialization           | 
| version          | -V --version          | NA       |             | false        | Program version                               |
| help             | -h --help             | NA       |             | false        | Show help                                     |
| population size  | -p --population-size  | integer  | 500         | false        | Dimension of generated population             |
| no upgrade limit | -n --no-upgrade-limit | integer  | 10          | false        | Number of generations without fitness upgrade |
| result file name | -r --result_file_name | string   | metrics.csv | false        | .csv file where metrics will be stored        |
| log level        | -l --log-level        | string   | info        | false        | application log level                         |

## Massive Test

This command will execute a massive execution of the program with different seeds and input files

```shell
./run.sh setup
./run.sh all
```

## Massive Test Help

```shell
./run.sh
```

## Massive relative instance

```shell
./run.sh relative <instance_name>
```