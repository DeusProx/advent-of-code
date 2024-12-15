use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/3/input").expect("Cannot read input");

    let output = input.match_indices("mul(")
        .filter_map(|(index, _)| Mul::parse(&input, index))
        .map(|mul| mul.execute())
        .sum();

    output
}

#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/3/input").expect("Cannot read input");

    let muls = input.match_indices("mul(")
        .filter_map(|(index, _)| Mul::parse(&input, index))
        .map(|mul| Instruction::Mul(mul));
    let dos =  input.match_indices("do()")
        .map(|(index, _)| Instruction::Do(Do { start_index: index}));
    let donts = input.match_indices("don't()")
        .map(|(index, _)| Instruction::Dont(Dont { start_index: index}));

    let mut instructions: Vec<Instruction> = muls
        .chain(dos)
        .chain(donts)
        .collect::<Vec<Instruction>>();

    instructions.sort_by_key(|instruction| match instruction {
        Instruction::Do(instruction) => instruction.start_index,
        Instruction::Dont(instruction) => instruction.start_index,
        Instruction::Mul(instruction) => instruction.start_index,
    });

    let output = instructions.iter()
        .fold((true, 0), |(mut enabled, mut acc), item| {
            match item {
                Instruction::Do(_) => enabled = true,
                Instruction::Dont(_) => enabled = false,
                Instruction::Mul(mul) => if enabled {
                    acc += mul.execute();
                }
            };
            (enabled, acc)
        });

    output.1
}

#[derive(Debug)]
pub enum Instruction<'a> {
    Do(Do),
    Dont(Dont),
    Mul(Mul<'a>)
}

#[derive(Debug)]
pub struct Do { start_index: usize }
#[derive(Debug)]
pub struct Dont { start_index: usize }

#[derive(Debug)]
pub struct Mul<'a> {
    start_index: usize,
    _last_index: usize,
    _text: &'a str,
    numbers: (u32, u32),
}

impl<'a> Mul<'a> {
    pub fn parse(input: &'a str, start_index: usize) -> Option<Self>{
        let sub_input = &input[start_index..];
        let closing_index = sub_input.find(")").unwrap();
        let text = &sub_input[..=closing_index];
        let (n1, n2): (&str, &str) = text[4..closing_index].split_once(',')?;
        let a = n1.parse::<u32>().ok()?;
        let b = n2.parse::<u32>().ok()?;

        Some(Self {
            start_index,
            _last_index: start_index + closing_index,
            _text: text,
            numbers: (a,b),
        })
    }
    pub fn execute(&self) -> u32 {
        self.numbers.0 * self.numbers.1
    }
}

