use std::collections::HashMap;
use std::time::Instant;
use std::iter;

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    let input = std::fs::read_to_string("../data/2024/day/21/input").expect("Cannot read input");

    let nkeys: Vec<Vec<NKey>> = input.lines()
        .map(|line|
            [NKey::A].into_iter()
                .chain(line.chars().map(NKey::from))
                .collect::<Vec<NKey>>()
        )
        .collect();

    let dkeys: Vec<Vec<DKey>> = nkeys.iter()
        .map(|line| {
            let movement_sequence = line
                .windows(2)
                .map(|a| a[0].move_to(a[1]))
                .flatten()
                .into_iter();

            [DKey::A].into_iter()
                .chain(movement_sequence)
                .collect::<Vec<DKey>>()
        })
        .collect();

    let dkeys: Vec<Vec<DKey>> = dkeys.iter()
        .map(|line| {
            let movement_sequence = line
                .windows(2)
                .map(|a| a[0].move_to(a[1]))
                .flatten()
                .into_iter();

            [DKey::A].into_iter()
                .chain(movement_sequence)
                .collect::<Vec<DKey>>()
        })
        .collect();

    let dkeys: Vec<Vec<DKey>> = dkeys.iter()
        .map(|line| line
            .windows(2)
            .map(|a| a[0].move_to(a[1]))
            .flatten()
            .collect::<Vec<DKey>>()
        )
        .collect();

    input.lines()
        .zip(dkeys.iter())
        .map(|(a, b)| a[0..3].parse::<u32>().unwrap() * b.len() as u32)
        .sum()
}

trait MoveTo<Rhs=Self> {
    // find shortest sequence of directional keys to be used to move from one button to another
    fn move_to(self, target_key: Self) -> impl Iterator<Item=DKey>;
}

