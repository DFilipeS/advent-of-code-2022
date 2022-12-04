use std::{io::stdin, process::exit};

use advent_of_code::read_input;

#[derive(Debug, PartialEq, Eq)]
struct SectionRange {
    start: u32,
    end: u32,
}

impl SectionRange {
    fn fully_overlap(&self, other: &SectionRange) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn partially_overlap(&self, other: &SectionRange) -> bool {
        (self.end >= other.start) && (other.end >= self.start)
    }
}

#[derive(Debug)]
enum Error {
    MissingRangeParameter,
    MissingRange,
    InvalidRangeValue,
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Failed to read input: {:?}", err);
        exit(1);
    });
    let ranges = parse_input(input.as_str()).unwrap_or_else(|err| {
        eprintln!("Failed to parse input: {:?}", err);
        exit(2);
    });
    let total_fully_overlapping_sections =
        get_total_overlapping_sections(&ranges, &SectionRange::fully_overlap).unwrap_or_else(
            |err| {
                eprintln!(
                    "Failed to get total of fully overlapping sections: {:?}",
                    err
                );
                exit(3);
            },
        );
    let total_partially_overlapping_sections =
        get_total_overlapping_sections(&ranges, &SectionRange::partially_overlap).unwrap_or_else(
            |err| {
                eprintln!(
                    "Failed to get total of partilly overlapping sections: {:?}",
                    err
                );
                exit(3);
            },
        );

    println!(
        "Total of fully overlapping sections: {}",
        total_fully_overlapping_sections
    );
    println!(
        "Total of partially overlapping sections: {}",
        total_partially_overlapping_sections
    );
}

fn get_total_overlapping_sections(
    ranges: &[SectionRange],
    is_overlapping_fn: &dyn Fn(&SectionRange, &SectionRange) -> bool,
) -> Result<u32, Error> {
    let mut total = 0;

    for c in ranges.chunks(2) {
        let range_1 = c.first().ok_or(Error::MissingRange)?;
        let range_2 = c.get(1).ok_or(Error::MissingRange)?;

        if is_overlapping_fn(range_1, range_2) {
            total += 1;
        }
    }

    Ok(total)
}

fn parse_input(input: &str) -> Result<Vec<SectionRange>, Error> {
    if input.trim().is_empty() {
        return Ok(vec![]);
    }

    input
        .trim()
        .split(&['\n', ','])
        .map(|token| {
            let numbers: Vec<u32> = token
                .split('-')
                .map(|t| t.parse::<u32>().map_err(|_| Error::InvalidRangeValue))
                .collect::<Result<Vec<u32>, Error>>()?;
            let start = numbers.first().ok_or(Error::MissingRangeParameter)?;
            let end = numbers.get(1).ok_or(Error::MissingRangeParameter)?;

            Ok(SectionRange {
                start: *start,
                end: *end,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_with_example_input() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let expected = parsed_example_input();
        let result = parse_input(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_input_with_empty_input() {
        let input = "\n\n\n";
        let result = parse_input(input).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn fully_overlap_with_example() {
        let test_cases = [
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), false),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), false),
        ];

        for ((x1, y1), (x2, y2), expected) in test_cases {
            let range_1 = SectionRange { start: x1, end: y1 };
            let range_2 = SectionRange { start: x2, end: y2 };
            let result = range_1.fully_overlap(&range_2);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn partially_overlap_with_example() {
        let test_cases = [
            ((2, 4), (6, 8), false),
            ((6, 8), (2, 4), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), true),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), true),
        ];

        for test_case in test_cases {
            let ((x1, y1), (x2, y2), expected) = test_case;
            let range_1 = SectionRange { start: x1, end: y1 };
            let range_2 = SectionRange { start: x2, end: y2 };
            let result = range_1.partially_overlap(&range_2);

            assert_eq!(
                result, expected,
                "expected {}, got {} for test case {:?}",
                expected, result, test_case
            );
        }
    }

    #[test]
    fn get_total_of_fully_overlapping_sections_with_example_input() {
        let input = parsed_example_input();
        let result = get_total_overlapping_sections(&input, &SectionRange::fully_overlap).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn get_total_of_partially_overlapping_sections_with_example_input() {
        let input = parsed_example_input();
        let result =
            get_total_overlapping_sections(&input, &SectionRange::partially_overlap).unwrap();

        assert_eq!(result, 4);
    }

    fn parsed_example_input() -> Vec<SectionRange> {
        vec![
            SectionRange { start: 2, end: 4 },
            SectionRange { start: 6, end: 8 },
            SectionRange { start: 2, end: 3 },
            SectionRange { start: 4, end: 5 },
            SectionRange { start: 5, end: 7 },
            SectionRange { start: 7, end: 9 },
            SectionRange { start: 2, end: 8 },
            SectionRange { start: 3, end: 7 },
            SectionRange { start: 6, end: 6 },
            SectionRange { start: 4, end: 6 },
            SectionRange { start: 2, end: 6 },
            SectionRange { start: 4, end: 8 },
        ]
    }
}
