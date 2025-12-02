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

    let elapsed = now.elapsed();
    println!("Executing all puzzles took: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());
}
