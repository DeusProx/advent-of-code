use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/3/input").expect("Cannot read input");

    input.lines()
        .map(|bank| {
            let mut iter = bank.char_indices().rev();
            _ = iter.advance_by(1);
            let (max_index, max) = iter
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();

            let mut iter = bank.chars();
            _ = iter.advance_by(max_index + 1);
            let max_2 = iter.max_by(|a, b| a.cmp(b)).unwrap();

            ((max as u8 - b'0') * 10 + (max_2 as u8 - b'0')) as u32
        })
        // .inspect(|n| println!("joltage: {n}"))
        .sum()
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/3/input").expect("Cannot read input");

    input.lines()
        .map(|bank| {
            let mut n = 0;
            let mut last_index = 0;
            let mut acc = 0;
            while n < 12 {
                let mut iter = bank.char_indices();
                _ = iter.advance_by(last_index);
                let mut iter = iter.rev();
                _ = iter.advance_by(11 - n);
                let (max_index, max) = iter
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap();

                acc = 10 * acc + (max as u8 - b'0') as u64;
                n += 1;
                last_index = max_index + 1;
            }
            acc

        })
        // .inspect(|n| println!("joltage: {n}"))
        .sum()
}
