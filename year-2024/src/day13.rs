use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> i64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/13/input").expect("Cannot read input");

    let clawmachines: Vec<Clawmachine> = input
        .split("\n\n")
        .map(|line| Clawmachine::parse(line))
        .collect();

    clawmachines.iter()
        .filter_map(|clawmachine| clawmachine.solve())
        .map(|(a, b)| 3 * a + b)
        .sum()
}

#[aoc_macro::bench()]
pub fn part2() -> i64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/13/input").expect("Cannot read input");

    let clawmachines: Vec<Clawmachine> = input
        .split("\n\n")
        .map(|line| Clawmachine::parse(line))
        .map(|mut clawmachine| {
            clawmachine.prize.0 += 10_000_000_000_000;
            clawmachine.prize.1 += 10_000_000_000_000;
            clawmachine
        })
        .collect();

    clawmachines.iter()
        .filter_map(|clawmachine| clawmachine.solve())
        .map(|(a, b)| 3 * a + b)
        .sum()
}

struct Clawmachine {
    a: Button,
    b: Button,
    prize: Prize,
}
impl Clawmachine {
    pub fn parse(input: &str) -> Self {
        let mut iter = input.split("\n");
        let a = Button::parse(iter.next().unwrap());
        let b = Button::parse(iter.next().unwrap());
        let prize = Prize::parse(iter.next().unwrap());
        Clawmachine { a, b, prize }
    }
    // - the vectors and positions presented by the buttons and the prize create a linear system we solve
    // - we also assume that a is no multiple of b or in other words the vectors represented by the
    //   buttons are not parallel
    //
    // Given:
    //    a.x * m.a + b.x * m.b = prize.x
    //    a.y * m.a + b.y * m.b = prize.y
    //
    // =>
    //    m.b = (prize.x - a.x * m.a) / b.x
    //    m.b = (prize.y - a.y * m.a) / b.y
    //
    // => (prize.x - a.x * m.a) / b.x = (prize.y - a.y * m.a) / b.y
    // => (prize.x - a.x * m.a) * b.y = (prize.y - a.y * m.a) * b.x
    // => prize.x * b.y - a.x * m.a * b.y = prize.y * b.x - a.y * m.a * b.x
    // => prize.x * b.y - prize.y * b.x = a.x * m.a * b.y - a.y * m.a * b.x
    // => prize.x * b.y - prize.y * b.x = (a.x * b.y - a.y * b.x) * m.a
    //
    // => m.a = prize.x * b.y - prize.y * b.x / (a.x * b.y - a.y * b.x)
    //
    // Calculate m.a and m.b and check calculations
    //
    pub fn solve(&self) -> Option<(i64, i64)> {
        let Clawmachine { a, b, prize: p } = self;

        let mult_a = (p.0 * b.1 - p.1 * b.0) / (a.0 * b.1 - a.1 * b.0);
        let mult_b = (p.0 - mult_a * a.0) / b.0;
        let m = (mult_a, mult_b);

        let tester = (m.0 * a.0 + m.1 * b.0, m.0 * a.1 + m.1 * b.1);
        match p.0 == tester.0 && p.1 == tester.1 {
            true => Some((mult_a, mult_b)),
            false => None,
        }
    }
}

struct Button(i64, i64);
impl Button {
    pub fn parse(input: &str) -> Self {
        let mut numbers = input
            .split_once(": ").unwrap()
            .1
            .split(", ")
            .map(|number| number.split_once("+").unwrap().1.parse::<i64>().unwrap());
        Self (numbers.next().unwrap(), numbers.next().unwrap())
    }
}
struct Prize(i64, i64);
impl Prize {
    pub fn parse(input: &str) -> Self {
        let mut numbers = input
            .split_once(": ").unwrap()
            .1
            .split(", ")
            .map(|number| number.split_once("=").unwrap().1.parse::<i64>().unwrap());
        Self (numbers.next().unwrap(), numbers.next().unwrap())
    }
}
