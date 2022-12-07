use std::{io::stdin, process::exit};

use advent_of_code::read_input;

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });

    match find_marker_position(input.as_str(), 4) {
        Some(value) => println!("Start-of-packet marker position: {}", value),
        None => println!("Start-of-packet marker not found in given input"),
    };

    match find_marker_position(input.as_str(), 14) {
        Some(value) => println!("Start-of-message marker position: {}", value),
        None => println!("Start-of-message marker not found in given input"),
    };
}

/// Finds the position of the start-of-packet or start-of-message marker, based
/// on the given `offset`. Returns an `Option<usize>`, where `None` is for the
/// cases where the marker is not detected on the given `input`.
fn find_marker_position(input: &str, offset: usize) -> Option<usize> {
    let mut last_chars: Vec<char> = input.trim().chars().take(offset - 1).collect();

    for (i, c) in input.chars().enumerate().skip(offset - 1) {
        last_chars.push(c);

        if !has_duplicates(&last_chars) {
            return Some(i + 1);
        }

        last_chars.remove(0);
    }

    None
}

fn has_duplicates(chars: &Vec<char>) -> bool {
    let mut seen = vec![];

    for c in chars {
        if seen.contains(c) {
            return true;
        }
        seen.push(*c);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_duplicates_with_example_input_parts() {
        let inputs = vec![
            (vec!['m', 'j', 'q', 'j'], true),
            (vec!['j', 'q', 'j', 'p'], true),
            (vec!['j', 'p', 'q', 'm'], false),
        ];

        for (input, expected) in inputs {
            let result = has_duplicates(&input);

            assert_eq!(
                result, expected,
                "wanted {}, got {} for {:?}",
                expected, result, input
            );
        }
    }

    #[test]
    fn find_marker_position_with_example_inputs() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ];

        for (input, offset, expected) in inputs {
            let result = find_marker_position(input, offset).unwrap();

            assert_eq!(
                result, expected,
                "wanted {}, got {} for {}",
                expected, result, input
            );
        }
    }

    #[test]
    fn find_marker_position_with_empty_input() {
        let input = "";
        let result = find_marker_position(input, 4);

        assert!(result.is_none());
    }

    #[test]
    fn find_marker_position_with_short_input() {
        let input = "abc";
        let result = find_marker_position(input, 4);

        assert!(result.is_none());
    }
}
