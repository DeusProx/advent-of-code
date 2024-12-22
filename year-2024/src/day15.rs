use core::panic;
use std::{collections::HashSet, fmt::Write, iter, time::Instant};
use colored::*;

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/15/input").expect("Cannot read input");

    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::parse(map);
    let mut robot = Robot::parse(moves, warehouse.find_robot());

    robot.run(&mut warehouse);

    warehouse.gps(&'O')
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/15/input").expect("Cannot read input");

    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::parse(map);
    warehouse.expand();
    let mut robot = Robot::parse(moves, warehouse.find_robot());

    robot.run(&mut warehouse);
    warehouse.gps(&'[')
}

pub struct Robot {
    position: Point,
    pub moves: Vec<char>,
}
impl Robot {
    pub fn parse(input: &str, position: Point) -> Self {
        let moves =  input.chars().filter(|c| *c != '\n').collect();
        Self { position, moves }
    }
    pub fn run(&mut self, warehouse: &mut Warehouse) {
        for direction in self.moves.clone().iter() {
            self.walk(warehouse, *direction);
        }
    }
    pub fn walk(&mut self,warehouse: &mut Warehouse, direction: char) {
        let next = self.position.next(direction);
        let field = warehouse.field(&next);
        match field {
            '#' => (),
            'O' => match self.push_box(warehouse, &next, direction) {
                false => (),
                true => {
                    warehouse.switch(&self.position, &next);
                    self.position = next;
                },
            },
            '.' => {
                warehouse.switch(&self.position, &next);
                self.position = next;
            },
            '[' | ']' => {
                // normalize position to the left of the bracket
                let left_bracket = match field {
                    '[' => next.clone(),
                    ']' => next.next('<'),
                    _ => panic!("in the disco"),
                };

                match self.push_big_box(warehouse, &left_bracket, direction)  {
                    None => (),
                    Some(elements_to_switch) => {
                        // remember already switched fields (diamond shape handling)
                        let mut visited = HashSet::with_capacity(elements_to_switch.len());
                        for ele in elements_to_switch.iter() {
                            if visited.contains(ele) {
                                continue
                            }
                            visited.insert(ele);
                            warehouse.switch(&ele.0, &ele.1);
                        }

                        warehouse.switch(&self.position, &next);
                        self.position = next;
                    },
                }
            }
            _ => panic!("at the disco"),
        };
    }
    fn push_box(&mut self, warehouse: &mut Warehouse, position: &Point, direction: char) -> bool {
        let next = position.next(direction);
        match warehouse.field(&next) {
            '#' => return false,
            'O' => match self.push_box(warehouse, &next, direction) {
                false => return false,
                true => {
                    warehouse.switch(&position, &next);
                    return true
                }
            },
            '.' => {
                warehouse.switch(&position, &next);
                return true;
            },
            _ => panic!("at the disco"),
        }
    }
    fn push_big_box(&mut self, warehouse: &mut Warehouse, left: &Point, direction: char) -> Option<Vec<(Point, Point)>> {
        let right = left.next('>');
        let next_left = left.next(direction);
        let next_right = right.next(direction);

        let (l, r) = (warehouse.field(&next_left).clone(), warehouse.field(&next_right).clone());

        // There is alot of repetition so this might could be improved, but it's good for now
        match (direction, l, r) {
            (_, '#', _) | (_, _, '#') => return None,
            (_, '.', '.') => return Some(vec![(left.clone(), next_left), (right, next_right)]),
            ('<', '.', _) => return Some(vec![(left.clone(), next_left), (right, next_right)]),
            ('>', _, '.') => return Some(vec![(right, next_right), (left.clone(), next_left)]),
            ('<', '[', _) => return self.push_big_box(warehouse, &next_left, direction).and_then(|mut acc| {
                acc.append(&mut vec![(left.clone(), next_left), (right, next_right)]);
                Some(acc)
            }),
            ('<', ']', '[') => return self.push_big_box(warehouse, &next_left.next('<'), direction).and_then(|mut acc| {
                acc.append(&mut vec![(left.clone(), next_left), (right, next_right)]);
                Some(acc)
            }),
            ('>', _, '[') => return self.push_big_box(warehouse, &next_right, direction).and_then(|mut acc| {
                acc.append(&mut vec![(right, next_right), (left.clone(), next_left)]);
                Some(acc)
            }),
            ('^', '[', ']') | ('v', '[', ']') => return self.push_big_box(warehouse, &next_left, direction).and_then(|mut acc| {
                acc.append(&mut vec![(left.clone(), next_left), (right, next_right)]);
                Some(acc)
            }),
            ('^', ']', _) | ('^', _, '[') | ('v', ']', _) | ('v', _, '[') => {
                let l = (l == '.')
                    .then_some(vec![(left.clone(), next_left.clone())])
                    .or(self.push_big_box(warehouse, &next_left.next('<'), direction).and_then(|mut acc| {
                        acc.push((left.clone(), next_left));
                        Some(acc)
                    })
                );
                let r = (r == '.')
                    .then_some(vec![(right.clone(), next_right.clone())])
                    .or(self.push_big_box(warehouse, &next_right, direction).and_then(|mut acc| {
                        acc.push((right, next_right));
                        Some(acc)
                    })
                );
                return l.and_then(|mut a| r.map(|mut b| {
                    a.append(&mut b);
                    a
                }));
            },
            _ => { panic!("in the disco") },
        }
    }
}

