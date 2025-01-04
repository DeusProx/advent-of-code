use std::{collections::{VecDeque, HashMap}, time::Instant};
use rayon::prelude::*;

fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/16/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert!(result == 7493);
}

#[derive(Debug, Default)]
struct Grid<T> {
    elements: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn get(&self, coords: &Coords) -> Option<&T> {
        let index = coords.y * (self.width + 1) + coords.x;
        self.elements.get(index)
    }
}
impl Grid<char> {
    fn parse(input: &str) -> Self {
        let mut grid: Grid<char> = Grid::default();

        grid.elements = input.chars().collect::<Vec<char>>();
        grid.width = input.find("\n").unwrap();
        grid.height = input.len() / (input.find("\n").unwrap() + 1);

        grid
    }
}

// HINT:
//   Solution would probably have been much shorter to use when...
//     - implementing Coords as tuple
//     - implementing Direction as Coords tuple
//     - simply mirroring coords according to mirror

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    coords: Coords,
    direction: Direction,
}

impl Beam {
    fn next<T>(mut self, grid: &Grid<T>, new_direction: &Direction) -> Option<Self> {
        self.direction = *new_direction;
        let coords =  match self.direction {
            Direction::North => match self.coords.y > 0 {
                true => Some(Coords { x: self.coords.x, y: self.coords.y - 1 }),
                false => None,
            },
            Direction::South => match self.coords.y < grid.height - 1 {
                true => Some(Coords { x: self.coords.x, y: self.coords.y + 1 }),
                false => None,
            },
            Direction::West => match self.coords.x > 0 {
                true => Some(Coords { x: self.coords.x - 1, y: self.coords.y }),
                false => None,
            },
            Direction::East => match self.coords.x < grid.width - 1 {
                true => Some(Coords { x: self.coords.x + 1, y: self.coords.y }),
                false => None,
            },
        };
        match coords {
            None => None,
            Some(coords) => {
                self.coords = coords;
                Some(self)
            }
        }
    }
}

fn calc(input: String) -> usize {
    let now = Instant::now();

    let grid: Grid<char> = Grid::parse(&input);
    let mut starting_points: Vec<Beam> = Vec::new();

    for x in 0..grid.width {
        starting_points.push(Beam {
            coords: Coords { x, y: grid.height - 1 },
            direction: Direction::North,
        });
        starting_points.push(Beam {
            coords: Coords { x, y: 0 },
            direction: Direction::South,
        });
        // highest = highest.max(propagate(&grid, start_north));
        // highest = highest.max(propagate(&grid, start_south));
    };
    for y in 0..grid.height {
        starting_points.push(Beam {
            coords: Coords { y, x: grid.width - 1 },
            direction: Direction::West,
        });
        starting_points.push(Beam {
            coords: Coords { y, x: 0 },
            direction: Direction::East,
        });
        // highest = highest.max(propagate(&grid, start_west));
        // highest = highest.max(propagate(&grid, start_east));
    };

    let highest = starting_points.par_iter()
        .map(|start_beam| propagate(&grid, *start_beam))
        .max();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    highest.unwrap()
}

fn propagate(grid: &Grid<char>, initial_beam: Beam) -> usize {
    let mut visited: HashMap<Beam, bool> = HashMap::new();
    let mut propagation: Vec<Direction> = Vec::default();
    let mut stack = VecDeque::default();

    stack.push_back(initial_beam);

    loop {
        let beam = match stack.pop_front() {
            None => break,
            Some(beam) => beam,
        };

        match visited.get(&beam) {
            Some(_) => continue,
            None => visited.insert(beam, true),
        };

        let el = match grid.get(&beam.coords) {
            None => continue,
            Some(el) => el,
        };

        // Be cautious when looking at these pattern matching
        // We use the direction saved in the beam, so we need to inverse it's direction for the
        // mirrors
        // e.g. a beam traveling to the East and hitting a "/" mirror will go to North
        match el {
            '.' => propagation.push(beam.direction),
            '\\' => {
                let new_direction = match beam.direction {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                };
                propagation.push(new_direction);
            },
            '/' => {
                let new_direction = match beam.direction {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                    Direction::East => Direction::North,
                };
                propagation.push(new_direction);
            },
            '|' => {
                match beam.direction {
                    Direction::North => propagation.push(Direction::North),
                    Direction::South => propagation.push(Direction::South),
                    Direction::West | Direction::East => {
                        propagation.push(Direction::North);
                        propagation.push(Direction::South);
                    },
                };
            },
            '-' => {
                match beam.direction {
                    Direction::North | Direction::South => {
                        propagation.push(Direction::West);
                        propagation.push(Direction::East);
                    },
                    Direction::West => propagation.push(Direction::West),
                    Direction::East => propagation.push(Direction::East),
                };
            },
            _ => panic!("NOOOOOO!"),
        }

        for next_direction in propagation.iter() {
            let new_beam = beam.next(&grid, next_direction);
            match new_beam {
                Some(new_beam) => stack.push_back(new_beam),
                None => ()
            };
        }
        propagation.clear()
    }

    let visited_elements = visited
        .keys()
        .fold(HashMap::new(), |mut acc, beam| {
            acc.entry(beam.coords).or_insert(true);
            acc
        })
        .len();

    visited_elements
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/16/testinput").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 51);
    }
}

