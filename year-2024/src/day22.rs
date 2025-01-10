use std::{collections::{hash_map::Entry::{Vacant, Occupied}, HashMap}, ops::Deref, time::Instant};

use rayon::prelude::*;

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/22/input").expect("Cannot read input");
    let steps = 2000;

    input
        // .lines() // time: 6339 μs (~6 ms)
        .par_lines() // time: 590 μs (~0 ms)
        .map(|secret| secret.parse::<u64>().unwrap())
        .map(|mut secret| {
            for _ in 0..steps {
                secret = next(secret);
            }
            secret
        })
        .sum()
}

// Thought that shifting and fast modulo could improve performance but that does not seem to be the case.
// n % m == n & (m - 1)
const PRUNER: u64 = 16777216 - 1;
fn next(mut secret: u64) -> u64 {
    secret = (secret ^ (secret << 6)) & PRUNER;
    secret = (secret ^ (secret >> 5)) & PRUNER;
    secret = (secret ^ (secret << 11)) & PRUNER;
    secret
}

/*
const PRUNER: u64 = 16777216;
fn next(mut secret: u64) -> u64 {
    secret = (secret ^ (secret * 64)) % PRUNER;
    secret = (secret ^ (secret / 32)) % PRUNER;
    secret = (secret ^ (secret * 2048)) % PRUNER;
    secret
}
*/

#[aoc_macro::bench()]
pub fn part2() -> u16 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/22/input").expect("Cannot read input");
    let steps = 2000;

    let map: HashMap<(i8, i8, i8, i8), u16> = input
        // .lines() // time: 344380 μs (~344 ms)
        .par_lines() // time: 58105 μs (~58 ms)
        .map(|secret| Secret { secret: secret.parse::<u64>().unwrap() })
        .map(|secret| {
            // println!("\n{: >width$}: {}", *secret, *secret % 10, width = 14);
            secret
                .map_windows(|[a, b]| {
                    let secret = *b;
                    let price = (secret % 10) as u16;
                    let diff = price as i8 - (*a % 10) as i8;
                    (secret, price, diff)
                })
                //.inspect(|(secret, price, diff)| println!("{secret: >width$}: {price} ({diff})", width = 14))
                .take(steps)
                .map_windows(|a: &[(u64, u16, i8); 4]| *a)
                .fold(
                    HashMap::with_capacity(2000),
                    |mut map: HashMap<(i8, i8, i8, i8), u16>, value: [(u64, u16, i8); 4]| {
                        let key = (value[0].2, value[1].2, value[2].2, value[3].2);
                        let _ = map.try_insert(key, value[3].1);
                        map
                    }
                )
        })
        // .inspect(|map| println!("{}", map.len()))
        .reduce(|| HashMap::with_capacity(40951), |mut map: HashMap<(i8, i8, i8, i8), u16>, original| {
            original.iter().for_each(|(key, value)| {
                let entry = match map.entry(*key) {
                    Vacant(entry) => entry.insert(0),
                    Occupied(entry) => entry.into_mut(),
                };
                *entry += value;
            });
            map
        });
    // println!("{}", map.len());
    map.iter()
        .map(|(_, sum)| sum)
        .max()
        .unwrap()
        .clone()
}

struct Secret {
    secret: u64
}
impl Deref for Secret {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.secret
    }
}

impl Iterator for Secret {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.secret;
        self.secret = next(self.secret);
        Some(value)
    }
}
