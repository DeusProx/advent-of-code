use std::{cmp::Ordering, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro

    let input = std::fs::read_to_string("../data/2024/day/14/input").expect("Cannot read input");
    let size = Size { width: 101, height: 103 };

    input.lines()
        .map(|line| Robot::parse(line, &size))
        .map(|robot| robot.teleport(&size, 100))
        .filter_map(|position| position.quadrant(&size))
        .fold(vec![0; 4], |mut quadrants, quadrant| {
            match quadrant {
                Quadrant::TL => quadrants[0] += 1,
                Quadrant::TR => quadrants[1] += 1,
                Quadrant::BL => quadrants[2] += 1,
                Quadrant::BR => quadrants[3] += 1,
            };
            quadrants
        })
        .iter()
        .product()
}

impl Robot {
    pub fn parse(input: &str, size: &Size) -> Self {
            let mut iter = input
                .split(|c| ['p', 'v', '=', ',', ' '].contains(&c))
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i32>().unwrap());

            Robot {
                p: Position {
                    x: iter.next().unwrap().rem_euclid(size.width as i32) as u32,
                    y: iter.next().unwrap().rem_euclid(size.height as i32) as u32,
                },
                v: Velocity {
                    x: iter.next().unwrap().rem_euclid(size.width as i32) as u32,
                    y: iter.next().unwrap().rem_euclid(size.height as i32) as u32,
                }
            }
    }
    pub fn teleport(&self, size: &Size, dt: u32) -> Position {
        Position {
            x: (self.p.x + dt * self.v.x).rem_euclid(size.width),
            y: (self.p.y + dt * self.v.y).rem_euclid(size.height),
        }
    }
}

impl Position {
    pub fn quadrant(&self, size: &Size) -> Option<Quadrant> {
        match (self.x.cmp(&(size.width / 2)), self.y.cmp(&(size.height / 2))) {
            (Ordering::Less,    Ordering::Less)    => Some(Quadrant::TL),
            (Ordering::Greater, Ordering::Less)    => Some(Quadrant::TR),
            (Ordering::Less,    Ordering::Greater) => Some(Quadrant::BL),
            (Ordering::Greater, Ordering::Greater) => Some(Quadrant::BR),
            (_, _) => None,
        }
    }
    pub fn index(&self, size: &Size) -> usize {
        (self.y * size.width + self.x) as usize
    }
}

#[derive(Debug)]
pub enum Quadrant {
    TL, // Top Left
    TR, // Top Right
    BL, // Bottom Left
    BR, // Bottom Right
}

#[derive(Debug, Clone)]
pub struct Robot {
    p: Position,
    v: Velocity,
}
#[derive(Debug, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}
#[derive(Debug, Clone)]
struct Velocity {
    x: u32,
    y: u32,
}
#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
