use std::{fs, time::Instant};
use itertools::Itertools;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/13/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {:?}", result);
    assert!(result == 33183);
}

fn calc(input: String) -> usize {
    let now = Instant::now();

    let sum = input.split("\n\n")
        .map(|lava_field| lava_field.trim().lines().collect::<Vec<&str>>())
        .map(|lava_field| (
            horizontal_mirror_index(lava_field.clone()).unwrap_or(0),
            vertical_mirror_index(lava_field.clone()).unwrap_or(0),
        ))
        .map(|(h, v)| h * 100 + v)
        .sum();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    sum
}

fn vertical_mirror_index(lava_field: Vec<&str>) -> Option<usize> {
    let transposed = transpose(lava_field);
    horizontal_mirror_index(transposed.iter().map(|line| line.as_str()).collect::<Vec<&str>>())
}

fn horizontal_mirror_index(lava_field: Vec<&str>) -> Option<usize> {
    let mut value = None;
    for index in 1..(lava_field.len()) {
        let (start, end) = lava_field.split_at(index);
        let errors = start.iter()
            .rev()
            .join("")
            .chars()
            .zip(end.join("").chars())
            .filter(|(a, b)| a != b)
            .count();
        if errors == 1 {
            value = Some(index);
        }
    }
    value
}

fn transpose(lava_field: Vec<&str>) -> Vec<String> {
    let line_length = lava_field.first().unwrap().len();
    let mut row_iters: Vec<_> = lava_field.iter()
        .map(|line| line.chars())
        .collect();
    let transposed = (0..line_length)
        .map(|_| {
            row_iters.iter_mut()
                .map(|row_iter| row_iter.next().unwrap())
                .collect::<String>()
        })
        .collect();
    transposed
}


#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/13/testinput").expect("Cannot read input file");
        let result = calc(input);
        println!("{:?}", result);
        assert!(result == 400);
    }
}

