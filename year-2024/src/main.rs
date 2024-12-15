use aoc_days::{
    day01::{day1_part1, day1_part2},
    day02::{day2_part1, day2_part2},
    day03::{day3_part1, day3_part2},
    day04::{day4_part1, day4_part2},
    day05::{day5_part1, day5_part2},
    day06::{day6_part1, day6_part2},
    day07::{day7_part1, day7_part2, day7_part2_rayon},
    day08::{day8_part1, day8_part2},
    day09::{day9_part1, day9_part2},
};

fn main() {
    println!("Day 1");
    println!("  Part 1");
    day1_part1();
    println!("  Part 2");
    day1_part2();
    println!();

    println!("Day 2");
    println!("  Part 1");
    day2_part1();
    println!("  Part 2");
    day2_part2();
    println!();

    println!("Day 3");
    println!("  Part 1");
    day3_part1();
    println!("  Part 2");
    day3_part2();
    println!();

    println!("Day 4");
    println!("  Part 1");
    day4_part1();
    println!("  Part 2");
    day4_part2();
    println!();

    println!("Day 5");
    println!("  Part 1");
    day5_part1();
    println!("  Part 2");
    day5_part2();
    println!();

    println!("Day 6");
    println!("  Part 1");
    day6_part1();
    println!("  Part 2");
    day6_part2();
    println!();

    println!("Day 7");
    println!("  Part 1");
    day7_part1();
    println!("  Part 2");
    day7_part2();
    println!("  Part 2 (Rayon)");
    day7_part2_rayon();
    println!();

    println!("Day 8");
    println!("  Part 1");
    day8_part1();
    println!("  Part 2");
    day8_part2();
    println!();

    println!("Day 9");
    println!("  Part 1");
    day9_part1();
    println!("  Part 2");
    day9_part2();
    println!();

    println!("Day 10");
    println!("  Part 1");
    aoc_days::day10::part1();
    println!("  Part 2");
    aoc_days::day10::part2();
    println!();

    println!("Day 11");
    println!("  Part 1");
    aoc_days::day11::part1();
    println!("  Part 2");
    aoc_days::day11::part2();
    println!();

    println!("Day 13");
    println!("  Part 1");
    aoc_days::day13::part1();
    println!("  Part 2");
    aoc_days::day13::part2();
    println!();

    println!("Day 14");
    println!("  Part 1");
    aoc_days::day14::part1();
    println!("  Part 2");
    println!("    Done graphically");
    println!("    See day14_runner binary");
    println!("    Solution: 6446 (for my input)");
    println!("    Time: A good one");
    println!();
}

