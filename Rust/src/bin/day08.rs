use std::{io::stdin, process::exit};

use advent_of_code::read_input;
use nom::{
    character::complete::{anychar, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Left,
    Bottom,
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, matrix) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Coult not parse input: {:?}", err);
        exit(2);
    });

    println!(
        "Total of trees visible outside the grid: {}",
        total_visible(&matrix)
    );

    println!("Max scenic score: {}", max_scenic_score(&matrix));
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, matrix) = separated_list1(newline, parse_line)(input)?;

    Ok((input, matrix))
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, x) = many1(verify(anychar, |c| *c != '\n'))(input)?;
    let digits = x.iter().map(|c| c.to_digit(10).unwrap()).collect();

    Ok((input, digits))
}

fn total_visible(matrix: &[Vec<u32>]) -> u32 {
    let mut sum = 0;

    for (x, row) in matrix.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if is_visible(matrix, (x, y)) {
                sum += 1;
            }
        }
    }

    sum
}

fn is_visible(matrix: &[Vec<u32>], (x, y): (usize, usize)) -> bool {
    if x == 0 || y == 0 {
        return true;
    }

    let height = matrix[x][y];
    let mut directions_hidden = vec![];

    for row in matrix.iter().take(x).rev() {
        if row[y] >= height {
            directions_hidden.push(Direction::Top);
            break;
        }
    }

    for row in matrix.iter().skip(x + 1) {
        if row[y] >= height {
            directions_hidden.push(Direction::Bottom);
            break;
        }
    }

    for &value in matrix[x].iter().take(y).rev() {
        if value >= height {
            directions_hidden.push(Direction::Left);
            break;
        }
    }

    for &value in matrix[x].iter().skip(y + 1) {
        if value >= height {
            directions_hidden.push(Direction::Right);
            break;
        }
    }

    directions_hidden.len() < 4
}

fn max_scenic_score(matrix: &Vec<Vec<u32>>) -> usize {
    let mut max = 0;

    for (x, row) in matrix.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let score = scenic_score(matrix, (x, y));
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn scenic_score(matrix: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> usize {
    let height = matrix[x][y];
    let mut score = 1;

    let mut multiplier = x;
    for (i, row) in matrix.iter().enumerate().take(x).rev() {
        if row[y] >= height {
            multiplier = x - i;
            break;
        }
    }
    score *= multiplier;

    multiplier = matrix.len() - x - 1;
    for (i, row) in matrix.iter().enumerate().skip(x + 1) {
        if row[y] >= height {
            multiplier = i - x;
            break;
        }
    }
    score *= multiplier;

    multiplier = y;
    for (i, &value) in matrix[x].iter().enumerate().take(y).rev() {
        if value >= height {
            multiplier = y - i;
            break;
        }
    }
    score *= multiplier;

    multiplier = matrix[x].len() - y - 1;
    for (i, &value) in matrix[x].iter().enumerate().skip(y + 1) {
        if value >= height {
            multiplier = i - y;
            break;
        }
    }
    score *= multiplier;

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_with_example_input() {
        let input = "30373\n";
        let expected = vec![3, 0, 3, 7, 3];
        let (_, result) = parse_line(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_input_with_example_input() {
        let input = "30373
25512
65332
33549
35390";
        let (input, result) = parse_input(input).unwrap();
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert!(input.is_empty());
        assert_eq!(result, expected);
    }

    #[test]
    fn is_visible_with_example_input() {
        let test_cases = vec![
            ((0, 0), true),
            ((4, 0), true),
            ((1, 1), true),
            ((1, 2), true),
            ((1, 3), false),
            ((2, 1), true),
            ((2, 2), false),
            ((2, 3), true),
            ((3, 1), false),
            ((3, 2), true),
            ((3, 3), false),
        ];
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        for t in test_cases {
            let result = is_visible(&matrix, t.0);
            assert_eq!(
                result, t.1,
                "wanted {}, but got {} for {:?}",
                t.1, result, t.0
            );
        }
    }

    #[test]
    fn total_visible_with_example_input() {
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let result = total_visible(&matrix);

        assert_eq!(result, 21);
    }

    #[test]
    fn scenic_score_with_example_input() {
        let test_cases = vec![
            ((0, 0), 0),
            ((1, 1), 1),
            ((1, 2), 4),
            ((1, 3), 1),
            ((3, 2), 8),
        ];
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        for t in test_cases {
            let result = scenic_score(&matrix, t.0);
            assert_eq!(
                result, t.1,
                "wanted {}, but got {} for {:?}",
                t.1, result, t.0
            );
        }
    }

    #[test]
    fn max_scenic_score_with_example_input() {
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let result = max_scenic_score(&matrix);

        assert_eq!(result, 8);
    }
}
