use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/1/input").expect("Cannot read input file");
    let output = calculate_calibration(input);
    println!("Number:\n{output}");
}

fn calculate_calibration(input: String) -> u32 {
    input.lines()
        .map(|line| line.chars()
            .filter_map(|char| char.to_digit(10))
            .collect::<Vec<u32>>()
        )
        .map(|numbers| numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0))
        .fold(0, |acc, e| acc + e)
}

#[cfg(test)]
mod tests {
    use super::calculate_calibration;

    #[test]
    fn calculates() {
        let input: String = std::fs::read_to_string("../../data/2023/day/1/testinput").expect("Cannot read input file");
        let output = calculate_calibration(input);
        assert!(output == 142);
    }
}

