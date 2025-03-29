use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/25/input").expect("Cannot read input");

    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(Schematic::parse)
        .partition(|schematic| schematic.schematic_type == SchematicType::Lock);

    let pairs: Vec<_> = keys.iter()
        .flat_map(|key| locks.iter()
            .map(move |lock| (key, lock))
        )
        .filter(|(key, lock)| key.overlap(lock))
        .collect();

    pairs.len()
}

#[derive(PartialEq, Eq)]
enum SchematicType { Key, Lock }
struct Schematic<'a> {
    #[allow(dead_code)]
    schematics: &'a str,
    schematic_type: SchematicType,
    heights: [u8; 5],
}
impl<'a> Schematic<'a> {
    pub fn parse(schematics: &'a str) -> Self {
        let is_top_row_filled = schematics
            .lines()
            .next().unwrap()
            .chars()
            .all(|c| c == '#');
        let schematic_type =  is_top_row_filled
            .then_some(SchematicType::Lock)
            .unwrap_or(SchematicType::Key);

        // count all parts
        let mut heights = [0, 0, 0, 0, 0];
        for line in schematics.lines() {
            for (index, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[index] += 1;
                }
            }
        }

        Self { schematics, schematic_type, heights }
    }
    pub fn overlap(&self, other: &Self) -> bool {
        if self.schematic_type == other.schematic_type {
            return false;
        }

        self.heights.iter()
            .zip(other.heights.iter())
            .all(|(h_a, h_b)| h_a + h_b < 8)
    }
}

