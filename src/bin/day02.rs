use advent::io;
use anyhow::{anyhow, Result};
use regex::Regex;

fn main() -> Result<()> {
    let input = io::for_day(2)?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn count(re: Regex, input: &str) -> usize {
    let Some(c) = re.captures(input) else {
        return 0;
    };

    let Some(s) = c.get(1) else {
        return 0;
    };

    s.as_str().parse::<usize>().unwrap_or(0)
}

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<Counts>,
}

#[derive(Debug, Clone, Copy)]
struct Counts {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(input: &str) -> Result<Game> {
    let id = Regex::new(r"Game (\d+):")?
        .captures(input)
        .ok_or(anyhow!("Could not find game id"))?
        .get(1)
        .ok_or(anyhow!("no first capture group"))?
        .as_str()
        .parse::<usize>()?;

    let trimmed = Regex::new(r"Game (\d+):")?.replace(input, "");
    let reveals = trimmed.split(';').collect::<Vec<_>>();

    let parsed_reveals = reveals
        .iter()
        .map(|s| {
            let trimmed = s.trim();

            let red = count(Regex::new(r"(\d+) red")?, trimmed);
            let green = count(Regex::new(r"(\d+) green")?, trimmed);
            let blue = count(Regex::new(r"(\d+) blue")?, trimmed);

            Ok(Counts { red, green, blue })
        })
        .collect::<Result<Vec<Counts>>>()?;

    Ok(Game {
        id,
        reveals: parsed_reveals,
    })
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(parse)
        .filter(|Game { reveals, .. }| {
            reveals
                .iter()
                .all(|&Counts { red, green, blue }| red <= 12 && green <= 13 && blue <= 14)
        })
        .map(|Game { id, .. }| id)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(parse)
        .map(|Game { reveals, .. }| {
            let max_red = reveals
                .iter()
                .map(|&Counts { red, .. }| red)
                .max()
                .unwrap_or(0);
            let max_green = reveals
                .iter()
                .map(|&Counts { green, .. }| green)
                .max()
                .unwrap_or(0);
            let max_blue = reveals
                .iter()
                .map(|&Counts { blue, .. }| blue)
                .max()
                .unwrap_or(0);

            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let Game { id, reveals } = parse(example_input)?;

        assert_eq!(id, 1);

        let &[reveal1, reveal2, reveal3] = reveals.as_slice() else {
            panic!("Expected 3 reveals, got {}", reveals.len());
        };

        assert_eq!(reveal1.red, 4);
        assert_eq!(reveal1.green, 0);
        assert_eq!(reveal1.blue, 3);

        assert_eq!(reveal2.red, 1);
        assert_eq!(reveal2.green, 2);
        assert_eq!(reveal2.blue, 6);

        assert_eq!(reveal3.red, 0);
        assert_eq!(reveal3.green, 2);
        assert_eq!(reveal3.blue, 0);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_output = 8;

        assert_eq!(part1(example_input), expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_output = 2286;

        assert_eq!(part2(example_input), expected_output);
        Ok(())
    }
}
