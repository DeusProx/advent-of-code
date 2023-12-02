use std::process::Command;

fn main() {
    let curl_output = Command::new("curl")
        .arg("-X")
        .arg("GET")
        .arg("-H")
        .arg(format!("Cookie: session={}", std::env::var("AOC_SESSION").expect("AOC_SESSION should be set in env")))
        .arg("https://adventofcode.com/2023/day/1/input")
        .output()
        .expect("Cannot download puzzle input");
    let input = String::from_utf8(curl_output.stdout).expect("Cannot parse puzzle input");
    let output = calculate_calibration(input);
    println!("Number:\n{output}");
}
const NUMBERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
];

fn calculate_calibration(input: String) -> u32 {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut modified_line = String::from(line);
            let mut found_number: (usize, (&str, &str));

            loop {
                found_number = NUMBERS.iter()
                    .map(|number| (modified_line.find(number.0).unwrap_or(usize::MAX), *number))
                    .fold((usize::MAX, (&"", &"")), |current_mininum, new_value| {
                        match new_value.0 < current_mininum.0 {
                            true => new_value,
                            false => current_mininum,
                        }
                    });
                if found_number.1.0.is_empty() {
                    break;
                } else {
                    modified_line.replace_range(found_number.0..found_number.0 +1, found_number.1.1);
                }
            }

            modified_line
        })
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
    const INPUT: &str = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "#;

    #[test]
    fn calculates() {
        let output = calculate_calibration(INPUT.to_string());
        assert!(output == 281);
    }
}

