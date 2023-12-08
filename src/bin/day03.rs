use std::collections::{HashMap, HashSet};

use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(3)?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> usize {
    find_valid_numbers(parse(input), input)
        .iter()
        .map(|Number { value, .. }| value)
        .sum()
}

fn part2(input: &str) -> usize {
    // find all part numbers
    // create a hashmap of start..end -> part number
    // find all the * symbols
    // generate the boundary for each
    // for each boundary number, look up part number
    // count the unique part numbers
    // keep only the ones with two adjacent part numbers

    // 467..114..
    // ...*...... <- 13 bounding box = 2,3,4,12,14,22,23,24  parts(2) -> 467 ... parts(22) -> 35, part(23) -> 35
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    let part_numbers = parse(input);
    let mut part_number_by_range = HashMap::new();
    for number in &part_numbers {
        for i in number.start..number.end {
            part_number_by_range.insert(i, number);
        }
    }

    let width = input.lines().next().unwrap().len();
    let data = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();

    let mut sum = 0;
    for (i, _) in data.match_indices('*') {
        let boundary = boundary(i, i + 1, width, data.len());

        let part_numbers = boundary
            .iter()
            .flat_map(|i| part_number_by_range.get(i))
            .collect::<HashSet<_>>();

        let parts = Vec::from_iter(part_numbers);
        let [part1, part2] = parts.as_slice() else {
            continue;
        };

        let gear_ratio = part1.value * part2.value;

        sum += gear_ratio;
    }

    sum
}

fn symbols(input: &str) -> HashSet<char> {
    input
        .chars()
        .filter(|&c| !c.is_ascii_digit() && c != '.' && !c.is_whitespace())
        .collect()
}

// there must be a better way to do this lol
fn boundary(start: usize, end: usize, width: usize, max: usize) -> Vec<usize> {
    let len = end - start;
    let mut output = vec![];

    // top row
    if start > width {
        let upper_left = start - width;
        for i in upper_left..upper_left + len {
            output.push(i);
        }
    }

    // left
    if start % width != 0 {
        output.push(start - 1);
    }

    // right
    if end % width != 0 {
        output.push(end);
    }

    // bottom row
    if end < max - width {
        let bottom_right = end + width;

        for i in bottom_right - len..bottom_right {
            output.push(i);
        }
    }

    // corners
    // top left
    if start > width && start % width != 0 {
        output.push(start - width - 1);
    }
    // top right
    if start > width && end % width != 0 {
        output.push(end - width);
    }
    // bottom left
    if end < max - width && start % width != 0 {
        output.push(start + width - 1);
    }
    // bottom right
    if end < max - width && end % width != 0 {
        output.push(end + width);
    }

    output.sort(); // so we can easily compare with the expected

    output
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct CurrentNumber {
    string: String,
    start: usize,
}

#[derive(Debug)]
struct ParsingState {
    current_number: Option<CurrentNumber>,
    numbers: Vec<Number>,
}

fn parse_line(line_number: usize, line: &str) -> Vec<Number> {
    let state = line.chars().zip(line_number..).fold(
        ParsingState {
            current_number: None,
            numbers: Vec::new(),
        },
        |mut state, (c, i)| {
            match state.current_number {
                // If we are currently reading a number, we continue reading
                Some(current_number) => {
                    // if we encounter a digit, we add it to the current number
                    if c.is_ascii_digit() {
                        let mut current_number = current_number;
                        current_number.string.push(c);
                        ParsingState {
                            current_number: Some(current_number),
                            numbers: state.numbers,
                        }
                    // if we encounter a non-digit, we are done reading the current number,
                    // we parse it and add it to the list of numbers
                    } else {
                        let number = current_number.string.parse::<usize>().unwrap();
                        state.numbers.push(Number {
                            value: number,
                            start: current_number.start,
                            end: i,
                        });
                        ParsingState {
                            current_number: None,
                            numbers: state.numbers,
                        }
                    }
                }
                // If we are not currently reading a number, we check if we encounter a digit
                None => {
                    // if we encounter a digit, we start reading a new number
                    if c.is_ascii_digit() {
                        ParsingState {
                            current_number: Some(CurrentNumber {
                                string: c.to_string(),
                                start: i,
                            }),
                            numbers: state.numbers,
                        }
                    // if we encounter a non-digit, we do nothing
                    } else {
                        ParsingState {
                            current_number: None,
                            numbers: state.numbers,
                        }
                    }
                }
            }
        },
    );

    let mut state = state;

    // check the last number we were reading, if we hit the end of the line we should still include this
    if let Some(s) = state.current_number {
        state.numbers.push(Number {
            value: s.string.parse::<usize>().unwrap(),
            start: s.start,
            end: line_number + line.len(),
        });
    }

    state.numbers
}

fn parse(input: &str) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| parse_line(line_number * line.len(), line))
        .collect()
}

