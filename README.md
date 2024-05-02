# Genetic - discounted [0-1] knapsack

This is a proposal solution of discounted [0-1] knapsack problem using genetic algorithm

Example input datasets are available at https://oae.uphf.fr/content/UVHC/paAEJHcO0

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

| **name**         | **arg**                | **Type** | **default** | **required** | **Description**                               |
|------------------|------------------------|----------|-------------|--------------|-----------------------------------------------|
| file-path        | -f --file-path         | string   |             | true         | Input file path                               |
| seed             | -s --seed              | integer  | 1           | false        | Seed used for random initialization           | 
| version          | -V --version           | NA       |             | false        | Program version                               |
| help             | -h --help              | NA       |             | false        | Show help                                     |
| population size  | -p --population-size   | integer  | 500         | false        | Dimension of generated population             |
| mutation factor  | -m --mutation-factor   | integer  | 5           | false        | Factor of mutation ({v}/100)% -> 5 = 0.05%    |
| no upgrade limit | -n --no-upgrade-limit  | integer  | 10          | false        | Number of generations without fitness upgrade |