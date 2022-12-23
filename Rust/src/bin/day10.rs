use std::{io::stdin, process::exit};

use advent_of_code::read_input;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug)]
struct Processor<'a> {
    instructions: &'a [Instruction],
    program_counter: usize,
    cycle: i32,
    instruction_cycle: u32,
    register: i32,
}

impl<'a> Processor<'a> {
    pub fn new(instructions: &[Instruction]) -> Processor {
        Processor {
            instructions,
            program_counter: 0,
            cycle: 1,
            instruction_cycle: 0,
            register: 1,
        }
    }

    pub fn next_cycle(&mut self) {
        let current_instruction = &self.instructions.get(self.program_counter).unwrap();

        match current_instruction {
            Instruction::Noop => self.next_instruction(),
            Instruction::Add(value) => {
                if self.instruction_cycle == 1 {
                    self.register += value;
                    self.next_instruction();
                } else {
                    self.instruction_cycle += 1;
                }
            }
        }

        self.cycle += 1;
    }

    fn next_instruction(&mut self) {
        self.program_counter += 1;
        self.instruction_cycle = 0;
    }
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, instructions) = parser::parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Coult not parse input: {:?}", err);
        exit(2);
    });

    println!("Signal strength: {}", signal_strengths(&instructions));
    println!();
    draw_screen(&instructions);
}

fn draw_screen(instructions: &[Instruction]) {
    let mut processor = Processor::new(instructions);

    for _ in 0..6 {
        for column in 0..40 {
            let sprite_location = processor.register - 1..=processor.register + 1;

            if sprite_location.contains(&column) {
                print!("#");
            } else {
                print!(" ");
            }
            processor.next_cycle();
        }
        println!();
    }
}

fn signal_strengths(instructions: &[Instruction]) -> i32 {
    let mut processor = Processor::new(instructions);
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut result = 0;

    while processor.cycle < 240 {
        if interesting_cycles.contains(&processor.cycle) {
            result += processor.register * processor.cycle;
        }
        processor.next_cycle();
    }

    result
}

mod parser {
    //! Parses input from Advent of Code 2022 (Day 10) problem.
    //!
    //! The input is very simple. It is a list of instructions and there are only two possible
    //! instructions:
    //! - `noop`
    //! - `addx V` where `V` is an signed integer

    use super::Instruction;
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::newline, multi::separated_list1,
        IResult,
    };

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
        let (input, instructions) = separated_list1(newline, alt((noop, addx)))(input)?;

        Ok((input, instructions))
    }

    fn noop(input: &str) -> IResult<&str, Instruction> {
        let (input, _) = tag("noop")(input)?;

        Ok((input, Instruction::Noop))
    }

    fn addx(input: &str) -> IResult<&str, Instruction> {
        let (input, _) = tag("addx ")(input)?;
        let (input, value) = nom::character::complete::i32(input)?;
        let instruction = Instruction::Add(value);

        Ok((input, instruction))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_input_with_small_example_input() {
        let input = "noop
addx 3
addx -5";
        let (input, result) = parser::parse_input(input).unwrap();
        let expected = vec![Instruction::Noop, Instruction::Add(3), Instruction::Add(-5)];

        assert_eq!(result, expected);
        assert!(input.is_empty());
    }

    #[test]
    fn signal_strengths_with_large_example_input() {
        let input = fs::read_to_string("inputs/day10_example.txt").unwrap();
        let (_, instructions) = parser::parse_input(input.as_str()).unwrap();
        let result = signal_strengths(&instructions);

        assert_eq!(result, 13140);
    }
}