static BROWN: colored::CustomColor = CustomColor { r: 185, g: 156, b: 107 };

pub struct Warehouse {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}
impl Warehouse {
    pub fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let height = input.len() / width;
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self { map, width, height }
    }
    pub fn field(&self, point: &Point) -> &char {
        &self.map[point.y][point.x]
    }
    pub fn switch(&mut self, a: &Point, b: &Point) {
        let tmp = self.map[a.y][a.x];
        self.map[a.y][a.x] = self.map[b.y][b.x];
        self.map[b.y][b.x] = tmp;
    }
    pub fn find_robot(&self) -> Point {
        (0..self.height - 1)
            .flat_map(|y|
                (0..self.width - 1)
                    .zip(iter::repeat_n(y, self.width))
                    .map(Point::new)
            )
            .find(|point| self.map[point.y][point.x] == '@')
            .unwrap()
    }
    pub fn expand(&mut self) {
        self.width *= 2;
        self.map = self.map
            .iter()
            .map(|line| line.iter()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => panic!("at the disco"),
                })
                .collect()
            )
            .collect();
    }
    pub fn printable(&self) -> impl Iterator<Item=String> {
        self.map.iter()
            .map(|line| line.iter()
                .map(|c| {
                    let colored = match c {
                        &'@' => "@".color("red").to_string(),
                        &'.' => ".".color("grey").to_string(),
                        &'O' => "O".custom_color(BROWN).to_string(),
                        &'[' => "[".custom_color(BROWN).to_string(),
                        &']' => "]".custom_color(BROWN).to_string(),
                        &'#' => "#".color("black").on_color("grey").to_string(),
                        _ => panic!("in the disco"),
                    };
                    colored.chars().collect::<Vec<char>>()
                })
                .flatten()
                .collect()
            )
            .into_iter()
    }
    pub fn to_string(&self) -> String {
        self.printable()
            .map(|mut line| {
                line.write_char('\n').ok();
                line
            })
            .collect::<String>()
    }
    pub fn gps(&self, box_identifier: &char) -> u32 {
        (0..self.height)
        .flat_map(|y| (0..self.width).zip(iter::repeat_n(y, self.width)))
        .map(Point::new)
        .filter(|point| self.field(point) == box_identifier)
        .map(|p| p.gps())
        .sum()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
    pub fn next(&self, direction: char) -> Self {
        match direction {
            '^' => Point { x: self.x,     y: self.y - 1 },
            'v' => Point { x: self.x,     y: self.y + 1 },
            '<' => Point { x: self.x - 1, y: self.y },
            '>' => Point { x: self.x + 1, y: self.y },
            _ => panic!("at the disco"),
        }
    }
    pub fn gps(&self) -> u32 {
        (100 * self.y + self.x) as u32
    }
}