/**
*  numeric keypad:
*
*   +---+---+---+
*   | 7 | 8 | 9 |
*   +---+---+---+
*   | 4 | 5 | 6 |
*   +---+---+---+
*   | 1 | 2 | 3 |
*   +---+---+---+
*       | 0 | A |
*       +---+---+
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NKey { A, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9 }

impl NKey {
    fn get_x(&self) -> i8 {
        match self {
                       NKey::K1 | NKey::K4 | NKey::K7 => 0, // left
            NKey::K0 | NKey::K2 | NKey::K5 | NKey::K8 => 1, // middle
            NKey::A  | NKey::K3 | NKey::K6 | NKey::K9 => 2, // right
        }
    }
    fn get_y(&self) -> i8 {
        match self {
            NKey::K7 | NKey::K8 | NKey::K9 => 3,            // top
            NKey::K4 | NKey::K5 | NKey::K6 => 2,
            NKey::K1 | NKey::K2 | NKey::K3 => 1,
                       NKey::K0 | NKey::A  => 0,            // bottom
        }
    }
}

impl MoveTo for NKey {
    fn move_to(self, target_key: NKey) -> impl Iterator<Item=DKey> {
        let (x_1, y_1) = (self.get_x(), self.get_y());
        let (x_2, y_2) = (target_key.get_x(), target_key.get_y());
        let (x_d, y_d) = (x_1 - x_2, y_1 - y_2);

        type Sequence = Box<dyn Iterator<Item=DKey>>;

        // "Cannot move over empty space" logic
        if (x_1 == 0 && y_2 == 0) || (x_2 == 0 && y_1 == 0) {
            if x_d < 0 {
                let movement_sequence = iter::empty::<DKey>()
                    .chain(iter::repeat(DKey::R).take(x_d.abs() as usize))
                    .chain(iter::repeat(DKey::D).take(y_d.abs() as usize))
                    .chain([DKey::A].into_iter());
                return Box::new(movement_sequence) as Sequence
            }
            if x_d > 0 {
                let movement_sequence = iter::empty::<DKey>()
                    .chain(iter::repeat(DKey::U).take(y_d.abs() as usize))
                    .chain(iter::repeat(DKey::L).take(x_d.abs() as usize))
                    .chain([DKey::A].into_iter());
                return Box::new(movement_sequence) as Sequence
            }
        }

        // General movement logic (movement order: <, v, ^, >)
        let movement_sequence = [
            (x_d > 0).then(|| iter::repeat(DKey::L).take(x_d.abs() as usize)),
            (y_d > 0).then(|| iter::repeat(DKey::D).take(y_d.abs() as usize)),
            (y_d < 0).then(|| iter::repeat(DKey::U).take(y_d.abs() as usize)),
            (x_d < 0).then(|| iter::repeat(DKey::R).take(x_d.abs() as usize)),
        ]
            .into_iter()
            .flatten()
            .flatten()
            .chain([DKey::A].into_iter());

        Box::new(movement_sequence) as Sequence
    }
}

/**
* directional keypad:
*
*       +---+---+
*       | ^ | A |
*   +---+---+---+
*   | < | v | > |
*   +---+---+---+
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DKey { A, U, D, L, R }

impl MoveTo for DKey {
    // movement order: <, v, ^, >
    fn move_to(self, target_key: DKey) -> impl Iterator<Item=DKey> {
        let movement_sequence = match (self, target_key) {
            (DKey::A, DKey::A) => vec![],
            (DKey::A, DKey::U) => vec![DKey::L],
            (DKey::A, DKey::R) => vec![DKey::D],
            (DKey::A, DKey::D) => vec![DKey::L, DKey::D],
            (DKey::A, DKey::L) => vec![DKey::D, DKey::L, DKey::L],

            (DKey::U, DKey::A) => vec![DKey::R],
            (DKey::U, DKey::U) => vec![],
            (DKey::U, DKey::R) => vec![DKey::D, DKey::R],
            (DKey::U, DKey::D) => vec![DKey::D],
            (DKey::U, DKey::L) => vec![DKey::D, DKey::L],

            (DKey::R, DKey::A) => vec![DKey::U],
            (DKey::R, DKey::U) => vec![DKey::L, DKey::U],
            (DKey::R, DKey::R) => vec![],
            (DKey::R, DKey::D) => vec![DKey::L],
            (DKey::R, DKey::L) => vec![DKey::L, DKey::L],

            (DKey::D, DKey::A) => vec![DKey::U, DKey::R],
            (DKey::D, DKey::U) => vec![DKey::U],
            (DKey::D, DKey::R) => vec![DKey::R],
            (DKey::D, DKey::D) => vec![],
            (DKey::D, DKey::L) => vec![DKey::L],

            (DKey::L, DKey::A) => vec![DKey::R, DKey::R, DKey::U],
            (DKey::L, DKey::U) => vec![DKey::R, DKey::U],
            (DKey::L, DKey::R) => vec![DKey::R, DKey::R],
            (DKey::L, DKey::D) => vec![DKey::R],
            (DKey::L, DKey::L) => vec![],
        };
        // click the button
        movement_sequence.into_iter().chain([DKey::A].into_iter())
    }
}

impl From<char> for NKey {
    fn from(value: char) -> Self {
        match value {
            'A' => NKey::A,
            '0' => NKey::K0,
            '1' => NKey::K1,
            '2' => NKey::K2,
            '3' => NKey::K3,
            '4' => NKey::K4,
            '5' => NKey::K5,
            '6' => NKey::K6,
            '7' => NKey::K7,
            '8' => NKey::K8,
            '9' => NKey::K9,
            _ => panic!("in the disco"),
        }
    }
}

#[aoc_macro::bench()]
pub fn part2() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/21/input").expect("Cannot read input");

    let nkeys: Vec<Vec<NKey>> = input.lines()
        .map(|line|
            [NKey::A].into_iter()
                .chain(line.chars().map(NKey::from))
                .collect::<Vec<NKey>>()
        )
        .collect();

    let dkeys: Vec<Vec<DKey>> = nkeys.iter()
        .map(|line| {
            let movement_sequence = line
                .windows(2)
                .map(|a| a[0].move_to(a[1]))
                .flatten()
                .into_iter();
            [DKey::A].into_iter()
                .chain(movement_sequence)
                .collect::<Vec<DKey>>()
        })
        .collect();

    let dkeys: Vec<usize> = min_seq_len(&dkeys, 25);

    input.lines()
        .map(|schematics| schematics[0..3].parse::<usize>().unwrap())
        .zip(dkeys.iter())
        .map(|(a, b)| a * b)
        .sum()
}

// Gets the length of the shortest sequence
fn min_seq_len(dkeys: &Vec<Vec<DKey>>, level: usize) -> Vec<usize> {
    let mut maps: Vec<HashMap<(DKey, DKey), usize>> = vec![HashMap::with_capacity(25); level];
    let dkeys: Vec<usize> = dkeys.into_iter()
        .map(|line|
            line.into_iter()
                .map_windows(|[a, b]| min_seq_len_rec((*a).clone(), (*b).clone(), level, &mut maps))
                .sum::<usize>()
        )
        .collect();

    dkeys
}

// Calculates the length of the shortest sequence between two DKeys in a translation chain
fn min_seq_len_rec(curr: DKey, other: DKey, level: usize, maps: &mut Vec<HashMap<(DKey, DKey), usize>>) -> usize {
    let level = match level {
        0 => return 1,
        level => level - 1,
    };

    let hashkey = (curr, other);
    if let Some(&i) = maps.get(level).unwrap().get(&hashkey) {
        return i;
    }

    let sequence_len: usize = [DKey::A].into_iter()
        .chain(curr.move_to(other))
        .map_windows(|&[a, b]| min_seq_len_rec(a, b, level, maps))
        .sum();
    let sequence_len = sequence_len;

    maps.get_mut(level).unwrap().insert(hashkey, sequence_len);

    sequence_len
}

