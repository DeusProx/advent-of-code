use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
    assert!(result == 1584748274);
}


fn calc(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|history| analyse(history))
        .sum()
}

fn analyse(history: Vec<i32>) -> i32 {
    match history.iter().any(|val| *val != 0) {
        true => history.last().unwrap() +  analyse(
            history.as_slice()
                .windows(2)
                .map(|window| window.last().unwrap() - window.first().unwrap())
                .collect::<Vec<i32>>()
        ),
        false => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(&input);
        assert!(result == 114);
    }
}

