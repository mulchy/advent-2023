use std::{collections::HashMap, str::FromStr};

use advent::io;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = io::for_day(8)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let [directions, map] = parts.as_slice() else {
            bail!("Invalid input {}", s);
        };

        let directions = directions
            .chars()
            .map(|c| match c {
                'R' => Ok(Direction::Right),
                'L' => Ok(Direction::Left),
                _ => bail!("Invalid input {}", s),
            })
            .collect::<Result<Vec<_>>>()?;

        // AAA = (BBB, CCC)
        let map = map
            .lines()
            .map(|l| {
                let parts = l.split(" = ").collect::<Vec<_>>();
                let [key, value] = parts.as_slice() else {
                    bail!("Invalid input {}", s);
                };

                let value = value.to_string().replace(['(', ')'], "");
                let parts = value.split(", ").collect::<Vec<_>>();

                let [left, right] = parts.as_slice() else {
                    bail!("Invalid input {}", s);
                };

                Ok((key.to_string(), (left.to_string(), right.to_string())))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self { directions, map })
    }
}

fn walk(instructions: Instruction) -> usize {
    let mut current = "AAA".to_string();
    let mut count = 0;

    loop {
        let (left, right) = instructions.map.get(&current).unwrap();
        let direction = instructions.directions[count % instructions.directions.len()];

        current = match direction {
            Direction::Right => right.to_string(),
            Direction::Left => left.to_string(),
        };

        count += 1;

        if current == "ZZZ" {
            break;
        }
    }

    count
}

fn ghost_walk(instructions: Instruction) -> usize {
    let mut current = instructions
        .map
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    println!("{:?}", current);
    let mut count = 0;

    loop {
        current = current
            .iter()
            .map(|s| {
                let (left, right) = instructions.map.get(s).unwrap();
                let direction = instructions.directions[count % instructions.directions.len()];

                match direction {
                    Direction::Right => right.to_string(),
                    Direction::Left => left.to_string(),
                }
            })
            .collect::<Vec<_>>();

        count += 1;

        println!("{:?}, {:?}", current, count);
        if current.iter().all(|s| s.ends_with('Z')) {
            break;
        }
    }

    count
}

fn part1(input: &str) -> Result<usize> {
    let instructions = Instruction::from_str(input)?;
    Ok(walk(instructions))
}

fn part2(input: &str) -> Result<usize> {
    let instructions = Instruction::from_str(input)?;
    Ok(ghost_walk(instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_from_str() -> Result<()> {
        let expected = Instruction {
            directions: vec![Direction::Right, Direction::Left],
            map: vec![
                ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
                ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
                ("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string())),
                ("DDD".to_string(), ("DDD".to_string(), "DDD".to_string())),
                ("EEE".to_string(), ("EEE".to_string(), "EEE".to_string())),
                ("GGG".to_string(), ("GGG".to_string(), "GGG".to_string())),
                ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(Instruction::from_str(EXAMPLE_INPUT)?, expected);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(example_input)?, 6);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(example_input)?, 6);
        Ok(())
    }
}
