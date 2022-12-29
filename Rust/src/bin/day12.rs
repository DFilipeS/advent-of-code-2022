use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::stdin,
    process::exit,
};

use advent_of_code::read_input;
use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Heightmap {
    values: Vec<Vec<usize>>,
    start: Point,
    end: Point,
}

#[derive(Debug, Eq)]
struct Position {
    coordinates: Point,
    score: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Heightmap {
    fn new(heightmap: &[Vec<char>]) -> Heightmap {
        let start = Heightmap::find_position(heightmap, 'S').unwrap();
        let end = Heightmap::find_position(heightmap, 'E').unwrap();
        let values: Vec<Vec<usize>> = heightmap
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&val| match val {
                        'S' => 0,
                        'E' => 25,
                        _ => val as usize - 'a' as usize,
                    })
                    .collect()
            })
            .collect();

        Heightmap { values, start, end }
    }

    fn find_position(heightmap: &[Vec<char>], needle: char) -> Option<Point> {
        for (y, row) in heightmap.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val == needle {
                    return Some(Point(x, y));
                }
            }
        }

        None
    }

    fn get_neighbors(&self, Point(x, y): Point) -> Vec<Point> {
        let value = self.values[y][x];
        let mut neighbors = Vec::new();

        // Top neighbor
        if y > 0 {
            let other_value = self.values[y - 1][x];
            if other_value <= value + 1 {
                neighbors.push(Point(x, y - 1));
            }
        }

        // Bottom neighbor
        if y < self.values.len() - 1 {
            let other_value = self.values[y + 1][x];
            if other_value <= value + 1 {
                neighbors.push(Point(x, y + 1));
            }
        }

        // Left neighbor
        if x > 0 {
            let other_value = self.values[y][x - 1];
            if other_value <= value + 1 {
                neighbors.push(Point(x - 1, y));
            }
        }

        // Right neighbor
        if x < self.values[y].len() - 1 {
            let other_value = self.values[y][x + 1];
            if other_value <= value + 1 {
                neighbors.push(Point(x + 1, y));
            }
        }

        neighbors
    }
}

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, heightmap) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Coult not parse input: {:?}", err);
        exit(2);
    });

    println!(
        "Shortest path length from start: {}",
        shortest_path(&heightmap, heightmap.start).unwrap().len()
    );

    println!(
        "Shortest path length: {}",
        min_shortest_path(&heightmap).len()
    );
}

fn min_shortest_path(heightmap: &Heightmap) -> Vec<Point> {
    let mut min_path_size = usize::MAX;
    let mut min_path = Vec::new();
    let mut checked_start_point: HashSet<Point> = HashSet::new();

    for (y, row) in heightmap.values.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let point = Point(x, y);

            if val == 0 && !checked_start_point.contains(&point) {
                checked_start_point.insert(point);

                if let Some(mut path) = shortest_path(heightmap, point) {
                    for window in path.clone().windows(2) {
                        if let &[_, p_next] = window {
                            let &next_value = heightmap
                                .values
                                .get(p_next.1)
                                .unwrap()
                                .get(p_next.0)
                                .unwrap();

                            if next_value != 0 {
                                break;
                            }
                            path.remove(0);
                            checked_start_point.insert(p_next);
                        }
                    }

                    if path.len() < min_path_size {
                        min_path_size = path.len();
                        min_path = path;
                    }
                }
            }
        }
    }

    min_path
}

/// Finds the shortest path between the start and end points in the given
/// `heightmap` using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
fn shortest_path(heightmap: &Heightmap, start: Point) -> Option<Vec<Point>> {
    let mut heap: BinaryHeap<Position> = BinaryHeap::new();
    heap.push(Position {
        coordinates: start,
        score: manhattan_distance(start, heightmap.end),
    });

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while !heap.is_empty() {
        let current = heap.pop().unwrap().coordinates;

        if current == heightmap.end {
            return Some(reconstruct_path(&came_from, current));
        }

        for neighbor in heightmap.get_neighbors(current) {
            let tentative_g_score = g_score.get(&current).unwrap_or(&usize::MAX) + 1;
            if &tentative_g_score < g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);

                if !heap.iter().any(|x| x.coordinates == neighbor) {
                    heap.push(Position {
                        coordinates: neighbor,
                        score: tentative_g_score + manhattan_distance(neighbor, heightmap.end),
                    })
                }
            }
        }
    }

    None
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut cursor = current;

    while came_from.contains_key(&cursor) {
        cursor = *came_from.get(&cursor).unwrap();
        path.insert(0, cursor);
    }

    path
}

fn manhattan_distance(Point(x1, y1): Point, Point(x2, y2): Point) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn parse_input(input: &str) -> IResult<&str, Heightmap> {
    let (input, heightmap) =
        separated_list1(newline, alpha1.map(|l: &str| l.chars().collect()))(input)?;
    let heightmap = Heightmap::new(&heightmap);

    Ok((input, heightmap))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parse_input_with_example_input() {
        let (input, result) = parse_input(EXAMPLE_INPUT).unwrap();
        let values = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        let expected = Heightmap {
            values,
            start: Point(0, 0),
            end: Point(5, 2),
        };

        assert_eq!(result, expected);
        assert!(input.is_empty());
    }

    #[test]
    fn find_position_with_example_input() {
        let heightmap = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];
        let starting_position = Heightmap::find_position(&heightmap, 'S');
        let final_position = Heightmap::find_position(&heightmap, 'E');

        assert_eq!(starting_position, Some(Point(0, 0)));
        assert_eq!(final_position, Some(Point(5, 2)));
    }

    #[test]
    fn get_neighbors_with_example_input() {
        let values = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        let heightmap = Heightmap {
            values,
            start: Point(0, 0),
            end: Point(5, 2),
        };

        let result = Heightmap::get_neighbors(&heightmap, Point(0, 0));
        assert_eq!(result, vec![Point(0, 1), Point(1, 0)]);

        let result = Heightmap::get_neighbors(&heightmap, Point(5, 2));
        assert_eq!(
            result,
            vec![Point(5, 1), Point(5, 3), Point(4, 2), Point(6, 2)]
        );

        let result = Heightmap::get_neighbors(&heightmap, Point(2, 0));
        assert_eq!(result, vec![Point(2, 1), Point(1, 0)]);
    }

    #[test]
    fn shortest_path_with_example_input() {
        let values = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        let heightmap = Heightmap {
            values,
            start: Point(0, 0),
            end: Point(5, 2),
        };
        let result = shortest_path(&heightmap, heightmap.start).unwrap();

        assert_eq!(result.len(), 31);
    }

    #[test]
    fn min_shortest_path_with_example_input() {
        let values = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        let heightmap = Heightmap {
            values,
            start: Point(0, 0),
            end: Point(5, 2),
        };
        let result = min_shortest_path(&heightmap);

        assert_eq!(result.len(), 29);
    }

    #[test]
    fn manhattan_distance_with_example_input_start_and_end() {
        let start = Point(0, 0);
        let end = Point(5, 2);
        let result = manhattan_distance(start, end);

        assert_eq!(result, 7);
    }
}
