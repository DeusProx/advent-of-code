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

    let elapsed = now.elapsed();
    println!("Executing all puzzles took: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());
}
