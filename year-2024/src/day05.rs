use std::{ops::{Deref, DerefMut}, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/5/input").expect("Cannot read input");

    let mut queue = PrintQueue::parse(&input);
    queue.check();

    queue.middles()
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/5/input").expect("Cannot read input");

    let mut queue = PrintQueue::parse(&input);
    queue.check();
    let initial = queue.middles();
    queue.correct();

    queue.middles() - initial
}

struct Rules(Vec<Vec<bool>>);
impl Deref for Rules {
    type Target = Vec<Vec<bool>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}


#[derive(Debug, Clone)]
pub enum Update {
    Uncorrected(Vec<u8>),
    Correct(Vec<u8>),
    Incorrect(Vec<u8>),
    Corrected(Vec<u8>, Vec<u8>),
}
impl Deref for Update {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        match self {
            Update::Uncorrected(uncorrected) => uncorrected,
            Update::Correct(correct) => correct,
            Update::Incorrect(incorrect) => incorrect,
            Update::Corrected(fixed, _initial_incorrect) => fixed,
        }
    }
}
impl FromIterator<u8> for Update {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Update::Uncorrected(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone)]
struct Updates(Vec<Update>);
impl Deref for Updates {
    type Target = Vec<Update>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Updates {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

struct PrintQueue(Rules, Updates);
impl PrintQueue {
    pub fn parse(input: &str) -> Self {
        let (rule_section, page_updates_section) = input.split_once("\n\n").unwrap();
        let rules: Vec<Vec<bool>> = rule_section.lines()
            .map(|line|
                line.split("|")
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            )
            .fold(
                vec![vec![false; 100]; 100],
                |mut map, numbers| {
                    map[numbers[0] as usize][numbers[1] as usize] = true;
                    map
                },
            );
        let updates: Vec<Update> = page_updates_section.lines()
            .map(|line| line.split(",")
                .map(|number| number.parse::<u8>().unwrap())
                .collect::<Update>()
            )
            .collect();
        Self(Rules(rules), Updates(updates))
    }
    pub fn check(&mut self) {
        for update in self.1.iter_mut() {
            let correct = update.iter()
                .enumerate()
                .all(|(index, &left)| update.iter()
                    .take(index)
                    .all(|&right| self.0[right as usize][left as usize])
                );
            *update = match correct {
                true => Update::Correct(update.to_vec()),
                false => Update::Incorrect(update.to_vec()),
            }
        }
    }
    pub fn correct(&mut self) {
        for update in self.1.iter_mut() {
            if let Update::Incorrect(data) = update {
                let data_copy = data.to_vec();
                let mut corrected: Vec<u8> = Vec::with_capacity(data.len());

                while !data.is_empty() {
                    let index = data.iter().enumerate()
                        .find_map(|(index, &left)| {
                            let blocked = data.iter().any(|&right| self.0[right as usize][left as usize]);
                            match !blocked {
                                true => Some(index),
                                false => None,
                            }
                        }).unwrap();

                    corrected.push(data.remove(index));
                }

                *update = Update::Corrected(corrected, data_copy);
            }
        }

    }
    pub fn middles(&self) -> u32 {
        self.1.iter()
            .filter_map(|update| match update {
                Update::Correct(numbers) => Some(numbers),
                Update::Corrected(numbers, _) => Some(numbers),
                _ => None,
            })
            .map(|update| update[update.len() / 2] as u32)
            .sum()
    }
}

