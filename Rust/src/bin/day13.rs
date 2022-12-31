use std::{io::stdin, process::exit};

use advent_of_code::read_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, packets) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Could not parse input: {:?}", err);
        exit(2);
    });

    println!(
        "Sum of indexes of pairs in right order: {}",
        pairs_in_right_order(&packets).iter().sum::<usize>()
    );
    println!("Decoder key: {}", find_decoder_key(&packets));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketElement {
    List(Vec<PacketElement>),
    Number(u32),
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketElement::List(list_1), PacketElement::List(list_2)) => {
                list_1.partial_cmp(list_2)
            }
            (PacketElement::List(list), PacketElement::Number(other_val)) => {
                let other_list = &vec![PacketElement::Number(*other_val)];
                list.partial_cmp(other_list)
            }
            (PacketElement::Number(val), PacketElement::List(other_list)) => {
                let list = &vec![PacketElement::Number(*val)];
                list.partial_cmp(other_list)
            }
            (PacketElement::Number(val), PacketElement::Number(other_val)) => {
                val.partial_cmp(other_val)
            }
        }
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn pairs_in_right_order(packets: &[Vec<PacketElement>]) -> Vec<usize> {
    packets
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            let first = pair.first().unwrap();
            let second = pair.get(1).unwrap();

            if first < second {
                Some(index + 1)
            } else {
                None
            }
        })
        .collect()
}

fn find_decoder_key(packets: &[Vec<PacketElement>]) -> usize {
    let mut packets: Vec<PacketElement> = packets.iter().flatten().cloned().collect();
    let divider_packets = vec![
        PacketElement::List(vec![PacketElement::List(vec![PacketElement::Number(2)])]),
        PacketElement::List(vec![PacketElement::List(vec![PacketElement::Number(6)])]),
    ];
    packets.append(&mut divider_packets.clone());
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if divider_packets.contains(packet) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<PacketElement>>> {
    separated_list1(tag("\n\n"), parse_pairs)(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<PacketElement>> {
    separated_list1(newline, parse_list)(input)
}

fn parse_number(input: &str) -> IResult<&str, PacketElement> {
    let (input, number) = nom::character::complete::u32(input)?;
    let element = PacketElement::Number(number);

    Ok((input, element))
}

fn parse_list(input: &str) -> IResult<&str, PacketElement> {
    let (input, list) = delimited(
        tag("["),
        separated_list0(tag(","), alt((parse_number, parse_list))),
        tag("]"),
    )(input)?;

    let element = PacketElement::List(list);

    Ok((input, element))
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::PacketElement::List;
    use super::PacketElement::Number;
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn parse_input_with_example_input() {
        let (input, result) = parse_input(EXAMPLE_INPUT).unwrap();
        let expected: Vec<Vec<PacketElement>> = vec![
            vec![
                List(vec![Number(1), Number(1), Number(3), Number(1), Number(1)]),
                List(vec![Number(1), Number(1), Number(5), Number(1), Number(1)]),
            ],
            vec![
                List(vec![
                    List(vec![Number(1)]),
                    List(vec![Number(2), Number(3), Number(4)]),
                ]),
                List(vec![List(vec![Number(1)]), Number(4)]),
            ],
            vec![
                List(vec![Number(9)]),
                List(vec![List(vec![Number(8), Number(7), Number(6)])]),
            ],
            vec![
                List(vec![List(vec![Number(4), Number(4)]), Number(4), Number(4)]),
                List(vec![
                    List(vec![Number(4), Number(4)]),
                    Number(4),
                    Number(4),
                    Number(4),
                ]),
            ],
            vec![
                List(vec![Number(7), Number(7), Number(7), Number(7)]),
                List(vec![Number(7), Number(7), Number(7)]),
            ],
            vec![List(vec![]), List(vec![Number(3)])],
            vec![
                List(vec![List(vec![List(vec![])])]),
                List(vec![List(vec![])]),
            ],
            vec![
                List(vec![
                    Number(1),
                    List(vec![
                        Number(2),
                        List(vec![
                            Number(3),
                            List(vec![Number(4), List(vec![Number(5), Number(6), Number(7)])]),
                        ]),
                    ]),
                    Number(8),
                    Number(9),
                ]),
                List(vec![
                    Number(1),
                    List(vec![
                        Number(2),
                        List(vec![
                            Number(3),
                            List(vec![Number(4), List(vec![Number(5), Number(6), Number(0)])]),
                        ]),
                    ]),
                    Number(8),
                    Number(9),
                ]),
            ],
        ];

        assert_eq!(result, expected);
        assert!(input.is_empty());
    }

    #[test]
    fn pairs_in_right_order_with_example_input() {
        let (_, input) = parse_input(EXAMPLE_INPUT).unwrap();
        let result = pairs_in_right_order(&input);

        assert_eq!(result, vec![1, 2, 4, 6]);
    }

    #[test]
    fn find_decoder_key_with_example_input() {
        let (_, input) = parse_input(EXAMPLE_INPUT).unwrap();
        let result = find_decoder_key(&input);

        assert_eq!(result, 140);
    }
}
