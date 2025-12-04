use std::{time::Instant, usize};

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/4/input").expect("Cannot read input");

    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let input: Vec<char> = input.chars().collect();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .filter(|&coords| adjacent_rolls(&input, width, height, coords) < 4 )
        // .inspect(|(x, y)| println!("lifted: ({x}, {y})"))
        .count()
}

#[aoc_macro::bench()]
pub fn part2() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/4/input").expect("Cannot read input");

    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mut input: Vec<char> = input.chars().collect();

    let iter = (0..height).flat_map(|y| (0..width).map(move |x| (y, x)));
    let mut last_count = usize::MAX;
    let mut counter = 0;

    while last_count != 0 {
        let coords: Vec<(usize, usize)> = iter.clone()
            .filter(|&coords| adjacent_rolls(&input, width, height, coords) < 4 )
            .collect();
        last_count = coords.len();
        counter += last_count;
        for (y, x) in coords {
            input[(y) * (width + 1) + x] = '#'
        }
    }

    counter
}

fn adjacent_rolls(input: &Vec<char>, width: usize, height: usize, (y, x): (usize, usize)) -> u8 {
    if input[y * (width + 1) + x] != '@' { return u8::MAX }

    let mut counter: u8 = 0;

    if y > 0 {
        if x     > 0     && input[(y - 1) * (width + 1) + x - 1] == '@' { counter +=1 };
        if                  input[(y - 1) * (width + 1) + x + 0] == '@' { counter +=1 };
        if x + 1 < width && input[(y - 1) * (width + 1) + x + 1] == '@' { counter +=1 };
    }

    if     x     > 0     && input[(y + 0) * (width + 1) + x - 1] == '@' { counter +=1 };
    if     x + 1 < width && input[(y + 0) * (width + 1) + x + 1] == '@' { counter +=1 };

    if y + 1 < height {
        if x     > 0     && input[(y + 1) * (width + 1) + x - 1] == '@' { counter +=1 };
        if                  input[(y + 1) * (width + 1) + x + 0] == '@' { counter +=1 };
        if x + 1 < width && input[(y + 1) * (width + 1) + x + 1] == '@' { counter +=1 };
    }

    counter
}

