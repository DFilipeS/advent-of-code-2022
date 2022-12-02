use std::{cmp::Ordering, io::Read, process::exit};

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn counter(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn advantage(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Shape::Rock, Shape::Scissor) => Some(Ordering::Greater),
            (Shape::Scissor, Shape::Paper) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Rock) => Some(Ordering::Greater),
            (Shape::Rock, Shape::Paper) => Some(Ordering::Less),
            (Shape::Paper, Shape::Scissor) => Some(Ordering::Less),
            (Shape::Scissor, Shape::Rock) => Some(Ordering::Less),
            (_, _) => Some(Ordering::Equal),
        }
    }
}

#[derive(Debug)]
enum Error {
    FailedToReadInput,
    InvalidShape,
}

#[derive(Debug, PartialEq)]
struct Round {
    player: Shape,
    opponent: Shape,
}

impl Round {
    fn points(&self) -> u32 {
        let mut score = 0;

        // Add points based on the round outcome
        match self.player.partial_cmp(&self.opponent) {
            Some(Ordering::Greater) => score += 6,
            Some(Ordering::Equal) => score += 3,
            _ => (),
        }

        // Add points based on the chosen shape
        match self.player {
            Shape::Rock => score += 1,
            Shape::Paper => score += 2,
            Shape::Scissor => score += 3,
        }

        score
    }
}

fn main() {
    let input = match read_input(&mut std::io::stdin()) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Could not read input: {:?}", err);
            exit(1);
        }
    };
    let rounds_with_guess = match parse_input_with_guess(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Could not parse input: {:?}", err);
            exit(2);
        }
    };
    let rounds_with_strategy = match parse_input_with_strategy(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Could not parse input: {:?}", err);
            exit(2);
        }
    };

    println!(
        "Total score with guess on input: {}",
        process_rounds(&rounds_with_guess)
    );
    println!(
        "Total score with correct strategy: {}",
        process_rounds(&rounds_with_strategy)
    );
}

fn process_rounds(rounds: &Vec<Round>) -> u32 {
    let mut score = 0;

    for round in rounds {
        score += round.points();
    }

    score
}

fn read_input(reader: &mut impl Read) -> Result<String, Error> {
    let mut buffer = String::new();

    reader
        .read_to_string(&mut buffer)
        .map_err(|_| Error::FailedToReadInput)?;

    Ok(buffer)
}

fn parse_input_with_guess(input: &str) -> Result<Vec<Round>, Error> {
    input
        .trim()
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let opponent = match tokens.first() {
                Some(&"A") => Shape::Rock,
                Some(&"B") => Shape::Paper,
                Some(&"C") => Shape::Scissor,
                _ => return Err(Error::InvalidShape),
            };
            let player = match tokens.last() {
                Some(&"X") => Shape::Rock,
                Some(&"Y") => Shape::Paper,
                Some(&"Z") => Shape::Scissor,
                _ => return Err(Error::InvalidShape),
            };
            Ok(Round { opponent, player })
        })
        .collect()
}

fn parse_input_with_strategy(input: &str) -> Result<Vec<Round>, Error> {
    input
        .trim()
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let opponent = match tokens.first() {
                Some(&"A") => Shape::Rock,
                Some(&"B") => Shape::Paper,
                Some(&"C") => Shape::Scissor,
                _ => return Err(Error::InvalidShape),
            };
            let player = match (&opponent, tokens.last()) {
                (shape, Some(&"X")) => shape.advantage(),
                (shape, Some(&"Y")) => shape.clone(),
                (shape, Some(&"Z")) => shape.counter(),
                (_, _) => return Err(Error::InvalidShape),
            };
            Ok(Round { opponent, player })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_parse_input_with_example_input_with_guess() {
        let result = example_rounds_with_guess();
        let expected = vec![
            Round {
                player: Shape::Paper,
                opponent: Shape::Rock,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Paper,
            },
            Round {
                player: Shape::Scissor,
                opponent: Shape::Scissor,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn read_and_parse_input_with_example_input_with_strategy() {
        let result = example_rounds_with_strategy();
        let expected = vec![
            Round {
                player: Shape::Rock,
                opponent: Shape::Rock,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Paper,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Scissor,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn read_and_parse_input_with_empty_input() {
        let input = "\n\n\n";
        let values = read_input(&mut input.as_bytes()).unwrap();
        let result = parse_input_with_guess(&values).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn process_rounds_with_example_input_with_guess() {
        let rounds = example_rounds_with_guess();
        let value = process_rounds(&rounds);

        assert_eq!(value, 15);
    }

    #[test]
    fn process_rounds_with_example_input_with_strategy() {
        let rounds = example_rounds_with_strategy();
        let value = process_rounds(&rounds);

        assert_eq!(value, 12);
    }

    fn example_rounds_with_guess() -> Vec<Round> {
        let input = "A Y\nB X\nC Z";
        let values = read_input(&mut input.as_bytes()).unwrap();

        parse_input_with_guess(&values).unwrap()
    }

    fn example_rounds_with_strategy() -> Vec<Round> {
        let input = "A Y\nB X\nC Z";
        let values = read_input(&mut input.as_bytes()).unwrap();

        parse_input_with_strategy(&values).unwrap()
    }
}
