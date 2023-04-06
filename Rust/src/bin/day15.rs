use std::{io::stdin, process::exit};

use advent_of_code::read_input;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    closest_beacon_distance: i32,
}

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, sensors) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Could not parse input: {:?}", err);
        exit(2);
    });

    let positions_without_beacons = count_positions_without_beacons(&sensors, 2_000_000);
    println!(
        "Number of positions without beacons: {}",
        positions_without_beacons
    );

    let distress_beacon_frequency = find_distress_beacon_frequency(&sensors, 4_000_000);
    println!("Distress beacon frequency: {}", distress_beacon_frequency);
}

fn count_positions_without_beacons(sensors: &[(Sensor, Beacon)], target_row: i32) -> usize {
    sensors
        .iter()
        .filter(|(sensor, _)| {
            (sensor.y - sensor.closest_beacon_distance) <= target_row
                && (sensor.y + sensor.closest_beacon_distance) >= target_row
        })
        .flat_map(|(sensor, _)| {
            let distance = sensor.closest_beacon_distance - (sensor.y - target_row).abs();

            (sensor.x - distance)..=(sensor.x + distance)
        })
        .unique()
        .filter(|x| {
            !sensors
                .iter()
                .any(|(_, beacon)| beacon.x == *x && beacon.y == target_row)
        })
        .count()
}

fn find_distress_beacon_frequency(sensors: &[(Sensor, Beacon)], limit: i32) -> i64 {
    let position = sensors
        .iter()
        .flat_map(|(sensor, _)| {
            let radius = sensor.closest_beacon_distance;

            (-radius - 1..=radius + 1)
                .flat_map(move |n| {
                    // Get points at the edge of the sensor range
                    let line_size = n.abs() - (radius + 1);
                    let y = sensor.y + n;
                    let point_1 = (sensor.x - line_size.abs(), y);
                    let point_2 = (sensor.x + line_size.abs(), y);

                    vec![point_1, point_2]
                })
                .filter(|&(x, y)| {
                    // Filter points outside the limit
                    x >= 0 && x <= limit && y >= 0 && y <= limit
                })
        })
        .find(|(x, y)| is_available_position(sensors, *x, *y));

    match position {
        Some((x, y)) => x as i64 * 4_000_000 + y as i64,
        None => panic!("No empty position found!"),
    }
}

fn is_available_position(sensors: &[(Sensor, Beacon)], x: i32, y: i32) -> bool {
    sensors.iter().all(|(sensor, _)| {
        let sensor_distance = manhattan_distance((sensor.x, sensor.y), (x, y));

        sensor_distance > sensor.closest_beacon_distance
    })
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    separated_list0(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, (mut sensor, beacon)) = separated_pair(
        preceded(
            tag("Sensor at "),
            parse_position.map(|(x, y)| Sensor {
                x,
                y,
                closest_beacon_distance: 0,
            }),
        ),
        tag(": closest beacon is at "),
        parse_position.map(|(x, y)| Beacon { x, y }),
    )(input)?;

    sensor.closest_beacon_distance = manhattan_distance((sensor.x, sensor.y), (beacon.x, beacon.y));

    Ok((input, (sensor, beacon)))
}

fn parse_position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day15_example.txt");

    #[test]
    fn parse_input_with_example() {
        let (input, result) = parse_input(INPUT).unwrap();

        assert!(input.is_empty());
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn count_positions_without_beacons_with_example() {
        let (_, sensors) = parse_input(INPUT).unwrap();
        let result = count_positions_without_beacons(&sensors, 10);

        assert_eq!(result, 26);
    }

    #[test]
    fn find_distress_beacon_frequency_with_example() {
        let (_, sensors) = parse_input(INPUT).unwrap();
        let result = find_distress_beacon_frequency(&sensors, 20);

        assert_eq!(result, 56000011);
    }
}
