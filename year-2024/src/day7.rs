use core::panic;
use std::time::Instant;

use rayon::prelude::*;

#[aoc_macro::bench()]
pub fn day7_part1() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/7/input").expect("Cannot read input");
    let operators = vec![Operator::Add, Operator::Multiply];
    input.lines()
        .map(|line| parse_line(line))
        .filter(|(result, numbers)| calc_starter(result, numbers, &operators))
        .map(|(result, _)| result)
        .sum()
}

#[aoc_macro::bench()]
pub fn day7_part2() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/7/input").expect("Cannot read input");
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];
    input.lines()
        .map(|line| parse_line(line))
        .filter(|(result, numbers)| calc_starter(result, numbers, &operators))
        .map(|(result, _)| result)
        .sum()
}

#[aoc_macro::bench()]
pub fn day7_part2_rayon() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/7/input").expect("Cannot read input");
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];
    input.par_lines()
        .map(|line| parse_line(line))
        .filter(|(result, numbers)| calc_starter(result, numbers, &operators))
        .map(|(result, _)| result)
        .sum()
}

enum Operator {
    Add,
    Multiply,
    Concat,
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (left, right) = line.split_once(": ").unwrap();
    let result: u64 = left.parse().unwrap();
    let numbers: Vec<u64> = right.split(" ").map(|number| number.parse().unwrap()).collect();
    (result, numbers)
}

fn calc_starter(result: &u64, numbers: &Vec<u64>, operators: &Vec<Operator>) -> bool {
    operators.iter().any(|operator| calc(result, numbers, operators, operator, 0, numbers[0]))
}

fn calc(result: &u64, numbers: &Vec<u64>, operators: &Vec<Operator>, operator: &Operator, position: usize, value: u64) -> bool {
    match &numbers.len() - position {
        0 => panic!("panic in the disco"),
        1 => *result == value,
        _ => {
            let position = position + 1;
            let value = match operator {
                Operator::Add => value + numbers[position],
                Operator::Multiply => value * numbers[position],
                Operator::Concat => value * (10 as u64).pow(integer_digit(numbers[position])) + numbers[position]
            };

            operators.iter().any(|operator| calc(result, numbers, operators, operator, position, value))
        },
    }
}

fn integer_digit(value: u64) -> u32 {
    match value / 10 {
        0 => 1,
        n => integer_digit(n),
    }
}

