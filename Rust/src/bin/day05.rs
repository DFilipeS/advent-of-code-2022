use std::io::stdin;

use advent_of_code::read_input;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Move {
    amount: usize,
    source: usize,
    destination: usize,
}

type Stack = Vec<char>;

#[derive(Debug)]
enum Error {
    InvalidInput,
    MissingStack,
    MissingMoveInformation,
    InvalidDigit,
}

fn main() {
    let input = read_input(&mut stdin()).unwrap();
    let (stacks, instructions) = parse_input(input.as_str()).unwrap();

    let mut stacks_1 = stacks.clone();
    for instruction in &instructions {
        apply_move_with_crate_mover_9000(&mut stacks_1, instruction);
    }

    println!(
        "Top crates after instructions with CrateMover 9000: {}",
        get_top_crates_message(stacks_1)
    );

    let mut stacks_2 = stacks;
    for instruction in &instructions {
        apply_move_with_crate_mover_9001(&mut stacks_2, instruction);
    }

    println!(
        "Top crates after instructions with CrateMover 9001: {}",
        get_top_crates_message(stacks_2)
    );
}

fn get_top_crates_message(stacks: Vec<Stack>) -> String {
    stacks.iter().filter_map(|s| s.last()).collect()
}

fn apply_move_with_crate_mover_9000(stacks: &mut [Stack], instruction: &Move) {
    let source = stacks.get_mut(instruction.source - 1).unwrap();
    let mut crates = vec![];

    for _ in 0..instruction.amount {
        crates.push(source.pop().unwrap());
    }

    stacks
        .get_mut(instruction.destination - 1)
        .unwrap()
        .append(&mut crates);
}
fn apply_move_with_crate_mover_9001(stacks: &mut [Stack], instruction: &Move) {
    let source = stacks.get_mut(instruction.source - 1).unwrap();
    let mut crates = vec![];

    for _ in 0..instruction.amount {
        crates.insert(0, source.pop().unwrap());
    }

    stacks
        .get_mut(instruction.destination - 1)
        .unwrap()
        .append(&mut crates);
}

fn parse_input(input: &str) -> Result<(Vec<Stack>, Vec<Move>), Error> {
    let input_blocks: Vec<&str> = input.split("\n\n").collect();
    let mut lines: Vec<&str> = input_blocks
        .first()
        .ok_or(Error::InvalidInput)?
        .lines()
        .collect();
    let (_, lines) = lines.split_last_mut().ok_or(Error::InvalidInput)?;
    lines.reverse();

    let line_length = lines.first().ok_or(Error::InvalidInput)?.len();
    let total_stacks = line_length / 3;
    let mut stacks = vec![];

    for _ in 0..total_stacks {
        let stack: Vec<char> = vec![];
        stacks.push(stack);
    }

    for line in lines {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks.get_mut(i).ok_or(Error::MissingStack)?.push(c);
            }
        }
    }

    let regex = r"move (?P<amount>\d+) from (?P<source>\d+) to (?P<destination>\d+)";
    let regex = Regex::new(regex).unwrap();
    let moves: Result<Vec<Move>, Error> = regex
        .captures_iter(input_blocks.last().ok_or(Error::InvalidInput)?)
        .map(|c| {
            let amount = c
                .name("amount")
                .ok_or(Error::MissingMoveInformation)?
                .as_str()
                .parse::<usize>()
                .map_err(|_| Error::InvalidDigit)?;
            let source = c.name("source").unwrap().as_str().parse::<usize>().unwrap();
            let destination = c
                .name("destination")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            Ok(Move {
                amount,
                source,
                destination,
            })
        })
        .collect();

    Ok((stacks, moves?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_with_example_input() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (stacks, moves) = parse_input(input).unwrap();
        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected_moves = vec![
            Move {
                amount: 1,
                source: 2,
                destination: 1,
            },
            Move {
                amount: 3,
                source: 1,
                destination: 3,
            },
            Move {
                amount: 2,
                source: 2,
                destination: 1,
            },
            Move {
                amount: 1,
                source: 1,
                destination: 2,
            },
        ];

        assert_eq!(stacks, expected_stacks);
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn apply_move_with_crate_mover_9000_with_first_example() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let move_instruction = Move {
            amount: 1,
            source: 2,
            destination: 1,
        };
        let expected = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];

        apply_move_with_crate_mover_9000(&mut stacks, &move_instruction);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn apply_move_with_crate_mover_9000_with_second_example() {
        let mut stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        let move_instruction = Move {
            amount: 3,
            source: 1,
            destination: 3,
        };
        let expected = vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']];

        apply_move_with_crate_mover_9000(&mut stacks, &move_instruction);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn apply_move_with_crate_mover_9001_with_first_example() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let move_instruction = Move {
            amount: 1,
            source: 2,
            destination: 1,
        };
        let expected = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];

        apply_move_with_crate_mover_9001(&mut stacks, &move_instruction);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn apply_move_with_crate_mover_9001_with_second_example() {
        let mut stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        let move_instruction = Move {
            amount: 3,
            source: 1,
            destination: 3,
        };
        let expected = vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']];

        apply_move_with_crate_mover_9001(&mut stacks, &move_instruction);

        assert_eq!(stacks, expected);
    }

    #[test]
    fn get_top_crates_message_with_example_input() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected = "NDP";
        let result = get_top_crates_message(stacks);

        assert_eq!(result, expected);
    }
}
