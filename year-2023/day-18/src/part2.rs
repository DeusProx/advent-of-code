// Remarks:
//   - Picks theorem (Satz von Pick):
//     - For any Polygon: A = I + R / 2 - 1
//     - I is interior tiles count
//     - R is boundary tile count
//   - Shoe lace formula (Gaußsche Trapezformel):
//     - Calculates interior tiles

use std::{fs, time::Instant};
use std::ops::{Add, Mul};

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/18/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert!(result == 85070763635666);
}

#[derive(Debug, Clone, Copy)]
struct Coords<T>(T, T);


impl<T: std::ops::Add<Output = T>> Add for Coords<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> Mul<T> for Coords<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

fn calc(input: String) -> i64 {
    let now = Instant::now();

    let mut position = Coords(0, 0);
    let mut nodes = vec![position];
    let mut boundary_count = 0;

    for line in input.lines() {
        let data: (&str, &str) = line.split(' ')
            .nth(2).unwrap()
            .strip_prefix("(#").unwrap()
            .strip_suffix(")").unwrap()
            .split_at(5);

        let len = i64::from_str_radix(data.0, 16).unwrap();
        let dir = match data.1 {
            "3" => Coords( 0, -1), // U
            "1" => Coords( 0,  1), // D
            "2" => Coords(-1,  0), // L
            "0" => Coords( 1,  0), // R
            _ => panic!("Nooooo"),
        };

        position = position + dir * len;
        nodes.push(position);
        boundary_count += len;
    };

    // Shoe lace
    let nodes_count = nodes.len();
    let interior = (0..(nodes.len())).into_iter()
        .map(|index| {
            let prev_index = index.checked_sub(1).unwrap_or(nodes_count - 1);
            let next_index = (index + 1) % nodes_count;
            &nodes[index].0 * (nodes[next_index].1 - nodes[prev_index].1)
        })
        .sum::<i64>();
    let interior_count = interior / 2;

    // picks
    let area = interior_count + boundary_count / 2 + 1;

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    area
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/18/testinput").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 952408144115);
    }
}

