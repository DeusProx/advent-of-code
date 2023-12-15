use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
}


fn calc(input: String) -> i32 {
    input.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 0);
    }
}

