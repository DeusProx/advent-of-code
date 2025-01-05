use core::panic;
use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> String {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/17/input").expect("Cannot read input");

    let mut pc = Computer::parse(&input);
    let output = pc.run();

    output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/17/input").expect("Cannot read input");
    let og = Computer::parse(&input);

    fn checker(aa: u64, index: usize, computer: &Computer) -> Option<u64> {
        for a in 0..8 {

            let mut pc = computer.clone();
            let value = (aa << 3) | a;
            pc.registers[0] = value;

            if *pc.run().first().unwrap() == pc.program[index] {
                match index == 0 {
                    true => return Some(value),
                    false => {
                        let result = checker(value, index - 1, &computer);
                        if result.is_some() {
                            return result;
                        }
                    }
                }
            }
        }
        None
    }

    checker(0, og.program.len() - 1, &og).unwrap()
}

#[derive(Clone)]
struct Computer {
    registers: Vec<u64>,
    program: Vec<u64>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();

        let registers: Vec<u64> = registers.lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect();
        let program: Vec<u64> = program.split_once(' ').unwrap().1
            .split(',')
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect();

        Self { registers, program }
    }
    fn combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("in the disco"),
        }
    }
    fn run(&mut self) -> Vec<u64> {
        let mut index: usize = 0;
        let mut output: Vec<u64> = Vec::with_capacity(100);

        loop {
            if index >= self.program.len() - 1 {
                break;
            };

            let opcode = self.program[index];
            let operand = self.program[index + 1];

            match opcode {
                0 => self.registers[0] = self.registers[0] >> self.combo_operand(operand),
                1 => self.registers[1] = self.registers[1] ^ operand,
                2 => self.registers[1] = self.combo_operand(operand) & 7,
                3 => {
                    if self.registers[0] != 0 {
                        index = operand as usize;
                        continue;
                    }
                },
                4 => self.registers[1] = self.registers[1] ^ self.registers[2],
                5 => output.push(self.combo_operand(operand) & 7),
                6 => self.registers[1] = self.registers[0] >> self.combo_operand(operand),
                7 => self.registers[2] = self.registers[0] >> self.combo_operand(operand),
                _ => panic!("in the disco"),
            };

            index += 2;
        }

        output
    }
}

