use std::{fs, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}
const START: &'static str = "AAA";
const END: &'static str = "ZZZ";

fn calc(input: &str) -> i32 {
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let map = nodes.lines()
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

    let mut steps: i32 = 0;
    let mut current_name: &str = START;
    let mut dir_iter = directions.chars();
    loop {
        if current_name == END {
            break;
        }

        let node = map.get(current_name).unwrap();
        current_name = match dir_iter.next() {
            None => {
                dir_iter = directions.chars();
                continue;
            },
            Some('L') => &node.left,
            Some('R') => &node.right,
            Some(n) => panic!("Wrong direction: {}", n),
        };
        steps += 1;
    };

    steps
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test1").expect("Cannot read input file");
        let result = calc(&input);
        assert!(result == 2);
    }
}

