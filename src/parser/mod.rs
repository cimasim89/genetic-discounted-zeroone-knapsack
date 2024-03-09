mod utils;

use std::io::Result;
use std::fs::File;
use std::io::{self};
use crate::structure::problem::Problem;
use crate::structure::item::Item;
use crate::structure::{make_problem, make_item};


pub fn parse_input(file_name: String) -> Problem {
    match utils::read_lines(&file_name) {
        Err(why) => panic!("couldn't read {}: {}", file_name, why),
        Ok(lines) =>
            {
                collector(lines)
            }
    }
}

fn collector(lines: io::Lines<io::BufReader<File>>) -> Problem {
    let ((size, capacity), header_rest) = parse_headers(lines);
    let (gains, gain_rest) = split_groups(header_rest);
    let (costs, cost_rest) = split_groups(gain_rest);
    let data = make_data(gains, costs);

    make_problem(
        capacity,
        data,
        size,
    )
}

fn make_data(gains: Vec<String>, costs: Vec<String>) -> Vec<Vec<Item>> {
    let mut data: Vec<Vec<Item>> = Vec::new();

    for (group_gains, group_costs) in gains.iter().zip(costs.iter()) {
        let mut group: Vec<Item> = Vec::new();
        for (gain, cost) in group_gains.split_whitespace().zip(group_costs.split_whitespace()) {
            let item = make_item(
                gain.parse::<i32>().unwrap(),
                cost.parse::<i32>().unwrap());
            group.push(item);
        }
        data.push(group);
    }

    data
}

fn parse_headers(mut lines: io::Lines<io::BufReader<File>>) -> ((i32, i32), io::Lines<io::BufReader<File>>) {
    let size_line = lines.next();

    let s = match size_line {
        None => panic!("couldn't read "),
        Some(l) => match l {
            Err(why) => panic!("couldn't read {}", why),
            Ok(l) => l
        }
    };
    let size = s.parse::<i32>().unwrap();


    let capacity_line = lines.next();
    let c = match capacity_line {
        None => panic!("couldn't read "),
        Some(l) => match l {
            Err(why) => panic!("couldn't read {}", why),
            Ok(l) => l
        }
    };
    let capacity = c.parse::<i32>().unwrap();

    ((size, capacity), lines)
}

fn split_groups(mut lines: io::Lines<io::BufReader<File>>) -> (Vec<String>, io::Lines<io::BufReader<File>>) {
    let mut v: Vec<String> = Vec::new();
    while let Some(line) = lines.next() {
        let val = unwrap_next_line_result(line);
        if val.is_empty() {
            break;
        }
        v.push(val)
    }

    (v.clone(), lines)
}

fn unwrap_next_line_result(next: Result<String>) -> String {
    match next {
        Err(why) => panic!("couldn't read {}", why),
        Ok(l) => l
    }
}
