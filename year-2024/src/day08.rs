use std::{collections::HashSet, ops::{Add, Sub}, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/8/input").expect("Cannot read input");

    let antenna_buckets = parse(&input);

    let width = input.find('\n').unwrap() as i8;
    let height = (input.len() / (width + 1) as usize) as i8;
    let bucket_iter = antenna_buckets.iter().filter(|antennas| !antennas.is_empty());

    let mut antinodes: HashSet<Position> = HashSet::with_capacity(1000);
    for antennas in bucket_iter {
        for (index, antenna1) in antennas.iter().enumerate() {
            for antenna2 in antennas.iter().skip(index + 1) {
                let delta = antenna1.position - antenna2.position;

                let antinode1 = antenna1.position + delta;
                if 0 <= antinode1.x && antinode1.x < width && 0 <= antinode1.y && antinode1.y < height {
                    antinodes.insert(antinode1);
                }

                let antinode2 = antenna2.position - delta;
                if 0 <= antinode2.x && antinode2.x < width && 0 <= antinode2.y && antinode2.y < height {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes.len() as u32
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/8/input").expect("Cannot read input");

    let antenna_buckets = parse(&input);

    let width = input.find('\n').unwrap() as i8;
    let height = (input.len() / (width + 1) as usize) as i8;
    let bucket_iter = antenna_buckets.iter().filter(|antennas| !antennas.is_empty());

    let mut antinodes: HashSet<Position> = HashSet::with_capacity(1000);
    for antennas in bucket_iter {
        for (index, antenna1) in antennas.iter().enumerate() {
            for antenna2 in antennas.iter().skip(index + 1) {
                for antinode in AntinodeIterator::new(antenna1.position, antenna2.position, width, height) {
                    antinodes.insert(antinode);
                }
                for antinode in AntinodeIterator::new(antenna2.position, antenna1.position, width, height) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn parse(input: &str) -> Vec<Vec<Antenna>> {
    input
        .lines().enumerate()
        .flat_map(move |(y, line)| line.char_indices()
            .filter(|(_, c)| *c != '.')
            .map(move |(x, c)| Antenna::new(c, x as i8, y as i8)
        ))
        .fold(vec![Vec::with_capacity(5); 256], |mut acc, antenna| {
            let index = antenna.frequency as u8 as usize;
            acc[index].push(antenna);

            acc
        })
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Copy)]
struct Position {
    x: i8,
    y: i8,
}
impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Antenna {
    frequency: char,
    position: Position,
}
impl Antenna {
    pub fn new(frequency: char, x: i8, y: i8) -> Self {
        Antenna {
            frequency,
            position: Position { x, y },
        }
    }
}

struct AntinodeIterator {
    delta: Position,
    current: Position,
    width: i8,
    height: i8,
}
impl AntinodeIterator {
    pub fn new(pos1: Position, pos2: Position, width: i8, height: i8) -> Self {
        AntinodeIterator {
            delta: pos1.clone() - pos2.clone(),
            current: pos1.clone(),
            width,
            height
        }
    }
}
impl Iterator for AntinodeIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.current - self.delta;
        match 0 <= self.current.x && self.current.x < self.width && 0 <= self.current.y && self.current.y < self.height {
            true => Some(self.current),
            false => None,
        }
    }
}