fn find_valid_numbers(numbers: Vec<Number>, input: &str) -> Vec<Number> {
    let symbols = symbols(input);
    let chars = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<_>>();

    numbers
        .into_iter()
        .filter(|number| {
            let line_len = input.lines().next().unwrap().len();
            for i in boundary(number.start, number.end, line_len, chars.len()) {
                let Some(&c) = chars.get(i) else {
                    panic!("tried to read an index outside of bound {}", i);
                };
                if symbols.contains(&{ c }) {
                    return true;
                }
            }
            false
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary() {
        assert_eq!(
            boundary(22, 24, 10, 100),
            vec![11, 12, 13, 14, 21, 24, 31, 32, 33, 34]
        );
    }

    #[test]
    fn test_top_right() {
        assert_eq!(boundary(7, 10, 10, 100), vec![6, 16, 17, 18, 19]);
    }

    #[test]
    fn test_top_left() {
        assert_eq!(boundary(0, 2, 10, 100), vec![2, 10, 11, 12]);
    }

    #[test]
    fn test_top_middle() {
        assert_eq!(boundary(3, 6, 10, 100), vec![2, 6, 12, 13, 14, 15, 16]);
    }

    #[test]
    fn test_bottom_right() {
        assert_eq!(boundary(98, 100, 10, 100), vec![87, 88, 89, 97]);
    }

    #[test]
    fn test_symbols() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let expected = HashSet::from(['*', '#', '+', '$']);

        assert_eq!(symbols(input), expected);
    }

    #[test]
    fn test_parse_line() {
        let input = "467..114..";

        let n1 = Number {
            value: 467,
            start: 0,
            end: 3,
        };
        let n2 = Number {
            value: 114,
            start: 5,
            end: 8,
        };

        let expected = vec![n1, n2];

        assert_eq!(parse_line(0, input), expected);
    }

    #[test]
    fn test_parse_line_right_edge() {
        let input = ".......755";

        let n1 = Number {
            value: 755,
            start: 7,
            end: 10,
        };

        let expected = vec![n1];

        assert_eq!(parse_line(0, input), expected);
    }

    #[test]
    fn test_parse() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let expected = vec![
            Number {
                value: 467,
                start: 0,
                end: 3,
            },
            Number {
                value: 114,
                start: 5,
                end: 8,
            },
            Number {
                value: 35,
                start: 22,
                end: 24,
            },
            Number {
                value: 633,
                start: 26,
                end: 29,
            },
            Number {
                value: 617,
                start: 40,
                end: 43,
            },
            Number {
                value: 58,
                start: 57,
                end: 59,
            },
            Number {
                value: 592,
                start: 62,
                end: 65,
            },
            Number {
                value: 755,
                start: 76,
                end: 79,
            },
            Number {
                value: 664,
                start: 91,
                end: 94,
            },
            Number {
                value: 598,
                start: 95,
                end: 98,
            },
        ];

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_find_valid_numbers() {
        let example_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let numbers = parse(example_input);
        let expected = vec![
            Number {
                value: 467,
                start: 0,
                end: 3,
            },
            Number {
                value: 35,
                start: 22,
                end: 24,
            },
            Number {
                value: 633,
                start: 26,
                end: 29,
            },
            Number {
                value: 617,
                start: 40,
                end: 43,
            },
            Number {
                value: 592,
                start: 62,
                end: 65,
            },
            Number {
                value: 755,
                start: 76,
                end: 79,
            },
            Number {
                value: 664,
                start: 91,
                end: 94,
            },
            Number {
                value: 598,
                start: 95,
                end: 98,
            },
        ];

        assert_eq!(find_valid_numbers(numbers, example_input), expected);
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected_output = 4361;

        assert_eq!(part1(example_input), expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected_output = 467835;

        assert_eq!(part2(example_input), expected_output);
        Ok(())
    }
}
