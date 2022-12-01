use std::{io::Read, num::ParseIntError, process::exit};

#[derive(Debug, PartialEq)]
struct Elf {
    inventory: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.inventory.iter().sum()
    }
}

fn main() {
    let elves = match read_input(&mut std::io::stdin()) {
        Ok(value) => value,
        Err(_err) => {
            eprintln!("Error parsing input, not a valid integer found");
            exit(1);
        }
    };
    let most_calories_elf = match find_elf_with_most_calories(&elves) {
        Some(elf) => elf,
        None => {
            eprintln!("Could not find any Elves");
            exit(2);
        }
    };
    let top_3_most_calories_elves = get_top_3_elves_with_most_calories(&elves);

    println!(
        "Max calories in a single Elf: {}",
        most_calories_elf.total_calories()
    );
    println!(
        "Sum of top 3 Elves with most calories: {}",
        top_3_most_calories_elves
            .iter()
            .map(|elf| elf.total_calories())
            .sum::<u32>()
    )
}

fn find_elf_with_most_calories(elves: &Vec<Elf>) -> Option<&Elf> {
    if elves.is_empty() {
        return None;
    }

    let mut max_calories_elf = elves.first().unwrap();

    for elf in elves {
        if elf.total_calories() > max_calories_elf.total_calories() {
            max_calories_elf = elf;
        }
    }

    Some(max_calories_elf)
}

fn get_top_3_elves_with_most_calories(elves: &Vec<Elf>) -> Vec<&Elf> {
    if elves.len() < 3 {
        panic!("not enough Elves")
    }

    let mut elves: Vec<&Elf> = elves.iter().collect();

    elves.sort_by_key(|a| a.total_calories());
    elves.reverse();

    elves.iter().take(3).copied().collect()
}

fn read_input(reader: &mut impl Read) -> Result<Vec<Elf>, ParseIntError> {
    let mut buffer = String::new();

    reader
        .read_to_string(&mut buffer)
        .expect("could not read input");

    if buffer.trim().is_empty() {
        return Ok(vec![]);
    }

    process_input(buffer.as_str())
}

fn process_input(input: &str) -> Result<Vec<Elf>, ParseIntError> {
    input
        .trim()
        .split("\n\n")
        .map(|inventory_input| {
            let inventory: Result<Vec<u32>, ParseIntError> = inventory_input
                .lines()
                .map(|line| line.parse::<u32>())
                .collect();

            Ok(Elf {
                inventory: inventory?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<Elf> {
        vec![
            Elf {
                inventory: vec![1000, 2000, 3000],
            },
            Elf {
                inventory: vec![4000],
            },
            Elf {
                inventory: vec![5000, 6000],
            },
            Elf {
                inventory: vec![7000, 8000, 9000],
            },
            Elf {
                inventory: vec![10000],
            },
        ]
    }

    #[test]
    fn reads_and_parses_example_input() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let values = read_input(&mut input.as_bytes()).unwrap();
        let expected = example_input();

        assert_eq!(values, expected)
    }

    #[test]
    fn reads_and_parses_empty_input() {
        let input = "";
        let values = read_input(&mut input.as_bytes()).unwrap();

        assert!(values.is_empty());
    }

    #[test]
    fn reads_and_parses_input_with_only_new_lines() {
        let input = "\n\n\n\n\n";
        let values = read_input(&mut input.as_bytes()).unwrap();

        assert!(values.is_empty());
    }

    #[test]
    fn find_elf_with_most_calories_with_example_input() {
        let values = example_input();
        let result = find_elf_with_most_calories(&values);

        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            &Elf {
                inventory: vec![7000, 8000, 9000]
            }
        )
    }

    #[test]
    fn find_elf_with_most_calories_with_empty_input() {
        let values = vec![];
        let result = find_elf_with_most_calories(&values);
        assert!(result.is_none());
    }

    #[test]
    fn get_top_3_elves_with_most_calories_with_example_input() {
        let values = example_input();
        let result = get_top_3_elves_with_most_calories(&values);

        assert_eq!(
            result,
            vec![
                &Elf {
                    inventory: vec![7000, 8000, 9000],
                },
                &Elf {
                    inventory: vec![5000, 6000],
                },
                &Elf {
                    inventory: vec![10000],
                },
            ]
        )
    }
}
