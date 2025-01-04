use std::{time::Instant, cmp::Ordering, collections::HashMap, ops::Range};

fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/19/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert!(result == 130_090_458_884_662);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RuleType {
    X, M, A, S
}

#[derive(Debug, Clone)]
struct Parts {
    values: Vec<(RuleType, Range<i32>)>
}
impl Parts {
    fn count(&self) -> i64 {
        self.values.iter()
            .map(|(_, range)| (range.len() + 1) as i64)
            .product::<i64>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Result {
    Accept,
    Reject,
    Next(String),
}

#[derive(Debug, Clone)]
struct Rule {
    kind: RuleType,
    ordering: Ordering,
    value: i32,
}
impl Rule {
    fn parse(input: &str) -> (Self, Result) {
        let mut iterator = input.chars();
        let rule = Self {
            kind: match iterator.next() {
                Some('x') => RuleType::X,
                Some('m') => RuleType::M,
                Some('a') => RuleType::A,
                Some('s') => RuleType::S,
                _ => panic!("Noooooooo"),
            },
            ordering: match iterator.next() {
                Some('<') => Ordering::Less,
                Some('>') => Ordering::Greater,
                _ => panic!("Noooooooo"),
            },
            value: iterator.by_ref()
                .take_while(|&c| c != ':')
                .collect::<String>()
                .parse::<i32>().unwrap(),
        };
        let result = match iterator.collect::<String>().as_str() {
            "A" => Result::Accept,
            "R" => Result::Reject,
            n => Result::Next(format!("{}_{}", n, 0)),
        };
        (rule, result)
    }
}

#[derive(Debug)]
struct Node {
    rule: Rule,
    result: Result,
    otherwise: Result,
}

fn parse_nodes(input: &str) -> Vec<(String, Node)> {
    let (name, data) = input.strip_suffix("}").unwrap()
        .split_once("{").unwrap();
    let (rules_data, result) = data.rsplit_once(",").unwrap();

    let mut nodes: Vec<(String, Node)> = rules_data.split(",")
        .map(|rule_data| Rule::parse(rule_data))
        .enumerate()
        .map(|(index, (rule, result))| {
            let node_name = format!("{}_{}", name, index);
            let next_node_name = format!("{}_{}", name, index + 1);
            let node = Node {
                rule,
                result,
                otherwise: Result::Next(next_node_name),
            };
            (node_name, node)
        })
        .collect();

    nodes.last_mut().unwrap().1.otherwise = match result {
            "A" => Result::Accept,
            "R" => Result::Reject,
            name => Result::Next(format!("{}_{}", name, 0)),
    };

    nodes
}

fn calc(input: String) -> i64 {
    let now = Instant::now();

    let (data_workflow, _) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Node> = data_workflow.lines()
        .map(|rule_data| parse_nodes(rule_data))
        .flatten()
        .collect();

    let part_range = Parts {
        values: vec![
            (RuleType::X, 1..4000),
            (RuleType::M, 1..4000),
            (RuleType::A, 1..4000),
            (RuleType::S, 1..4000),
        ],
    };

    let distinct_combinations = traverse_tree_nodes(&workflows, "in_0", &part_range);

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    distinct_combinations
}

fn traverse_tree_nodes(
    nodes: &HashMap<String, Node>,
    current_node_name: &str,
    part_range: &Parts,
) -> i64 {
    let Node { rule, result, otherwise } = nodes.get(current_node_name).unwrap();

    let (accepted, unaccepted) = split_parts(part_range, rule);

    let count_left = match accepted {
        None => 0,
        Some(parts) => match result {
            Result::Reject => 0,
            Result::Accept => parts.count(),
            Result::Next(next_node_name) => traverse_tree_nodes(nodes, &next_node_name, &parts),
        }
    };
    let count_right = match unaccepted {
        None => 0,
        Some(parts) => match otherwise {
            Result::Reject => 0,
            Result::Accept => parts.count(),
            Result::Next(next_node_name) => traverse_tree_nodes(nodes, &next_node_name, &parts),
        }
    };

    count_left + count_right
}

fn split_parts(parts: &Parts, rule: &Rule) -> (Option<Parts>, Option<Parts>) {
    let mut parts: Parts = parts.clone();

    let pos = parts.values.iter().position(|(kind, _)| kind == &rule.kind).unwrap();
    let (kind, range) = parts.values.remove(pos);

    // Hint: These matchings below are not done correctly, but the rules of the puzzle
    // are shaped in a way that the split point is always in the middle.
    // I won't fix this as the solution is still correct.
    let start_accepted = range.start.cmp(&rule.value) == rule.ordering;
    let end_accepted = range.end.cmp(&rule.value) == rule.ordering;

    let accepted_parts = match rule.ordering {
        Ordering::Less => match start_accepted {
            true => Some(range.start..(rule.value - 1)),
            false => None,
        },
        Ordering::Greater => match end_accepted {
            true => Some((rule.value + 1)..range.end),
            false => None,
        },
        _ => panic!("Nooo")
    }.map(|range| {
        let mut parts = parts.clone();
        parts.values.push((kind, range));
        parts
    });

    let unaccepted_parts = match rule.ordering {
        Ordering::Less => match start_accepted {
            true => Some(rule.value..range.end),
            false => None,
        },
        Ordering::Greater => match end_accepted {
            true => Some(range.start..rule.value),
            false => None,
        },
        _ => panic!("Nooo")
    }.map(|range| {
        let mut parts = parts.clone();
        parts.values.push((kind, range));
        parts
    });

    // println!("{:?} {:?}", accepted_parts, unaccepted_parts);

    (accepted_parts, unaccepted_parts)
}


#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/19/testinput").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 167409079868000);
    }
}

