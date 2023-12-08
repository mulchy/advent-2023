use advent::io;
use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    let input = io::for_day(6)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    Ok(Input::from_str(input)?.count())
}

fn part2(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let mut time = lines
        .next()
        .ok_or(anyhow!("No data"))?
        .strip_prefix("Time:")
        .ok_or(anyhow!("No data"))?
        .to_string();
    let mut distance = lines
        .next()
        .ok_or(anyhow!("No data"))?
        .strip_prefix("Distance:")
        .ok_or(anyhow!("No data"))?
        .to_string();

    time.retain(|c| !c.is_ascii_whitespace());
    distance.retain(|c| !c.is_ascii_whitespace());

    let record = Record {
        time: time.parse()?,
        distance: distance.parse()?,
    };

    Ok(record.number_of_ways_to_beat_the_record())
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Record {
    time: i64,
    distance: i64,
}

#[derive(Debug, PartialEq)]
struct Input {
    records: Vec<Record>,
}

impl Input {
    fn count(self) -> usize {
        self.records
            .iter()
            .map(|r| r.number_of_ways_to_beat_the_record())
            .product()
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .ok_or(anyhow!("No data"))?
            .strip_prefix("Time:")
            .ok_or(anyhow!("No data"))?;
        let distances = lines
            .next()
            .ok_or(anyhow!("No data"))?
            .strip_prefix("Distance:")
            .ok_or(anyhow!("No data"))?;

        let records: Result<Vec<Record>> = times
            .split_whitespace()
            .zip(distances.split_whitespace())
            .map(|(t, d)| {
                Ok(Record {
                    time: t.parse()?,
                    distance: d.parse()?,
                })
            })
            .collect();

        Ok(Self { records: records? })
    }
}

impl Record {
    fn possible_distances(self) -> impl Iterator<Item = i64> {
        (0..=self.time).map(move |n| n * (self.time - n))
    }

    fn number_of_ways_to_beat_the_record(self) -> usize {
        self.possible_distances()
            .filter(|n| n > &self.distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_map_from_str() -> Result<()> {
        let input = Input::from_str(EXAMPLE_INPUT)?;
        let expected = Input {
            records: vec![
                Record {
                    time: 7,
                    distance: 9,
                },
                Record {
                    time: 15,
                    distance: 40,
                },
                Record {
                    time: 30,
                    distance: 200,
                },
            ],
        };

        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_possible_distances() -> Result<()> {
        let input = Record {
            time: 7,
            distance: 9,
        };

        assert_eq!(
            input.possible_distances().collect::<Vec<i64>>(),
            vec![0, 6, 10, 12, 12, 10, 6, 0]
        );

        Ok(())
    }

    #[test]
    fn test_number_of_ways_to_beat_the_record() -> Result<()> {
        let input = Record {
            time: 7,
            distance: 9,
        };

        assert_eq!(input.number_of_ways_to_beat_the_record(), 4);

        Ok(())
    }

    #[test]
    fn test_count() -> Result<()> {
        let input = Input::from_str(EXAMPLE_INPUT)?;
        assert_eq!(input.count(), 288);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let expected_output = 288;

        assert_eq!(part1(EXAMPLE_INPUT)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 71503);
        Ok(())
    }
}
