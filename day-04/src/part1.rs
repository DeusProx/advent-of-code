use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Cannot read input file")
        .parse().expect("Cannot parse input file");
    let output = calc_points(input);
    println!("calc_points: {}", output);
}

fn calc_points(input: String) -> i32 {
    input.trim()
        .lines()
        .map(|line| line.trim())
        .filter_map(|line| {
            let (_, data) = line.split_once(": ").unwrap();
            let (winning_numbers_data, picked_numbers_data) = data.split_once(" | ").unwrap();

            let winning_numbers: Vec<i32> = winning_numbers_data.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            let picked_numbers: Vec<i32> = picked_numbers_data.split_whitespace()
                .map(|n| n.parse().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .collect();

            match picked_numbers.len() > 0 {
                true => Some(2_i32.pow(picked_numbers.len() as u32 - 1)),
                false => None,
            }
        }).sum()
}


#[cfg(test)]
mod tests {
    use crate::calc_points;

    const INPUT: &str = r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test() {
        let points = calc_points(INPUT.to_string());
        assert!(points == 13);
    }
}

