/**
* Notes:
*
* The puzzle describes a Ripple-Carry-Adder, where x and y registers are the bits of the input of
* the adder and the z registers are its output.
*/
use std::{collections::{HashMap, VecDeque}, time::Instant};

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    let input = std::fs::read_to_string("../data/2024/day/24/input").expect("Cannot read input");

    let (values, gates) = input.split_once("\n\n").unwrap();
    let mut registers: HashMap<&str, Signal> = values.lines()
        .map(|line| Signal::parse(line))
        .collect();
    let mut rules: VecDeque<Rule> = gates.lines()
        .map(|line| Rule::parse(line))
        .collect();

    'solve: loop {
        if rules.is_empty() {
            break 'solve;
        }
        let rule = rules.pop_front().unwrap();

        match rule.solve(&registers) {
            Some(value) => registers.insert(rule.t, value),
            None => {
                rules.push_back(rule);
                continue;
            },
        };
    }

    get_register(&registers, Register::Z)
}

#[derive(Debug, PartialEq)]
enum Signal { ON, OFF }

impl Signal {
    fn parse(input: &str) -> (&str, Self) {
        let (name, value) = input.split_once(": ").unwrap();
        let value = match value {
            "1" => Signal::ON,
            "0" => Signal::OFF,
            _ => panic!(""),
        };

        (name, value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate { AND, OR, XOR }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule<'a> {
    pub gate: Gate,
    pub l: &'a str,
    pub r: &'a str,
    pub t: &'a str,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        let (operation, t) = input.split_once(" -> ").unwrap();
        let mut iter = operation.splitn(3, " ");
        let l = iter.next().unwrap();
        let gate =  match iter.next().unwrap() {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => panic!(""),
        };
        let r = iter.next().unwrap();

        Self { gate, l, r, t }
    }
    fn solve(self, values: &HashMap<&str, Signal>) -> Option<Signal> {
        let l = values.get(self.l);
        let r = values.get(self.r);

        if l.is_none() || r.is_none() {
            return None;
        }

        let value = match (self.gate, l.unwrap(), r.unwrap()) {
            (Gate::AND, Signal::ON,  Signal::ON) => Signal::ON,
            (Gate::AND,          _,           _) => Signal::OFF,
            (Gate::OR, Signal::OFF, Signal::OFF) => Signal::OFF,
            (Gate::OR,           _,           _) => Signal::ON,
            (Gate::XOR, l, r) => match l != r {
                true => Signal::ON,
                false => Signal::OFF,
            },
        };

        Some(value)
    }
}

pub enum Register { X, Y, Z }
impl Register {
    pub fn as_str(&self) -> &'static str {
        match self {
            Register::X => "x",
            Register::Y => "y",
            Register::Z => "z",
        }
    }
}

fn get_register(registers: &HashMap<&str, Signal>, register: Register) -> u64 {
    let mut outputs: Vec<(&&str, &Signal)> = registers.iter()
        .filter(|(name, _value)| name.starts_with(register.as_str()))
        .collect();
    outputs.sort_by(|a, b| b.0.cmp(a.0));

    outputs.iter().fold(0, |acc, signal| {
        let value = match signal.1 {
            Signal::ON => 1,
            Signal::OFF => 0,
        };

        acc << 1 ^ value
    })
}

#[aoc_macro::bench()]
pub fn part2() -> String {
    let input = std::fs::read_to_string("../data/2024/day/24/input").expect("Cannot read input");

    let (_, gates) = input.split_once("\n\n").unwrap();
    let rules: VecDeque<Rule> = gates.lines()
        .map(|line| Rule::parse(line))
        .collect();

    let mut marked: Vec<&str> = vec![];

    // find the invalid rules for a Ripple-Carry-Adder by filtering after following properties
    for rule in &rules {
        // all rules targeting "z" registers have XOR gates except for last register "z45"
        if rule.t.starts_with("z")
            && rule.gate != Gate::XOR
            && rule.t != "z45"
        {
            marked.push(rule.t);
            continue;
        }

        // all rules with XOR gates that do not target "z" registers cannot source "x" and "y" registers
        if rule.gate == Gate::XOR
           && !rule.t.starts_with("z")
           && !rule.l.starts_with("x")
           && !rule.l.starts_with("y")
           && !rule.r.starts_with("x")
           && !rule.r.starts_with("y")
        {
            marked.push(rule.t);
            continue;
        }

        // for each valid rule with a XOR gate
        // there has to be another rule with XOR gate that sources the target of the first rule
        if rule.gate == Gate::XOR
            && !rule.t.starts_with("z")
            && (rule.l.starts_with("x") || rule.l.starts_with("y"))
            && (rule.r.starts_with("x") || rule.r.starts_with("y"))
        {
            let mut found = false;
            for rule2 in &rules {
                if rule != rule2
                    && rule2.gate == Gate::XOR
                    && (rule.t == rule2.l || rule.t == rule2.r)
                {
                    found = true;
                    break;
                }

            }
            if !found {
                marked.push(rule.t);
                continue;
            }
        }

        // for each valid rule with AND gate which sources "x" and "y" registers expect the start registers
        // there has to be another rule with OR gate that sources the target of the first rule
        if rule.gate == Gate::AND
            && (rule.l.starts_with("x") || rule.l.starts_with("y"))
            && (rule.r.starts_with("x") || rule.r.starts_with("y"))
            && rule.l != "x00"
            && rule.l != "y00"
            && rule.r != "x00"
            && rule.r != "y00"
        {
            let mut found = false;
            for rule2 in &rules {
                if rule != rule2
                    && rule2.gate == Gate::OR
                    && (rule.t == rule2.l || rule.t == rule2.r)
                {
                    found = true;
                    break;
                }

            }
            if !found {
                marked.push(rule.t);
                continue;
            }
        }

    }
    marked.sort();

    marked.join(",")
}

