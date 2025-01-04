fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/15/input").expect("Cannot read input file");
    let result = hash(input);
    println!("Result: {}", result);
}

fn hash(input: String) -> u32 {
    input.trim().split(",")
        .map(|seq|
            seq.chars()
                .fold(0, |mut acc, c| {
                    acc += c as u32;
                    acc *= 17;
                    acc %= 256;
                    acc
                })
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/15/testinput").expect("Cannot read input file");
        let result = hash(input);
        assert!(result == 1320);
    }
    #[test]
    fn test_hash() {
        let result = hash("HASH".to_string());
        assert!(result == 52);
    }
}

