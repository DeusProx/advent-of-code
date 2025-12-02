use std::{time::Instant};

use rayon::prelude::*;

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: Solve with macro
    // let input = std::fs::read_to_string("../data/2025/day/2/test").expect("Cannot read input");
    let input = std::fs::read_to_string("../data/2025/day/2/input").expect("Cannot read input");

    input.split(',')
        .par_bridge()
        .map(str::trim)
        .map(|ids| ids.split_once('-').unwrap())
        .flat_map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .filter(|num| {
            let len = num.ilog10() + 1;
            if len % 2 != 0 { return false }

            let half = 10_u32.pow(len/2) as u64;
            let (first, last) = (num / half, num % half);
            if first != last { return false }

            true
        })
        // .inspect(|num| println!("invalid number: {num}"))
        .map(|num| num as u64)
        .sum()

}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    // let input = std::fs::read_to_string("../data/2025/day/2/test").expect("Cannot read input");
    let input = std::fs::read_to_string("../data/2025/day/2/input").expect("Cannot read input");

    input.split(',')
        .par_bridge()
        .map(str::trim)
        .map(|ids| ids.split_once('-').unwrap())
        .flat_map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .filter(|num| {
            let len = num.ilog10() + 1;
            for i in 2..=len {
                if len % i != 0 { continue }

                let step_size = 10_u32.pow(len/i) as u64;
                let mut rest = num.clone();
                let mut arr: Vec<u64> = Vec::with_capacity(step_size as usize);

                for j in 1..=i {
                    if j != i {
                        arr.push(rest % step_size);
                        rest = rest / step_size;
                    } else {
                        arr.push(rest);
                    }
                }
                arr.dedup();
                if arr.len() == 1 {
                    return true;
                }
            }

            false
        })
        // .inspect(|num| println!("invalid number: {num}"))
        .map(|num| num as u64)
        .sum()
}
