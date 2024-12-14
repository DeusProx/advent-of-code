use std::{collections::HashSet, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/10/input").expect("Cannot read input");

    let map = Map::parse(&input);
    map.trailheads().iter()
        .map(|index| map.score(&mut HashSet::new(), *index))
        // .inspect(|score| println!("Score: {}", score))
        .sum()
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/10/input").expect("Cannot read input");

    let map = Map::parse(&input);
    map.trailheads().iter()
        .map(|index| map.rate(*index))
        // .inspect(|rating| println!("Rating: {}", rating))
        .sum()
}

struct Map {
    width: usize,
    height: usize,
    fields: Vec<u8>
}
impl Map {
    pub fn parse(input: &str) -> Self {
        let width = input.find("\n").unwrap();
        let height = input.len() / (width + 1);
        let fields: Vec<u8> = input.chars()
            .filter(|c|c.is_ascii_digit())
            .map(|c| c as u8 - 48u8)
            .collect();
        Self { width, height, fields }
    }
    pub fn trailheads(&self) -> Vec<usize> {
        self.fields.iter()
            .enumerate()
            .filter(|(_, number)| **number == 0)
            .map(|(index, _)| index)
            .collect()
    }
    pub fn score(&self, cache: &mut HashSet<usize>, index: usize) -> u32 {
        if cache.get(&index).is_some() {
            return 0
        }

        let height = self.fields[index];
        match height {
            9 => {
                cache.insert(index);
                return 1
            },
            _ => self.higher(index).iter()
                .map(|index| self.score(cache, *index))
                .sum()
        }
    }
    pub fn rate(&self, index: usize) -> u32 {
        let height = self.fields[index];
        match height {
            9 => return 1,
            _ => self.higher(index).iter()
                .map(|index| self.rate(*index))
                .sum()
        }
    }
    pub fn higher(&self, index: usize) -> Vec<usize> {
        let next_height = self.fields[index] + 1;
        let mut directions: Vec<usize> = vec![];

        if index / self.width > 0  && self.fields[index - self.width] == next_height {
            directions.push(index - self.width);
        }
        if index / self.width < self.height - 1  && self.fields[index + self.width] == next_height {
            directions.push(index + self.width);
        }
        if index % self.width > 0  && self.fields[index - 1] == next_height {
            directions.push(index - 1);
        }
        if index % self.width < self.width - 1  && self.fields[index + 1] == next_height {
            directions.push(index + 1);
        }

        directions
    }
}

