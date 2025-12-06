use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/6/input").expect("Cannot read input");

    let mut iter = input.lines().rev();
    let acc: Vec<(&str, u64)> = iter.next()
        .unwrap()
        .split_whitespace()
        .map(|op| match op {
            "*" => (op, 1),
            "+" => (op, 0),
            _ => panic!("in the disco"),
        })
        .collect();

    iter.map(|line| line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
        )
        .fold(acc, |mut acc, ns| {
            for i in 0..acc.len() {
                match acc[i].0 {
                    "*" => acc[i].1 *= ns[i],
                    "+" => acc[i].1 += ns[i],
                    _ => panic!("in the disco"),
                }
            }
            acc
        })
        .iter()
        .map(|(_op, n)| n)
        .sum()
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/6/input").expect("Cannot read input");

    let width = input.find("\n").unwrap();
    let height = (input.len() + 1) / (width + 1);

    let last_line_index = (height - 1) * (width + 1);
    let ops: Vec<&str> = input[(last_line_index - 1)..(last_line_index + width - 1)]
        .split_whitespace()
        .collect();

    let range_size = (height - 2) * (width + 1);
    let ns: Vec<Vec<u64>> = (0..width).into_iter()
        .map(|i| input
            .get(i..=(range_size + i)).unwrap()
            .chars()
            .step_by(width + 1)
            .filter_map(|c| c.to_digit(10))
            .reduce(|acc, v| acc * 10 + v)
        )
        .fold(vec![vec![]], |mut acc, n| {
            match n {
                None => acc.push(vec![]),
                Some(n) => acc.last_mut().unwrap().push(n as u64),
            };
            acc
        });

    ops.iter()
        .zip(ns.iter())
        .map(|(&op, numbers)| match op {
            "*" => numbers.iter().copied().product::<u64>(),
            "+" => numbers.iter().copied().sum::<u64>(),
            _ => panic!("in the disco"),
        })
        .sum::<u64>()
}

