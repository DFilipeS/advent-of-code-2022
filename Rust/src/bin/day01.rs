use std::io::Read;

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
    let elves = read_input(&mut std::io::stdin());
    let most_calories_elf = find_elf_with_most_calories(&elves).expect("there are no Elves");
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

fn read_input(reader: &mut impl Read) -> Vec<Elf> {
    let mut buffer = String::new();

    reader
        .read_to_string(&mut buffer)
        .expect("could not read input");

    if buffer.trim().is_empty() {
        return vec![];
    }

    process_input(buffer.as_str())
}

fn process_input(input: &str) -> Vec<Elf> {
    input
        .trim()
        .split("\n\n")
        .map(|inventory_input| {
            let inventory = inventory_input
                .lines()
                .map(|line| line.parse().expect("invalid integer"))
                .collect();

            Elf { inventory }
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
        let values = read_input(&mut input.as_bytes());
        let expected = example_input();

        assert_eq!(values, expected)
    }

    #[test]
    fn reads_and_parses_empty_input() {
        let input = "";
        let values = read_input(&mut input.as_bytes());

        assert!(values.is_empty());
    }

    #[test]
    fn reads_and_parses_input_with_only_new_lines() {
        let input = "\n\n\n\n\n";
        let values = read_input(&mut input.as_bytes());

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
