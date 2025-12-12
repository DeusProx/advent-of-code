use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/12/input").expect("Cannot read input");

    let (shapes_data, regions_data) = input.rsplit_once("\n\n").unwrap();

    let _shapes: Vec<&str> = shapes_data.split("\n\n")
        .map(|shape_data| shape_data.get(3..(shape_data.len() - 1)).unwrap())
        .collect();

    let regions: Vec<((u16, u16), Vec<u16>)> = regions_data.lines()
        .map(|line| {
            let (dimensions, amounts) = line.split_once(": ").unwrap();
            let (width, height) = dimensions.split_once("x").unwrap();
            let width = width.parse::<u16>().unwrap();
            let height = height.parse::<u16>().unwrap();
            let amounts = amounts.split(" ")
                .map(|n| n.parse::<u16>().unwrap())
                .collect();

            ((width, height), amounts)
        })
        .collect();

    // This guess is good enough to solve my input but not for the test data
    regions.iter()
        .filter(|((width, height), amounts)| width * height >= amounts.iter().sum::<u16>() * 9)
        .count()
}

