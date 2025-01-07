use std::{collections::HashMap, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/19/input").expect("Cannot read input");

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    designs.iter()
        .filter(|design| matches(design, &patterns, 0))
        .count()
}

// Do not care for optimization at all: 2830 μs (~2 ms)
fn matches(design: &str, patterns: &Vec<&str>, i: usize) -> bool {
    for pattern in patterns {
        let len = pattern.len();
        match (design.len() - i).cmp(&len) {
            std::cmp::Ordering::Less => continue,
            std::cmp::Ordering::Equal => {
                if *pattern == &design[i..(i + len)] {
                    return true
                }
            },
            std::cmp::Ordering::Greater => {
                if **pattern == design[i..(i + len)] && matches(design, patterns, i + len) {
                    return true
                }
            },
        };
    }
    false
}

#[aoc_macro::bench()]
pub fn part2() -> usize {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/19/input").expect("Cannot read input");

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    designs.iter()
        .map(|design| {
            let mut cache: HashMap<usize, usize> = HashMap::with_capacity(1000);
            all_matches(&mut cache, design, &patterns, 0)
        })
        .sum()
}

// Better Cache Design takes less time: 21091 μs (~21 ms)
fn all_matches<'a>(cache: &mut HashMap<usize, usize>, design: &str, patterns: &'a Vec<&str>, i: usize) -> usize {
    if let Some(value) = cache.get(&i) {
        return *value;
    }

    let mut count: usize = 0;
    for pattern in patterns {
        let len = pattern.len();
        match (design.len() - i).cmp(&len) {
            std::cmp::Ordering::Less => continue,
            std::cmp::Ordering::Equal => {
                if *pattern == &design[i..(i + len)] {
                    count += 1;
                }
            },
            std::cmp::Ordering::Greater => {
                if **pattern == design[i..(i + len)] {
                    let value = all_matches(cache, design, patterns, i + len);
                    count += value;
                }
            },
        };
    }
    cache.insert(i, count);
    count
}


/*
// Bad Cache design with &str takes long time: 263968 μs (~263 ms)
fn all_matches<'a>(cache: &mut HashMap<(&'a str, usize), usize>, design: &str, patterns: &'a Vec<&str>, i: usize) -> usize {
    let mut count: usize = 0;
    for pattern in patterns {
        let len = pattern.len();
        match (design.len() - i).cmp(&len) {
            std::cmp::Ordering::Less => continue,
            std::cmp::Ordering::Equal => {
                let end = i + len;
                if *pattern == &design[i..end] {
                    count += 1;
                }
            },
            std::cmp::Ordering::Greater => {
                let end = i + len;
                if let Some(value) = cache.get(&(pattern, end)) {
                    count += value;
                    continue;
                }

                if **pattern == design[i..end] {
                    let value = all_matches(cache, design, patterns, end);
                    count += value;
                    cache.insert((pattern, end), value);
                }
            },
        };
    }
    count
}
*/
