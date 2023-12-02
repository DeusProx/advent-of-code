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
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#.to_string();
        let output = calculate_calibration(input);
        assert!(output == 142);
    }
}

