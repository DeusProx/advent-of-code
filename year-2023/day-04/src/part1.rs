fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/4/input").expect("Cannot read input file");
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

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/4/testinput").expect("Cannot read input file");
        let points = calc_points(input);
        assert!(points == 13);
    }
}

