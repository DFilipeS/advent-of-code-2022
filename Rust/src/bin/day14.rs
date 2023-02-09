use std::{collections::BTreeSet, io::stdin, process::exit};

use advent_of_code::read_input;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser, ToUsize,
};

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, mut points) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Could not parse input: {:?}", err);
        exit(2);
    });

    let total = process_sand(&mut points.clone());
    println!("Total units of sand at rest: {}", total);

    let total_with_floor = process_sand_with_floor(&mut points);
    println!(
        "Total units of sand at rest with floor: {}",
        total_with_floor
    );
}

fn process_sand(points_set: &mut BTreeSet<(usize, usize)>) -> usize {
    let initial_size = points_set.len();
    let maximum_depth: usize = points_set.iter().map(|(_, y)| y).max().cloned().unwrap();
    let mut sand: (usize, usize) = (500, 0);

    while sand.1 <= maximum_depth {
        if !points_set.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !points_set.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if !points_set.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            points_set.insert(sand);
            sand = (500, 0);
        }
    }

    points_set.len() - initial_size
}

fn process_sand_with_floor(points_set: &mut BTreeSet<(usize, usize)>) -> usize {
    let initial_size = points_set.len();
    let maximum_depth: usize = points_set.iter().map(|(_, y)| y).max().cloned().unwrap() + 2;
    let mut sand: (usize, usize) = (500, 0);

    while !points_set.contains(&(500, 0)) {
        if sand.1 + 1 != maximum_depth && !points_set.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if sand.1 + 1 != maximum_depth && !points_set.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if sand.1 + 1 != maximum_depth && !points_set.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            points_set.insert(sand);
            sand = (500, 0);
        }
    }

    points_set.len() - initial_size
}

fn parse_input(input: &str) -> IResult<&str, BTreeSet<(usize, usize)>> {
    let (input, paths) = separated_list1(
        line_ending,
        separated_list1(
            tag(" -> "),
            separated_pair(
                complete::u32.map(|n| n.to_usize()),
                tag(","),
                complete::u32.map(|n| n.to_usize()),
            ),
        ),
    )(input)?;

    Ok((input, get_points_set_from_paths(paths)))
}

fn get_points_set_from_paths(paths: Vec<Vec<(usize, usize)>>) -> BTreeSet<(usize, usize)> {
    let mut points = BTreeSet::new();

    for path in paths {
        for ((x1, y1), (x2, y2)) in path.into_iter().tuple_windows() {
            let x_range = x1.min(x2)..=x1.max(x2);
            let y_range = y1.min(y2)..=y1.max(y2);

            for p in x_range.cartesian_product(y_range) {
                points.insert(p);
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parse_input_with_example_input() {
        let (input, result) = parse_input(EXAMPLE_INPUT).unwrap();
        let expected = example_input_set();

        assert_eq!(result, expected);
        assert!(input.is_empty());
    }

    #[test]
    fn process_sand_with_example_input() {
        let mut points_set = example_input_set();
        let result = process_sand(&mut points_set);

        assert_eq!(result, 24);
    }

    #[test]
    fn process_sand_with_floor_with_example_input() {
        let mut points_set = example_input_set();
        let result = process_sand_with_floor(&mut points_set);

        assert_eq!(result, 93);
    }

    fn example_input_set() -> BTreeSet<(usize, usize)> {
        BTreeSet::from([
            (498, 4),
            (498, 5),
            (498, 6),
            (497, 6),
            (496, 6),
            (503, 4),
            (502, 4),
            (502, 5),
            (502, 6),
            (502, 7),
            (502, 8),
            (502, 9),
            (501, 9),
            (500, 9),
            (499, 9),
            (498, 9),
            (497, 9),
            (496, 9),
            (495, 9),
            (494, 9),
        ])
    }
}
