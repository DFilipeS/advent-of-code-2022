use std::{io::stdin, process::exit};

use advent_of_code::read_input;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn apply_operation(&self, item: u64) -> u64 {
        match &self.operation {
            Operation::Sum(op1, op2) => op1.get_value(item) + op2.get_value(item),
            Operation::Mult(op1, op2) => op1.get_value(item) * op2.get_value(item),
        }
    }

    fn execute_test(&self, value: u64) -> u64 {
        match self.test {
            Test::Divisible(divisor, then_val, else_val) => {
                if value % divisor == 0 {
                    then_val
                } else {
                    else_val
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operand {
    Old,
    Value(u64),
}

impl Operand {
    fn get_value(&self, value: u64) -> u64 {
        match self {
            Operand::Old => value,
            Operand::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Sum(Operand, Operand),
    Mult(Operand, Operand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Test {
    Divisible(u64, u64, u64),
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, monkeys) = parser::parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Coult not parse input: {:?}", err);
        exit(2);
    });

    let mut part_1_monkeys = monkeys.clone();
    println!(
        "Monkey business level with worry level relief of 3: {}",
        get_monkey_business_level(&mut part_1_monkeys, 20, 3)
    );

    let mut part_2_monkeys = monkeys;
    println!(
        "Monkey business level with worry level relief of 1: {}",
        get_monkey_business_level(&mut part_2_monkeys, 10000, 1)
    );
}

fn get_monkey_business_level(
    monkeys: &mut [Monkey],
    total_rounds: usize,
    worry_level_factor: u64,
) -> u64 {
    let mut count = vec![0; monkeys.len()];
    let magic_number: u64 = monkeys
        .iter()
        .map(|m| match m.test {
            Test::Divisible(num, _, _) => num,
        })
        .product();

    for _ in 0..total_rounds {
        process_round(monkeys, &mut count, worry_level_factor, magic_number);
    }

    count.sort();

    count.iter().rev().take(2).cloned().product()
}

fn process_round(
    monkeys: &mut [Monkey],
    count: &mut [u64],
    worry_level_factor: u64,
    magic_number: u64,
) {
    (0..monkeys.len()).for_each(|i| {
        let monkey = monkeys.get(i).cloned().unwrap();

        for _ in &monkey.items {
            count[i] += 1;

            let item = monkeys.get_mut(i).unwrap().items.remove(0);
            let worry_level = (monkey.apply_operation(item) / worry_level_factor) % magic_number;
            let target_monkey_id = monkey.execute_test(worry_level);

            monkeys
                .get_mut(target_monkey_id as usize)
                .unwrap()
                .items
                .push(worry_level);
        }
    });
}

mod parser {
    use crate::{Monkey, Operand, Operation, Test};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::newline,
        multi::{separated_list0, separated_list1},
        sequence::separated_pair,
        IResult,
    };

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
        separated_list1(tag("\n\n"), monkey)(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = header(input)?;
        let (input, items) = starting_items(input)?;
        let (input, operation) = operation(input)?;
        let (input, test) = test(input)?;

        let monkey = Monkey {
            items,
            operation,
            test,
        };

        Ok((input, monkey))
    }

    fn header(input: &str) -> IResult<&str, u64> {
        let (input, _) = tag("Monkey ")(input)?;
        let (input, id) = nom::character::complete::u64(input)?;
        let (input, _) = tag(":\n")(input)?;

        Ok((input, id))
    }

    fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
        let (input, _) = tag("  Starting items: ")(input)?;
        let (input, items) = separated_list0(tag(", "), nom::character::complete::u64)(input)?;
        let (input, _) = newline(input)?;

        Ok((input, items))
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("  Operation: new = ")(input)?;
        let (input, operation) = alt((multiplication, sum))(input)?;
        let (input, _) = newline(input)?;

        Ok((input, operation))
    }

    fn multiplication(input: &str) -> IResult<&str, Operation> {
        let (input, (operand1, operand2)) = separated_pair(operand, tag(" * "), operand)(input)?;
        let operation = Operation::Mult(operand1, operand2);

        Ok((input, operation))
    }

    fn sum(input: &str) -> IResult<&str, Operation> {
        let (input, (operand1, operand2)) = separated_pair(operand, tag(" + "), operand)(input)?;
        let operation = Operation::Sum(operand1, operand2);

        Ok((input, operation))
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        let (input, operand) = alt((operand_old, operand_value))(input)?;

        Ok((input, operand))
    }

    fn operand_old(input: &str) -> IResult<&str, Operand> {
        let (input, _) = tag("old")(input)?;
        let operand = Operand::Old;

        Ok((input, operand))
    }

    fn operand_value(input: &str) -> IResult<&str, Operand> {
        let (input, value) = nom::character::complete::u64(input)?;
        let operand = Operand::Value(value);

        Ok((input, operand))
    }

    fn test(input: &str) -> IResult<&str, Test> {
        let (input, _) = tag("  Test: divisible by ")(input)?;
        let (input, value) = nom::character::complete::u64(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("    If true: throw to monkey ")(input)?;
        let (input, true_value) = nom::character::complete::u64(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("    If false: throw to monkey ")(input)?;
        let (input, false_value) = nom::character::complete::u64(input)?;
        let test = Test::Divisible(value, true_value, false_value);

        Ok((input, test))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_with_example_input() {
        let input = fs::read_to_string("./inputs/day11_example.txt").unwrap();
        let result = parser::parse_input(input.trim()).unwrap();
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                operation: Operation::Mult(Operand::Old, Operand::Value(19)),
                test: Test::Divisible(23, 2, 3),
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation::Sum(Operand::Old, Operand::Value(6)),
                test: Test::Divisible(19, 2, 0),
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Operation::Mult(Operand::Old, Operand::Old),
                test: Test::Divisible(13, 1, 3),
            },
            Monkey {
                items: vec![74],
                operation: Operation::Sum(Operand::Old, Operand::Value(3)),
                test: Test::Divisible(17, 0, 1),
            },
        ];

        assert_eq!(result.1, expected);
        assert!(result.0.is_empty());
    }

    #[test]
    fn get_monkey_business_level_with_example_input() {
        let input = fs::read_to_string("./inputs/day11_example.txt").unwrap();
        let (_, mut monkeys) = parser::parse_input(input.trim()).unwrap();
        let result = get_monkey_business_level(&mut monkeys, 20, 3);

        assert_eq!(result, 10605);
    }

    #[test]
    fn get_monkey_business_level_with_example_input_and_no_worry_level_factor() {
        let input = fs::read_to_string("./inputs/day11_example.txt").unwrap();
        let (_, mut monkeys) = parser::parse_input(input.trim()).unwrap();
        let result = get_monkey_business_level(&mut monkeys, 10000, 1);

        assert_eq!(result, 2713310158);
    }
}
