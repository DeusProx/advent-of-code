use std::{collections::HashMap, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/11/input").expect("Cannot read input");

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (id, targets) = line.split_once(": ").unwrap();
        let targets: Vec<&str> = targets.split(" ").collect();
        map.insert(id, targets);
    }

    let mut counter = 0;
    let mut nodes = vec!["you"];

    while let Some(node) = nodes.pop() {
        if node == "out" {
            counter += 1;
            continue;
        }
        if let Some(targets) = map.get_mut(node) {
            nodes.extend(targets.iter());
        }
    }

    counter
}


#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/11/input").expect("Cannot read input");

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (id, targets) = line.split_once(": ").unwrap();
        let targets: Vec<&str> = targets.split(" ").collect();
        map.insert(id, targets);
    }

    find_all_paths(("svr", false, false), &map, &mut HashMap::new())
}

type NodeData<'a> = (&'a str, bool, bool);
fn find_all_paths<'a>((node, visited_dac, visited_fft): NodeData<'a>, map: &HashMap<&'a str, Vec<&'a str>>, memoizer: &mut HashMap<NodeData<'a>, u64>) -> u64 {
        if node == "out" {
            match visited_dac && visited_fft {
                true => return 1,
                false => return 0,
            }
        }

        let visited_dac = visited_dac || node == "dac";
        let visited_fft = visited_fft || node == "fft";

        let node_data = (node, visited_dac, visited_fft);

        if let Some(&memoized_value) = memoizer.get(&node_data) {
            return memoized_value;
        }

        if let Some(targets) = map.get(node) {
            let mut counter = 0;
            for &target in targets.iter() {
                counter += find_all_paths((target, visited_dac, visited_fft), map, memoizer)
            }

            memoizer.insert(node_data, counter);
            return counter;
        }

        panic!("in the disco");
}

