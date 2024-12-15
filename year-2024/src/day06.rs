use core::panic;
use std::{collections::HashSet, fmt::Display, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/6/input").expect("Cannot read input");

    let mut guard = Guard::parse(&input);
    guard.run();
    // println!("Lab:\n{}", guard.lab);

    guard.lab.visited_count()
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/6/input").expect("Cannot read input");

    let mut places: HashSet<usize> = HashSet::with_capacity(1500);
    let mut guard = Guard::parse(&input);

    while guard.walk().is_some() {
        let mut clone = guard.clone();
        if let Some(position) = clone.next_position() {
            if D.contains(&clone.lab.fields[position]) || clone.lab.fields[position] == '#' {
                continue;
            }

            clone.lab.fields[position] = 'O';

            if clone.run() == Run::Loop {
                places.insert(position);
                // println!("Lab Clone:\n{}\n", guard.lab);
            }
        }
    }

    places.len() as u32
}

#[derive(Debug, Clone)]
pub struct Lab {
    fields: Vec<char>,
    width: usize,
    height: usize,
}
impl Lab {
    pub fn visited_count(&self) -> u32 {
        self.fields.iter().filter(|&c| D.contains(c)).count() as u32
    }
}
impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.fields
            .chunks(self.width)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", output)
    }
}

#[derive(PartialEq)]
pub enum Run {
    Loop,
    OutOfBounds,
}

#[derive(Debug, Clone)]
pub struct Guard {
    steps: usize,
    pub start_position: usize,
    position: usize,
    direction: Direction,
    pub lab: Lab

}
impl Guard {
    pub fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let height = input.len() / (width + 1);
        let fields: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        let lab = Lab {
            fields,
            width,
            height,
        };

        let guard_position = lab.fields.iter().position(|c| D.contains(c)).unwrap();
        let guard_direction = Direction::parse(lab.fields[guard_position]);

        Guard {
            steps: 0,
            start_position: guard_position.clone(),
            position: guard_position,
            direction: guard_direction,
            lab
        }
    }
    pub fn run(&mut self) -> Run {
        loop {
            match self.walk() {
                None => return Run::OutOfBounds,
                Some(_) if self.lab.fields[self.position] == self.direction.char() => {
                    return Run::Loop
                },
                _ => (),
            }
        }
    }
    pub fn walk(&mut self) -> Option<()> {
        match self.next_position() {
            None => {
                self.lab.fields[self.position] = self.direction.char();
                return None
            },
            Some(position) => match self.lab.fields[position] {
                '#' | 'O' => { self.direction.right()},
                '.' |  '^' | 'v' | '<' | '>' => {
                    self.steps += 1;
                    self.lab.fields[self.position] = self.direction.char();
                    self.position = position;
                },
                _ => panic!("panic in the disco"),
            }
        };

        Some(())
    }
    pub fn next_position(& self) -> Option<usize> {
        match (&self.direction, self.position) {
            (Direction::North, position) => match position / self.lab.width > 0 {
                true => Some(position - self.lab.width),
                false => None,
            },
            (Direction::South, position) => match position / self.lab.width < self.lab.height - 1 {
                true => Some(position + self.lab.width),
                false => None,
            },
            (Direction::East, position) => match position % self.lab.width < self.lab.width - 1 {
                true => Some(position + 1),
                false => None,
            },
            (Direction::West, position) => match position % self.lab.width > 0 {
                true => Some(position - 1),
                false => None,
            }
        }
    }
}

static D: [char; 4] = [ '^', 'v', '<', '>' ];

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    pub fn parse(c: char) -> Self {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("panic in the disco"),
        }
    }
    pub fn char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East =>  '>',
            Direction::West  => '<',
        }
    }
    pub fn right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

