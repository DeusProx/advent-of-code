use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/9/input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {:?}", result);
    assert!(result == (1026, 1584748274));
}

fn calc(input: &str) -> (i32, i32) {
    input.lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|history| analyse(history))
        .fold((0,0),|acc, (before, after)| (acc.0 + before, acc.1 + after))
}

fn analyse(history: Vec<i32>) -> (i32, i32) {
    match history.iter().any(|val| *val != 0) {
        true => {
            let extrapolation = history.as_slice()
                .windows(2)
                .map(|window| window.last().unwrap() - window.first().unwrap())
                .collect::<Vec<i32>>();
            let (before, after) = analyse(extrapolation.clone());
            (
                -before +  history.first().unwrap(),
                after + history.last().unwrap(),
            )
        },
        false => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/9/testinput").expect("Cannot read input file");
        let result = calc(&input);
        assert!(result == (2, 114));
    }
}

