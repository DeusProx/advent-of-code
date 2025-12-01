use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> i16 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/1/input").expect("Cannot read input");

    let mut point: i16 = 50;
    let mut counter: i16 = 0;
    for rotation in input.lines() {
        let (direction, degree) = rotation.split_at(1);
        let degree = degree.parse::<i16>().expect("value cannot be parsed");

        match direction {
            "L" => point -= degree,
            "R" => point += degree,
            _ => panic!("at the disco!"),
        }
        point = (point + 100) % 100; // we add 100 or there could be negative values

        if point == 0 {
            counter += 1;
        }

        // println!("- {rotation} -> {point}; counter: {counter}");
    }

    counter
}

#[aoc_macro::bench()]
pub fn part2_old() -> i16 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/1/input").expect("Cannot read input");

    let mut point: i16 = 50;
    let mut counter: i16 = 0;
    for rotation in input.lines() {
        let direction = rotation.chars().nth(0).unwrap();
        let degree = rotation[1..].parse::<i16>().expect("value cannot be parsed");

        let mut new_point = match direction {
            'L' => point - degree,
            'R' => point + degree,
            _ => panic!("at the disco!"),
        };

        let turns = new_point.div_euclid(100).abs();
        new_point = new_point.rem_euclid(100);

        counter += match (point, new_point, direction) {
            (0, 0, 'L') => turns,
            (0, _, 'L') => turns - 1,
            (_, 0, 'L') => turns + 1,
            (_, _, 'L') => turns,
            (_, _, 'R') => turns,
            _ => panic!("in the disco"),
        };

        point = new_point;

        // println!("- {rotation} -> {point}; +{turns} => counter: {counter}");
    }

    counter
}

// without ugly/"slow" matching
#[aoc_macro::bench()]
pub fn part2() -> i16 {
    // TODO: Solve with macro
    // let input = std::fs::read_to_string("../data/2025/day/1/test").expect("Cannot read input"); // 6
    let input = std::fs::read_to_string("../data/2025/day/1/input").expect("Cannot read input"); // 6122

    let mut point: i16 = 50;
    let mut counter: i16 = 0;
    for rotation in input.lines() {
        let direction = rotation.chars().nth(0).unwrap();
        let degree = rotation[1..].parse::<i16>().expect("value cannot be parsed");

        if direction == 'L' && point == 0 {
            counter -= 1;
        }
        point = match direction {
            'L' => point - degree,
            'R' => point + degree,
            _ => panic!("at the disco!"),
        };

        counter += point.div_euclid(100).abs();
        point = point.rem_euclid(100);

        if direction == 'L' && point == 0 {
            counter += 1;
        }

        // println!("- {rotation} -> {point}; counter: {counter}");
    }

    counter
}

