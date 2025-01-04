use std::{fs, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/8/input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn calc(input: &str) -> i64 {
    let (directions_data, node_data) = input.split_once("\n\n").unwrap();
    let nodes: HashMap<String, Node> = node_data.lines()
        .fold(HashMap::new(), |mut map, line| {
            let (name, paths) = line.split_once(" = ").unwrap();
            let (left, right) = paths.strip_prefix("(").unwrap()
                .strip_suffix(")").unwrap()
                .split_once(", ").unwrap();
            let node = Node {
                name: name.to_owned(),
                left: left.to_owned(),
                right: right.to_owned(),
            };
            map.insert(node.name.to_owned(), node);
            map
        });

    let mut current_names: Vec<String> = nodes.keys()
        .filter_map(|key| match key.ends_with('A') {
            true => Some(key.to_owned()),
            false => None,
        })
        .collect();

    let path_lengths: Vec<i64> = current_names.iter_mut()
        .map(|start_name| {
            let mut current_name: &str = &start_name;
            let mut steps: i64 = 0;
            let mut dir_iter = directions_data.chars();

            loop {
                if current_name.ends_with('Z') {
                    break;
                }

                let node = nodes.get(current_name).unwrap();
                current_name = match dir_iter.next() {
                    None => {
                        dir_iter = directions_data.chars();
                        continue;
                    },
                    Some('L') => &node.left,
                    Some('R') => &node.right,
                    Some(n) => panic!("Wrong direction: {}", n),
                };

                steps += 1;
            };

            steps
        })
        .collect();

    lcm(path_lengths)
}

// least common multiple
fn lcm(numbers: Vec<i64>) -> i64 {
    numbers.iter()
        .fold(1 as i64, |acc, &val| acc * val / gcd(acc, val))
}

// greatest common denominator
fn gcd(a: i64, b: i64) -> i64 {
    match b == 0 {
        true => a,
        false => gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/8/testinput2").expect("Cannot read input file");
        let result = calc(&input);
        println!("# {}", result);
        assert!(result == 6);
    }
}

