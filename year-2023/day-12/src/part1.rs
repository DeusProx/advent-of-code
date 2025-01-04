use std::{fs, time::Instant};

// Brute force algorithm which recursively replaces unknown elements and
// checks if the combinations are valid

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/12/input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
    assert_eq!(result, 6_949);
}

fn valid(springs: &str, groups: &Vec<i32>) -> bool {
    let mut count = 0;
    let mut counted_groups = Vec::new();

    for spring in springs.chars() {
        match spring {
            '.' => {
                if count > 0 {
                    counted_groups.push(count);
                    count = 0;
                }
            },
            '#' => count += 1,
            _ => panic!("No"),
        }
    }

    if count > 0 {
        counted_groups.push(count);
    }

    let is_valid =
        counted_groups.len() == groups.len() &&
        counted_groups.iter()
            .zip(groups.iter())
            .all(|(g1, g2)| g1 == g2);

    is_valid
}

fn solve(springs: &str, groups: &Vec<i32>, index: usize) -> usize {
    if index == springs.len() {
        return match valid(springs, &groups) {
            true => 1,
            false => 0
        };
    }
    let parts = [
        &springs[..index],
        &springs[(index + 1)..],
    ];
    match springs.chars().nth(index).unwrap() == '?' {
        true => {
            solve(&parts.join("#"), groups, index + 1) +
            solve(&parts.join("."), groups, index + 1)
        },
        false => solve(springs, groups, index + 1)
    }
}

fn calc(input: &str) -> usize {
    let now = Instant::now();

    let sum = input.lines()
        .map(|line| {
            let (springs_data, groups_data) = line.split_once(" ").unwrap();
            let groups: Vec<i32> = groups_data.split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            solve(springs_data, &groups, 0)
        })
        .sum();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    sum
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/12/testinput").expect("Cannot read input file");
        let result = calc(&input);
        assert_eq!(result, 21);
    }
}

