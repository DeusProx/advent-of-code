use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let ways_to_beat_record = calc(&input);
    println!("Result: {}", ways_to_beat_record);
}


fn calc(input: &str) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let ways_to_beat_record = calc(&input);
        assert!(ways_to_beat_record == 0);
    }
}

