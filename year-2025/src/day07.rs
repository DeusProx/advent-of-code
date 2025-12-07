use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/7/input").expect("Cannot read input");

    let width = input.find("\n").unwrap();
    let height = (input.len() + 1) / (width + 1);

    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;
    for y in 0..(height - 1) {
        for x in 0..width {
            match map[y][x] {
                'S' | '|' => {
                    match map[y+1][x] {
                        '.' => map[y+1][x] = '|',
                        '^' => {
                            counter += 1;
                            if x > 0       { map[y+1][x-1] = '|' }
                            if x < width-1 { map[y+1][x+1] = '|' }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    counter
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/7/input").expect("Cannot read input");

    let width = input.find("\n").unwrap();
    let height = (input.len() + 1) / (width + 1);

    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut beams = vec![0 as u64; width];
    beams[input.find("S").unwrap()] = 1;

    for y in 0..(height - 1) {
        let mut new_beams = vec![0 as u64; width];
        for x in 0..width {
            match map[y][x] {
                'S' | '|' => {
                    match map[y+1][x] {
                        '.' | '|' => {
                            map[y+1][x] = '|';
                            new_beams[x] += beams[x];
                        },
                        '^' => {
                            if x > 0 {
                                map[y+1][x-1] = '|';
                                new_beams[x-1] += beams[x];
                            }
                            if x < width-1 {
                                map[y+1][x+1] = '|';
                                new_beams[x+1] += beams[x];
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }
        beams = new_beams;
    }

    beams.iter().sum()
}

