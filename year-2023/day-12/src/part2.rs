use std::{fs, time::Instant, collections::HashMap};
use rayon::prelude::*;

// Hint:
//   - Brute force solution from part 1 takes too long
//   - Use dynamic programming a.k.a. divide & conquer recursion
//   - dynamic programming can be easily optimized with caches
//
//   - Alternative: Sliding window?

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/12/input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
    assert_eq!(result, 51456609952403);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,     // ?
    Damaged,     // #
    Operational, // .
}

impl Spring {
    fn parse(c: char) -> Self {
        match c {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            _ => panic!("Nooo"),
        }
    }
    fn is_operational(self) -> bool {
        // Spring is either operational or unknown and thus can be assumed operational
        self != Spring::Damaged
    }
}

fn solve(springs: &[Spring], groups: &[usize]) -> i64 {
    let mut cache = HashMap::new();
    solve_inner(springs, groups, &mut cache)
}

fn solve_inner(springs: &[Spring], groups: &[usize], cache: &mut HashMap<(usize, usize), i64>) -> i64 {
    // termination conditions
    if groups.is_empty() {
        return match springs.contains(&Spring::Damaged) {
            false => 1, // valid
            true => 0,  // invalid
        }
    }

    if groups.iter().sum::<usize>() + groups.len() > springs.len() {
        return 0 // invalid since groups cannot fit anymore
    }

    let cache_index = (groups.len() - 1, springs.len());
    if let Some(cached) = cache.get(&cache_index){
        return *cached
    }

    // divide & conquer
    let mut count = 0;

    if springs[0].is_operational() {
        count += solve_inner(&springs[1..], groups, cache);
    }

    let (next_group_size, groups) = groups.split_first().unwrap();
    let (possible_group, boundary, springs) = split_at_pivot(springs, *next_group_size);
    if !possible_group.contains(&Spring::Operational) && boundary.is_operational() {
        count += solve_inner(springs, groups, cache);
    }

    cache.insert(cache_index, count);
    count
}

fn split_at_pivot<T>(elements: &[T], index: usize) -> (&[T], &T, &[T]) {
    (
        &elements[..index],
        &elements[index],
        &elements[index + 1..],
    )
}

fn calc(input: &str) -> i64 {
    let now = Instant::now();

    let sum = input
        .par_lines()
        .map(|line| parse_line(line))
        .map(|(mut springs, groups)| {
            springs.push(Spring::Unknown);
            let mut springs = springs.repeat(5);
            springs.pop();
            springs.push(Spring::Operational);
            (springs, groups.repeat(5))
        })
        .map(|(springs, groups)| solve(&springs, &groups))
        .sum();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    sum
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs_data, groups_data) = line.split_once(" ").unwrap();
    let springs: Vec<Spring> = springs_data
        .chars()
        .map(Spring::parse)
        .collect();
    let groups: Vec<usize> = groups_data
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    (springs, groups)
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/12/testinput").expect("Cannot read input file");
        let result = calc(&input);
        assert_eq!(result, 525_152);
    }
}

