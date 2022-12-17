use std::{collections::HashSet, io::stdin, process::exit, str::FromStr};

use advent_of_code::read_input;

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct Point(isize, isize);

impl Point {
    fn is_adjacent(&self, other: &Point) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }

    fn move_right(&mut self) {
        self.0 += 1;
    }

    fn move_left(&mut self) {
        self.0 -= 1;
    }

    fn move_up(&mut self) {
        self.1 += 1;
    }

    fn move_down(&mut self) {
        self.1 -= 1;
    }

    fn row(&self) -> isize {
        self.0
    }

    fn column(&self) -> isize {
        self.1
    }

    fn find_and_set_adjacent_diagonal(&mut self, other: &Point) {
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 || y == 0 {
                    continue;
                }

                let point = Point(self.0 + x, self.1 + y);
                if point.is_adjacent(other) {
                    self.0 += x;
                    self.1 += y;
                    return;
                }
            }
        }
    }

    fn find_and_set_adjacent_non_diagonal(&mut self, other: &Point) {
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 && y != 0 {
                    continue;
                }

                let point = Point(self.0 + x, self.1 + y);
                if point.is_adjacent(other) {
                    self.0 += x;
                    self.1 += y;
                    return;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let value: usize = tokens
            .get(1)
            .ok_or("motion value not found")?
            .parse()
            .map_err(|_| "could not parse motion value")?;

        let motion = match tokens.first() {
            Some(&"R") => Self::Right(value),
            Some(&"L") => Self::Left(value),
            Some(&"U") => Self::Up(value),
            Some(&"D") => Self::Down(value),
            Some(_) => {
                return Err("invalid direction".to_string());
            }
            None => {
                return Err("direction not found".to_string());
            }
        };

        Ok(motion)
    }
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let motions = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Coult not parse input: {:?}", err);
        exit(2);
    });

    println!(
        "Tail positions with rope of length 2: {}",
        unique_tail_positions(&motions, 2).len()
    );

    println!(
        "Tail positions with rope of length 10: {}",
        unique_tail_positions(&motions, 10).len()
    );
}

fn unique_tail_positions(motions: &Vec<Motion>, rope_length: usize) -> HashSet<Point> {
    let mut positions = HashSet::new();
    let knots = (0..rope_length).map(|_| Point::default()).collect();
    let mut rope = Rope {
        knots,
        ..Default::default()
    };

    positions.insert(Point(0, 0));
    for motion in motions {
        rope = apply_motion(&rope, motion);
    }

    rope.visited
}

fn apply_motion(rope: &Rope, motion: &Motion) -> Rope {
    let mut rope = rope.clone();

    let steps = match motion {
        Motion::Right(steps) => *steps,
        Motion::Left(steps) => *steps,
        Motion::Up(steps) => *steps,
        Motion::Down(steps) => *steps,
    };

    for _ in 0..steps {
        let mut knots = vec![];

        // Process first knot of the rope.
        let mut head = rope.knots.first().cloned().unwrap();

        match motion {
            Motion::Right(_) => head.move_right(),
            Motion::Left(_) => head.move_left(),
            Motion::Up(_) => head.move_up(),
            Motion::Down(_) => head.move_down(),
        };

        knots.push(head);

        // Process remaining knots.
        for (i, mut current) in rope.knots.iter().cloned().enumerate().skip(1) {
            let previous = knots.get(i - 1).unwrap();

            if !current.is_adjacent(previous) {
                if current.row() != previous.row() && current.column() != previous.column() {
                    current.find_and_set_adjacent_diagonal(previous);
                } else {
                    current.find_and_set_adjacent_non_diagonal(previous);
                }
            }

            knots.push(current);
        }

        rope.visited.insert(knots.last().cloned().unwrap());
        rope.knots = knots;
    }

    rope
}

fn parse_input(input: &str) -> Result<Vec<Motion>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, vec};

    use super::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn motion_from_str_with_examples() {
        let test_cases = vec![
            ("R 4", Motion::Right(4)),
            ("U 10", Motion::Up(10)),
            ("L 3", Motion::Left(3)),
            ("D 1", Motion::Down(1)),
        ];

        for (input, expected) in test_cases {
            let result = Motion::from_str(input).unwrap();
            assert_eq!(
                result, expected,
                "wanted {:?}, got {:?} with {}",
                expected, result, input
            );
        }
    }

    #[test]
    fn parse_input_with_example_input() {
        let result = parse_input(EXAMPLE_INPUT).unwrap();
        let expected = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn is_adjacent_with_examples() {
        let test_cases = vec![
            (Point(0, 0), Point(0, 0), true),
            (Point(0, 0), Point(0, 1), true),
            (Point(0, 0), Point(1, 0), true),
            (Point(0, 0), Point(1, 1), true),
            (Point(0, 0), Point(-1, -1), true),
            (Point(0, 0), Point(-1, 1), true),
            (Point(0, 0), Point(2, 0), false),
            (Point(0, 0), Point(0, 2), false),
            (Point(0, 0), Point(2, 2), false),
        ];

        for (p1, p2, expected) in test_cases {
            let result = p1.is_adjacent(&p2);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn apply_motion_with_examples() {
        let mut rope = Rope {
            knots: vec![Point::default(), Point::default()],
            ..Default::default()
        };
        let motions = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];
        let expected = Rope {
            knots: vec![Point(2, 2), Point(1, 2)],
            visited: HashSet::from_iter(vec![
                Point(2, 2),
                Point(0, 0),
                Point(3, 4),
                Point(2, 0),
                Point(4, 1),
                Point(3, 2),
                Point(4, 2),
                Point(2, 4),
                Point(4, 3),
                Point(1, 0),
                Point(3, 3),
                Point(3, 0),
                Point(1, 2),
            ]),
        };

        for motion in motions {
            rope = apply_motion(&rope, &motion);
        }

        assert_eq!(rope, expected);
    }

    #[test]
    fn apply_motion_with_diagonal_movement() {
        let mut rope = Rope {
            knots: vec![Point(4, 1), Point(3, 0), Point(2, 0)],
            ..Default::default()
        };
        let motions = vec![Motion::Up(1)];
        let expected = Rope {
            knots: vec![Point(4, 2), Point(4, 1), Point(3, 1)],
            visited: HashSet::from_iter(vec![Point(3, 1)]),
        };

        for motion in motions {
            rope = apply_motion(&rope, &motion);
        }

        assert_eq!(rope, expected);
    }

    #[test]
    fn unique_tail_positions_with_example_input_and_rope_length_2() {
        let motions = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];
        let positions: HashSet<Point> = unique_tail_positions(&motions, 2);

        assert_eq!(positions.len(), 13);
    }

    #[test]
    fn unique_tail_positions_with_example_input_and_rope_length_10() {
        let motions = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];
        let positions: HashSet<Point> = unique_tail_positions(&motions, 10);

        assert_eq!(positions.len(), 1);
    }

    #[test]
    fn unique_tail_positions_with_larger_example_input() {
        let motions = vec![
            Motion::Right(5),
            Motion::Up(8),
            Motion::Left(8),
            Motion::Down(3),
            Motion::Right(17),
            Motion::Down(10),
            Motion::Left(25),
            Motion::Up(20),
        ];
        let positions: HashSet<Point> = unique_tail_positions(&motions, 10);

        assert_eq!(positions.len(), 36);
    }
}
