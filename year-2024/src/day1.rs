use std::time::Instant;
use std::collections::HashMap;

#[aoc_macro::bench()]
pub fn day1_part1() -> u32 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2024/day/1/input").expect("Cannot read input");

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input.lines()
        .map(|line| {
            let n: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().ok().unwrap())
                .collect();
            (n[0], n[1])
        })
        .unzip();


    left.sort();
    right.sort();

    left.iter().zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc_macro::bench()]
pub fn day1_part2() -> u32 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2024/day/1/input").expect("Cannot read input");

    let (left, right): (Vec<u32>, Vec<u32>) = input.lines()
        .map(|line| {
            let n: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().ok().unwrap())
                .collect();
            (n[0], n[1])
        })
        .unzip();

    let right_count = right.iter().fold(
        HashMap::with_capacity(input.lines().count()),
        | mut map: HashMap<&u32, u32>, number | {
            match map.get(number) {
                Some(value) => map.insert(number, *value + 1),
                None => map.insert(number, 1),
            };
            map
        }
    );

    left.iter()
        .map(|number| number * right_count.get(number).unwrap_or(&0))
        .sum()
}

