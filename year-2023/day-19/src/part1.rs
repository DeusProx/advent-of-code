use std::{fs, time::Instant, cmp::Ordering, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/19/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert!(result == 446517);
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Part {
    fn parse(input: &str) -> Self {
        let parsed = &input.get(1..(input.len() - 1)).unwrap()
            .split(",")
            .map(|p| p.split_once("=").unwrap()
            .1.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let &[x, m, a, s] = &parsed[..] else { panic!("Noo")};
        Self { x, m, a, s }
    }
    fn score(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

enum RuleType {
    X, M, A, S
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Result {
    Accept,
    Reject,
    Next(String),
}

struct Rule {
    kind: RuleType,
    ordering: Ordering,
    value: i32,
    result: Result,
}
impl Rule {
    fn parse(input: &str) -> Self {
        let mut iterator = input.chars();
        Self {
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
            result: match iterator.collect::<String>().as_str() {
                "A" => Result::Accept,
                "R" => Result::Reject,
                n => Result::Next(n.to_owned()),
            }
        }
    }
    fn accepts(&self, part: &Part) -> (bool, Result) {
        let part_value = match self.kind {
            RuleType::X => part.x,
            RuleType::M => part.m,
            RuleType::A => part.a,
            RuleType::S => part.s,
        };
        let is_condition_met = part_value.cmp(&self.value) == self.ordering;
        (is_condition_met, self.result.to_owned())
    }
}


struct Workflow {
    name: String,
    rules: Vec<Rule>,
    result: Result,
}
impl Workflow {
    fn parse(input: &str) -> Self {
        let (name, data) = input.strip_suffix("}").unwrap()
            .split_once("{").unwrap();
        let (rules, result) = data.rsplit_once(",").unwrap();
        Self {
            name: name.to_string(),
            rules: rules.split(",")
                .map(|rule_data| Rule::parse(rule_data))
                .collect(),
            result: match result {
                "A" => Result::Accept,
                "R" => Result::Reject,
                name => Result::Next(name.to_owned()),
            }
        }
    }
    fn accepts(&self, part: &Part) -> Result {
        for rule in self.rules.iter() {
            let (is_rule_met, result) = rule.accepts(part);
            if is_rule_met == true {
                return result
            }
        };
        self.result.clone()
    }
}

fn calc(input: String) -> i32 {
    let now = Instant::now();

    let (data_workflow, data_machine_parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = data_workflow.lines()
        .map(|rule_data| {
            let workflow = Workflow::parse(rule_data);
            (workflow.name.clone(), workflow)
        })
        .collect();


    let parts: Vec<Part> = data_machine_parts.lines()
        .map(|part_data| Part::parse(part_data))
        .collect();

    let mut accepted_parts:Vec<Part> = Vec::default();
    for part in parts {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let result = workflow.accepts(&part);
            match result {
                Result::Reject => break,
                Result::Accept => {
                    accepted_parts.push(part);
                    break;
                },
                Result::Next(name) => workflow = workflows.get(&name).unwrap(),
            }
        }
    }

    let rating = accepted_parts.iter()
        .map(|part| part.score())
        .sum();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    rating
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/19/testinput").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 19114);
    }
}

