use std::time::Instant;

fn main() {
    let now = Instant::now();

    println!("Day 1");
    println!("  Part 1");
    days::day01::part1();
    println!("  Part 2");
    days::day01::part2();
    println!();

    println!("Day 2");
    println!("  Part 1");
    days::day02::part1();
    println!("  Part 2");
    days::day02::part2();
    println!();

    println!("Day 3");
    println!("  Part 1");
    days::day03::part1();
    println!("  Part 2");
    days::day03::part2();
    println!();

    println!("Day 4");
    println!("  Part 1");
    days::day04::part1();
    println!("  Part 2");
    days::day04::part2();
    println!();

    println!("Day 5");
    println!("  Part 1");
    days::day05::part1();
    println!("  Part 2");
    days::day05::part2();
    println!();

    println!("Day 6");
    println!("  Part 1");
    days::day06::part1();
    println!("  Part 2");
    days::day06::part2();
    println!();

    println!("Day 7");
    println!("  Part 1");
    days::day07::part1();
    println!("  Part 2");
    days::day07::part2();
    println!("  Part 2 (Rayon)");
    days::day07::part2_rayon();
    println!();

    println!("Day 8");
    println!("  Part 1");
    days::day08::part1();
    println!("  Part 2");
    days::day08::part2();
    println!();

    println!("Day 9");
    println!("  Part 1");
    days::day09::part1();
    println!("  Part 2");
    days::day09::part2();
    println!();

    println!("Day 10");
    println!("  Part 1");
    days::day10::part1();
    println!("  Part 2");
    days::day10::part2();
    println!();

    println!("Day 11");
    println!("  Part 1");
    days::day11::part1();
    println!("  Part 2");
    days::day11::part2();
    println!();

    println!("Day 13");
    println!("  Part 1");
    days::day13::part1();
    println!("  Part 2");
    days::day13::part2();
    println!();

    println!("Day 14");
    println!("  Part 1");
    days::day14::part1();
    println!("  Part 2");
    println!("    Done graphically");
    println!("    See day14_runner binary");
    println!("    Solution: 6446 (for my input)");
    println!("    Time: a good one");
    println!();

    println!("Day 15");
    println!("  Part 1");
    days::day15::part1();
    println!("  Part 2");
    days::day15::part2();
    println!();

    println!("Day 17");
    println!("  Part 1");
    days::day17::part1();
    println!("  Part 2");
    days::day17::part2();
    println!();

    println!("Day 18");
    println!("  Part 1");
    days::day18::part1();
    println!("  Part 2");
    days::day18::part2();
    println!();

    println!("Day 19");
    println!("  Part 1");
    days::day19::part1();
    println!("  Part 2");
    days::day19::part2();
    println!();

    println!("Day 20");
    println!("  Part 1");
    days::day20::part1();
    println!("  Part 2");
    days::day20::part2();
    println!();

    println!("Day 21");
    println!("  Part 1");
    days::day21::part1();
    println!("  Part 2");
    days::day21::part2();
    println!();

    println!("Day 22");
    println!("  Part 1");
    days::day22::part1();
    println!("  Part 2");
    days::day22::part2();
    println!();

    println!("Day 25");
    println!("  Part 1");
    days::day25::part1();
    println!("  Part 2 - Just had to get all other 49 stars");
    println!();

    let elapsed = now.elapsed();
    println!("Executing all puzzles took: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());
}

