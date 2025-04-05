use std::time::Instant;

/**
 * Builds iterators from all fields in all directions
 * then searches for "XMAS"
 *
 * takes 5023 μs (~5 ms)
 */
#[aoc_macro::bench()]
pub fn part1_first_try() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/4/input").expect("Cannot read input");
    let wordsearch = Wordsearch::parse(&input);
    wordsearch.search("XMAS", Direction::all()).count()
}

/**
 * Builds iterators from all fields in diagonal directions
 * then searches for "MAS" in all diagonal directions
 * then try to find pairs for the "A" field
 *
 * takes 2666 μs (~2 ms)
 */
#[aoc_macro::bench()]
pub fn part2_first_try() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/4/input").expect("Cannot read input");

    let wordsearch = Wordsearch::parse(&input);
    let mut matches = wordsearch.search("MAS", Direction::diagonals())
        .filter(|(_, direction)| direction.diagonal())
        .map(|(start_field, direction)| start_field.mov(direction)) // take the middle "A" field
        .collect::<Vec<Field>>();

    matches.sort(); // array needs to be sorted for partition_dedup to work
    let (_, duplicates) = matches.partition_dedup();

    duplicates.len()
}

struct Wordsearch {
    height: usize,
    width: usize,
    fields: Vec<char>,
}
impl Wordsearch {
    pub fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let height = input.lines().count();
        let fields = input.chars().filter(|c| !c.is_whitespace()).collect();

        Self { height, width, fields }
    }
    pub fn get(&self, field: Field) -> Option<&char> {
        let x = usize::try_from(field.x).ok()?;
        let y = usize::try_from(field.y).ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.fields.get(index)
    }
    pub fn search(&self, search_word: &str, directions: Vec<Direction>) -> impl std::iter::Iterator<Item = (Field, Direction)> {
        (0..(self.height as i16)).into_iter()
            .flat_map(|y| (0..(self.width as i16)).into_iter()
                .map(move |x| Field { x, y })
            )
            .flat_map(move |start_field|
                directions.iter()
                    .filter_map(move |direction| {
                        let word = FieldIterator::new(start_field, direction.clone())
                            .take(search_word.len())
                            .filter_map(|field| self.get(field))
                            .collect::<Vec<&char>>();
                        if word.len() != search_word.len() {
                            return None;
                        }
                        let equal = search_word.chars()
                            .zip(word.clone().into_iter())
                            .all(|(a, b)| a == *b);
                        if !equal {
                            return None;
                        }
                        Some((start_field, direction.clone()))
                    })
                    .collect::<Vec<(Field, Direction)>>()
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    y: i16,
    x: i16,
}

impl Field {
    fn mov(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();
        Field {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

pub struct FieldIterator {
    next: Field,
    direction: Direction,
}

impl FieldIterator {
    fn new(start_field: Field, direction: Direction) -> Self {
        FieldIterator {
            next: start_field,
            direction,
        }
    }
}
impl Iterator for FieldIterator {
    type Item = Field;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = self.next.mov(self.direction);
        Some(current)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    N, S, E, W,
    NE, NW, SE, SW,
}

impl Direction {
    pub fn delta(self) -> (i16, i16) {
        match self {
            Direction::N =>  ( 0, -1),
            Direction::S =>  ( 0,  1),
            Direction::E =>  ( 1,  0),
            Direction::W =>  (-1,  0),
            Direction::NE => ( 1, -1),
            Direction::NW => (-1, -1),
            Direction::SE => ( 1,  1),
            Direction::SW => (-1,  1),
        }
    }
    pub fn diagonal(self) -> bool {
        matches!(self, Direction::NE | Direction::NW | Direction::SE | Direction::SW)
    }
    pub fn all() -> Vec<Self>  {
        vec![
            Direction::N,  Direction::S,  Direction::E,  Direction::W,
            Direction::NE, Direction::NW, Direction::SE, Direction::SW,
        ]
    }
    pub fn diagonals() -> Vec<Self>  {
        Direction::all().into_iter()
            .filter(|&dir| dir.diagonal())
            .collect::<Vec<Direction>>()
    }
}

// Arrays to match to find the patterns
// I wish it would be possible to give reversed ranges to the "get"-method of a Vec so we would only need one here
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];

/**
 *  Search for "XMAS" in all directions directly in the input
 *
 *  takes 121 μs
 */
#[aoc_macro::bench()]
fn part1() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/4/input").expect("Cannot read input");
    let width = input.find('\n').unwrap() + 1;
    let input: Vec<char> = input.chars().collect();
    println!("W: {}", width);

    input.iter().enumerate()
        .filter(|&(_, &c)| c == 'X')
        .map(|(index, _)|
            [
                input.get(index - 3..index).is_some_and(|seq| seq.eq(&SAM)),
                input.get(index + 1..index + 4).is_some_and(|seq| seq.eq(&MAS)),
                input.get(index - 3 * width..index).is_some_and(|seq| seq.iter().step_by(width).eq(&SAM)),
                input.get(index + width..index + 3 * width + 1).is_some_and(|seq| seq.iter().step_by(width).eq(&MAS)),
                input.get(index - 3 * width - 3..index).is_some_and(|seq| seq.iter().step_by(width + 1).eq(&SAM)),
                input.get(index - 3 * width + 3..index).is_some_and(|seq| seq.iter().step_by(width - 1).eq(&SAM)),
                input.get(index + width + 1..index + 3 * width + 4).is_some_and(|seq| seq.iter().step_by(width + 1).eq(&MAS)),
                input.get(index + width - 1..index + 3 * width - 2).is_some_and(|seq| seq.iter().step_by(width - 1).eq(&MAS)),
            ]
                .iter()
                .filter(|a| **a)
                .count()
        )
        .sum()
}

/**
 *  Searches for following "XMAS" patterns directly in the input:
 *
 *      M.S   S.M   S.S   M.M
 *      .A.   .A.   .A.   .A.
 *      M.S   S.M   M.M   S.S
 *
 *  takes 71 μs
 */
#[aoc_macro::bench()]
fn part2() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/4/input").expect("Cannot read input");
    let input: Vec<char> = input.chars().collect();
    let width = input.iter().position(|&c| c == '\n').unwrap() + 1;

    input.iter().enumerate()
        .take(input.len() - width - 2) // cut off end   (optimization)
        .skip(width + 1)               // cut off start (optimization)
        .filter(|&(index, &c)|
            c == 'A'
            && [['M', 'S'], ['S', 'M']].contains(&[input[index - width - 1], input[index + width + 1]])
            && [['M', 'S'], ['S', 'M']].contains(&[input[index - width + 1], input[index + width - 1]])
        )
        .count()
}

