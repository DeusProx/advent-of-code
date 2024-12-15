use std::{collections::HashMap, ops::{Deref, DerefMut}, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/11/input").expect("Cannot read input");

    let stones: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|c| c.parse::<u64>().unwrap()).collect();

    let mut cache: StoneCache = StoneCache(HashMap::new());
    stones.iter()
        .map(|&stone| blink(stone, 25, &mut cache))
        .sum()
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/11/input").expect("Cannot read input");

    let stones: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|c| c.parse::<u64>().unwrap()).collect();

    let mut cache: StoneCache = StoneCache(HashMap::new());
    stones.iter()
        .map(|&stone| blink(stone, 75, &mut cache))
        .sum()
}


pub fn blink(stone: u64, left: u8, cache: &mut StoneCache) -> u64 {
    if left == 0 {
        return 1
    }

    if let Some(sub_count) = cache.get(&(stone, left)) {
        return *sub_count;
    }

    let left = left - 1;
    let sub_count = match stone {
        0 => blink(1, left, cache),
        _ => {
            let digits = integer_digit(stone);
            match digits % 2 == 0 {
                true => {
                    let halfway_multiplier = u64::pow(10, digits as u32/ 2);
                    let stone1 = stone / halfway_multiplier;
                    let stone2 = stone - stone1 * halfway_multiplier;
                    blink(stone1, left, cache) + blink(stone2, left, cache)
                },
                false => blink(stone * 2024, left, cache),
            }
        }
    };

    cache.insert((stone, left + 1), sub_count);
    sub_count
}

pub struct StoneCache(HashMap<(u64, u8), u64>);

impl Deref for StoneCache {
    type Target = HashMap<(u64, u8), u64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StoneCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn integer_digit(value: u64) -> u64 {
    match value / 10 {
        0 => 1,
        n => integer_digit(n) + 1,
    }
}

