use std::{time::Instant, usize};

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/5/input").expect("Cannot read input");

    let (fresh, available) = input.split_once("\n\n").unwrap();

    let fresh: Vec<(u64, u64)> = fresh.lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    available.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| fresh.iter().any(|(start, end)| start <= id && id <= end))
        .count()
}

#[aoc_macro::bench()]
pub fn part2() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/5/input").expect("Cannot read input");

    let (fresh, _) = input.split_once("\n\n").unwrap();

    let mut fresh: Vec<(u64, u64)> = fresh.lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    fresh.sort();
    let first = fresh.pop().unwrap();
    fresh.reverse();

    let (mut fresh, last) = fresh.iter()
        .fold(
            (Vec::with_capacity(fresh.len() + 1), first),
            |(mut acc, (a2, b2)), &(a1, b1)| { // do not forget that we reversed the data
                if b1 < a2 { // no overlap
                    acc.push((a2, b2));
                    return (acc, (a1, b1))
                }

                if b1 <= b2 { // overlap
                    return (acc, (a1, b2));
                }

                (acc, (a1, b1)) // new range includes old one
            }
        );

    fresh.push(last);

    fresh.iter()
        .flat_map(|&(a, b)| (a..=b).into_iter())
        .count()
}

