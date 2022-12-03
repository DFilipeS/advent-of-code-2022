use std::{collections::HashSet, process::exit};

use advent_of_code::read_input;

#[derive(Debug)]
enum Error {
    RepeatedItemDoesNotExist,
    CommonItemDoesNotExist,
    InventoryNotFound,
}

fn main() {
    let input = match read_input(&mut std::io::stdin()) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to read input: {:?}", err);
            exit(1);
        }
    };

    let priorities_sum = match get_sum_of_priorities_for_repeated_items(input.as_str()) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to get sum of priorities: {:?}", err);
            exit(2);
        }
    };
    println!("Sum of priorities for repeated items: {}", priorities_sum);

    let group_priorities_sum = match get_sum_of_priorities_for_groups(input.as_str()) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to get sum of group priorities: {:?}", err);
            exit(3);
        }
    };
    println!("Sum of group priorities: {}", group_priorities_sum);
}

fn parse_rucksack_compartments(input: &str) -> (HashSet<char>, HashSet<char>) {
    let (first, second) = input.trim().split_at(input.len() / 2);

    (
        HashSet::from_iter(first.chars()),
        HashSet::from_iter(second.chars()),
    )
}

fn find_repeated_item<'a>(first: &'a HashSet<char>, second: &'a HashSet<char>) -> Option<&'a char> {
    first.intersection(second).next()
}

fn get_priority(item_type: &char) -> u32 {
    if item_type.is_uppercase() {
        return *item_type as u32 - 38;
    }

    *item_type as u32 - 96
}

fn get_sum_of_priorities_for_repeated_items(input: &str) -> Result<u32, Error> {
    input
        .lines()
        .map(|line| {
            let (first, second) = parse_rucksack_compartments(line);
            let repeated_item =
                find_repeated_item(&first, &second).ok_or(Error::RepeatedItemDoesNotExist)?;

            Ok(get_priority(repeated_item))
        })
        .sum()
}

fn get_sum_of_priorities_for_groups(input: &str) -> Result<u32, Error> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(get_common_item_from_chunk)
        .sum()
}

fn get_common_item_from_chunk(chunk: &[&str]) -> Result<u32, Error> {
    let intersection = chunk
        .iter()
        .map(|input| HashSet::from_iter(input.chars()))
        .reduce(|acc, item| acc.intersection(&item).cloned().collect::<HashSet<char>>())
        .ok_or(Error::InventoryNotFound)?;

    let common_item = intersection
        .iter()
        .next()
        .ok_or(Error::CommonItemDoesNotExist)?;

    Ok(get_priority(common_item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_with_first_example_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n";
        let values = parse_rucksack_compartments(input);
        let expected = (
            HashSet::from_iter("vJrwpWtwJgWr".chars()),
            HashSet::from_iter("hcsFMMfFFhFp".chars()),
        );

        assert_eq!(values, expected);
    }

    #[test]
    fn find_repeated_char_with_first_jexample_input() {
        let first = HashSet::from_iter("vJrwpWtwJgWr".chars());
        let second = HashSet::from_iter("hcsFMMfFFhFp".chars());
        let result = find_repeated_item(&first, &second).unwrap();

        assert_eq!(result, &'p');
    }

    #[test]
    fn get_priority_with_first_example_input() {
        let expected = 16;
        let result = get_priority(&'p');

        assert_eq!(result, expected);
    }

    #[test]
    fn get_priority_with_second_example_input() {
        let expected = 38;
        let result = get_priority(&'L');

        assert_eq!(result, expected);
    }

    #[test]
    fn get_sum_of_priorities_for_repeated_items_with_example_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = get_sum_of_priorities_for_repeated_items(input).unwrap();

        assert_eq!(result, 157);
    }

    #[test]
    fn get_sum_of_priorities_for_groups_with_example_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = get_sum_of_priorities_for_groups(input).unwrap();

        assert_eq!(result, 70);
    }
}
