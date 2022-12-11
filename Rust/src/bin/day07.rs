use std::{collections::BTreeMap, io::stdin, process::exit};

use advent_of_code::read_input;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

/// I was getting some trouble with the parsing of this problem but thanks to
/// the video of Chris Biscardi (https://www.youtube.com/watch?v=t9OQ3ca8OWk) I
/// learned about `nom` and how to use it.
fn main() {
    let input = read_input(&mut stdin()).unwrap_or_else(|err| {
        eprintln!("Could not read input: {:?}", err);
        exit(1);
    });
    let (_, operations) = parse_input(&input).unwrap_or_else(|err| {
        eprintln!("Could not parse input: {:?}", err);
        exit(2);
    });
    let tree = build_tree(operations);

    println!(
        "Total sum of candidates for deletion: {}",
        get_total_sum_of_candidates_for_deletion(&tree)
    );

    println!(
        "Smallest directory size that needs to be deleted: {}",
        get_smallest_directory_size_to_be_deleted(&tree)
    );
}

fn get_total_sum_of_candidates_for_deletion(tree: &BTreeMap<String, Vec<Files>>) -> u32 {
    let mut sum = 0;

    for (dir_name, _) in tree.iter() {
        let dir_size = get_directory_size(tree, dir_name);
        if dir_size <= 100_000 {
            sum += dir_size;
        }
    }

    sum
}

fn get_smallest_directory_size_to_be_deleted(tree: &BTreeMap<String, Vec<Files>>) -> u32 {
    let total_space = 70_000_000;
    let used_space = get_directory_size(tree, "");
    let free_space = total_space - used_space;
    let required_space = 30_000_000 - free_space;
    let mut min_size = used_space;

    for (dir_name, _) in tree.iter() {
        let dir_size = get_directory_size(tree, dir_name);
        if dir_size >= required_space && dir_size < min_size {
            min_size = dir_size;
        }
    }

    min_size
}

fn get_directory_size(tree: &BTreeMap<String, Vec<Files>>, name: &str) -> u32 {
    let dir_files = tree.get(name).unwrap();
    let mut sum = 0;

    for f in dir_files {
        match f {
            Files::File { size, name: _ } => {
                sum += size;
            }
            Files::Dir(value) => {
                let child_name = vec![name, value].join("/");
                sum += get_directory_size(tree, child_name.as_str());
            }
        }
    }

    sum
}

fn build_tree(operations: Vec<Operation>) -> BTreeMap<String, Vec<Files>> {
    let mut path_stack: Vec<&str> = vec![];
    let mut fs_tree = BTreeMap::new();

    for op in operations {
        match op {
            Operation::Cd(Cd::Root) => {
                path_stack.push("");
            }
            Operation::Cd(Cd::Up) => {
                path_stack.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                path_stack.push(name);
            }
            Operation::Ls(files) => {
                let path = path_stack.join("/");

                fs_tree.entry(path.clone()).or_default();
                files.iter().for_each(|f| match f {
                    Files::File { size, name } => {
                        fs_tree
                            .entry(path.clone())
                            .and_modify(|vec: &mut Vec<Files>| {
                                vec.push(Files::File { size: *size, name });
                            });
                    }
                    Files::Dir(name) => {
                        fs_tree
                            .entry(path.clone())
                            .and_modify(|vec: &mut Vec<Files>| {
                                vec.push(Files::Dir(name));
                            });
                    }
                });
            }
        }
    }

    fs_tree
}

fn parse_input(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;

    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    Ok((input, op))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((directory, file)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Files::Dir(name)))
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        take_till(|c| c == '\n'),
    )(input)?;

    Ok((input, Files::File { size, name }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cd_with_examples() {
        let test_cases = &[
            ("$ cd /", Operation::Cd(Cd::Root)),
            ("$ cd ..", Operation::Cd(Cd::Up)),
            ("$ cd foo", Operation::Cd(Cd::Down("foo"))),
        ];

        for t in test_cases {
            let (_, result) = cd(t.0).unwrap();
            assert_eq!(
                result, t.1,
                "got {:?}, wanted {:?} for {:?}",
                result, t.1, t.0
            );
        }
    }

    #[test]
    fn parse_ls_with_example() {
        let input = "$ ls\ndir a\n14848514 b.txt\n";
        let expected = Operation::Ls(vec![
            Files::Dir("a"),
            Files::File {
                size: 14848514,
                name: "b.txt",
            },
        ]);
        let (_, result) = ls(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_directory_with_example() {
        let input = "dir a\n";
        let expected = Files::Dir("a");
        let (_, result) = directory(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_file_with_example() {
        let input = "14848514 b.txt\n";
        let expected = Files::File {
            size: 14848514,
            name: "b.txt",
        };
        let (_, result) = file(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_input_with_example_input() {
        let input = example_input();
        let (input, result) = parse_input(input).unwrap();

        assert!(input.is_empty());
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn build_tree_with_example_input() {
        let input = example_input();
        let (_, operations) = parse_input(input).unwrap();
        let tree = build_tree(operations);

        assert!(!tree.is_empty());
    }

    #[test]
    fn get_directory_size_with_example_input() {
        let test_cases = &[
            ("/a/e", 584),
            ("/a", 94853),
            ("/d", 24933642),
            ("", 48381165),
        ];
        let input = example_input();
        let (_, operations) = parse_input(input).unwrap();
        let tree = build_tree(operations);

        for t in test_cases {
            let result = get_directory_size(&tree, t.0);
            assert_eq!(result, t.1, "wanted {}, got {} for {}", t.1, result, t.0);
        }
    }

    #[test]
    fn get_total_sum_of_candidates_for_deletion_with_example_input() {
        let input = example_input();
        let (_, operations) = parse_input(input).unwrap();
        let tree = build_tree(operations);
        let result = get_total_sum_of_candidates_for_deletion(&tree);

        assert_eq!(result, 95437);
    }

    #[test]
    fn get_smallest_directory_size_to_be_deleted_with_example_input() {
        let input = example_input();
        let (_, operations) = parse_input(input).unwrap();
        let tree = build_tree(operations);
        let result = get_smallest_directory_size_to_be_deleted(&tree);

        assert_eq!(result, 24933642);
    }

    fn example_input() -> &'static str {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k" as _
    }
}
