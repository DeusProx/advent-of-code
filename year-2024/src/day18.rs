use std::{collections::VecDeque, fmt::Display, time::Instant, usize};

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/18/input").expect("Cannot read input");
    let map = Map::parse(&input, 71, 1024);

    map.solve().unwrap()
}

#[aoc_macro::bench()]
pub fn part2() -> String {
    // TODO: macro

    let input = std::fs::read_to_string("../data/2024/day/18/input").expect("Cannot read input");
    let mut max = input.lines().count();
    let mut min = 1014;

    /* Iterating (slow; several ms)
    let mut i = 1024;
    while Map::parse(&input, 71, pivot).solve().is_some() {
        if pivot < max { pivot+= 1; }
    }
    pivot -= 1;
    */

    // binary log search to go below 1 ms
    let mut pivot = (min + max) / 2;
    loop {
        if pivot == min || pivot == max {
            break;
        }
        match Map::parse(&input, 71, pivot).solve() {
            Some(_) => {
                min = pivot;
                pivot = (min + max) / 2;
            },
            None => {
                max = pivot;
                pivot = (min + max) / 2;
            }
        }
    }

    input.lines().nth(pivot).unwrap().to_string()
}

#[derive(Clone, PartialEq, Eq)]
enum Field {
    Safe,
    Corrupted,
}

struct Map {
    size: usize,
    fields: Vec<Vec<Field>>
}
impl Map {
    fn parse(input: &str, size: usize, kbs: usize) -> Self {
        let size = size + 2;
        let mut fields: Vec<Vec<Field>> = input.lines()
            .take(kbs)
            .fold(vec![vec![Field::Safe; size]; size], |mut fields, line| {
                let (x, y) = line.split_once(',').unwrap();
                fields[y.parse::<usize>().unwrap() + 1][x.parse::<usize>().unwrap() + 1] = Field::Corrupted;
                fields
            });

        for a in 0..size {
            fields[a][0] = Field::Corrupted;
            fields[a][size - 1] = Field::Corrupted;
            fields[0][a] = Field::Corrupted;
            fields[size - 1][a] = Field::Corrupted;
        }

        Self { size, fields }
    }
    fn solve(&self) -> Option<usize> {
        let start: (usize, usize) = (1, 1);
        let end: (usize, usize) = (self.size - 2, self.size - 2);

        let mut costs = vec![vec![usize::MAX; self.size]; self.size];
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::with_capacity(100);
        queue.push_front((start, 0));

        while let Some(((x, y), step)) = queue.pop_front() {
            if costs[y][x] <= step {
                continue;
            }
            costs[y][x] = step;

            if end.0 == x && end.1 == y {
                continue;
            }

            [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ].iter()
                .filter(|(x, y)| self.fields[*y][*x] == Field::Safe)
                .for_each(|&pos| queue.push_back((pos, step + 1)));

        }
        let steps = costs[end.1][end.0];
        match steps != usize::MAX {
            true => Some(steps),
            false => None,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = self.fields.iter()
            .map(|col| col.iter()
                .map(|f| match f {
                    Field::Safe => '.',
                    Field::Corrupted => '#',
                })
                .collect::<String>()
            )
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&printable)
    }
}
