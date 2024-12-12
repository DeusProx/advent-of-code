use std::time::Instant;

#[aoc_macro::bench()]
pub fn day2_part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/2/input").expect("Cannot read input");
    let count = input.lines()
        .map(|line: &str| Report::parse(line))
        .map(|report: Report| report.is_valid())
        .filter(|entry: &Safety| matches!(entry, Safety::Safe))
        .count();
    count as u32
}

#[aoc_macro::bench()]
pub fn day2_part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/2/input").expect("Cannot read input");
    let count = input.lines()
        .map(|line: &str| Report::parse(line))
        .map(|report: Report| report.is_valid_dampened())
        .filter(|entry: &Safety| matches!(entry, Safety::Safe))
        .count();
    count as u32
}

#[derive(Debug)]
pub enum Safety {
    Safe,
    Unsafe,
}

struct Report { levels: Vec<i8> }
impl Report {
    pub fn parse(input: &str) -> Self {
        Self {
            levels: input.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i8>>()
        }
    }

    pub fn is_valid(&self) -> Safety {
        let diffs = Self::get_diffs(&self.levels);
        Self::is_diff_valid(&diffs)
    }
    pub fn is_valid_dampened(&self) -> Safety {
        if matches!(self.is_valid(), Safety::Safe) {
            return Safety::Safe;
        }

        for index in 0..self.levels.len() {
            let mut sub_levels = self.levels.to_vec();
            sub_levels.remove(index);
            let sub_diffs = Self::get_diffs(&sub_levels);

            if matches!(Self::is_diff_valid(&sub_diffs), Safety::Safe) {
                return Safety::Safe;
            }
        }

        Safety::Unsafe
    }

    fn get_diffs(levels: &Vec<i8>) -> Vec<i8> {
        levels
            .windows(2)
            .map(|n| n[1] - n[0])
            .collect()
    }
    fn is_diff_valid(diffs: &Vec<i8>) -> Safety {
        let inc = diffs.iter().all(|diff| diff.signum() == 1);
        let dec = diffs.iter().all(|diff| diff.signum() == -1);
        let size = diffs.iter().all(|diff| (1..=3).contains(&diff.abs()));

        match (inc || dec) && size {
            true => Safety::Safe,
            false => Safety::Unsafe,
        }
    }
}

