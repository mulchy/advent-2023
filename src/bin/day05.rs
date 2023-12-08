use std::{collections::HashMap, ops::Range, str::FromStr};

use advent::io;
use anyhow::{anyhow, bail, Error, Result};

fn main() -> Result<()> {
    let input = io::for_day(5)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i64> {
    let input = Input::from_str(input)?;
    input
        .map_seeds()
        .into_iter()
        .min()
        .ok_or(anyhow!("No data"))
}

fn part2(_input: &str) -> Result<String> {
    unimplemented!("You have to solve the puzzle first!")
}

#[derive(Debug, PartialEq)]
struct Mapping {
    domain: Range<i64>,
    offset: i64,
}

impl Mapping {
    fn new(domain: Range<i64>, offset: i64) -> Self {
        Self { domain, offset }
    }

    fn map(&self, value: i64) -> i64 {
        if self.domain.contains(&value) {
            value + self.offset
        } else {
            value
        }
    }
}

struct Map {
    source: String,
    dest: String,
    inner: Vec<Mapping>, // assumed to be disjoint
}

impl Map {
    fn map(&self, value: i64) -> i64 {
        self.inner
            .iter()
            .find(|m| m.domain.contains(&value))
            .map(|m| m.map(value))
            .unwrap_or(value)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let mut header = lines.next().ok_or(anyhow!("No data"))?;
        header = header.strip_suffix("map:").unwrap_or(header);
        header = header.trim();

        let parts = header.split('-').collect::<Vec<_>>();
        let [source, _, dest] = parts.as_slice() else {
            bail!("Malformed header {}", header);
        };

        let mappings = lines
            .map(|line| {
                let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
                let [dest_start, source_start, length] = parts.as_slice() else {
                    bail!("Invalid data {}", line);
                };

                let dest_start = dest_start.parse::<i64>()?;
                let source_start = source_start.parse::<i64>()?;
                let length = length.parse::<i64>()?;

                let offset = dest_start - source_start;
                let domain = source_start..source_start + length;

                Ok(Mapping::new(domain, offset))
            })
            .collect::<Result<Vec<Mapping>>>()?;

        Ok(Map {
            source: source.to_string(),
            dest: dest.to_string(),
            inner: mappings,
        })
    }
}

struct Input {
    start: String,
    seeds: Vec<i64>,
    maps_by_source: HashMap<String, Map>,
}

impl Input {
    fn next_seed(&self, source: &str, seed: i64) -> Option<(String, i64)> {
        self.maps_by_source
            .get(source)
            .map(|m| (m.dest.clone(), m.map(seed)))
    }

    fn map_seeds(&self) -> Vec<i64> {
        self.seeds
            .iter()
            .map(|&seed| {
                let mut current = Some((self.start.clone(), seed));
                let mut last = seed;

                while let Some((source, value)) = current {
                    last = value;
                    current = self.next_seed(&source, value);
                }

                last
            })
            .collect()
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chunks = s.split("\n\n");
        let header = chunks.next().ok_or(anyhow!("No data"))?;

        let parts = header.split(": ").collect::<Vec<_>>();
        let [mut name, rest] = parts.as_slice() else {
            bail!("Invalid header {}", header);
        };
        name = name.strip_suffix('s').unwrap_or(name); // strip plural

        let seeds: Vec<i64> = rest
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<i64>())
            .collect();

        // let seeds = seeds.windows(2).flat_map(|w| {
        //     let mut out = Vec::new();
        //     let start = w[0];
        //     for i in 0..w[1] {
        //         out.push(start + i);
        //     }
        //     out
        // }).collect::<Vec<i64>>();

        // lol i didn't look at the input, those are big numbers
        // we probably need to operate on intervals instead

        // given n piecewise constant functions

        // (-inf, 50) -> (-inf, 50) // identity
        // [50, 98) -> [52, 100)    // +2
        // [98,100) -> [50,52)      // -48
        // [100, inf) -> [100, inf) // identity

        let maps: Vec<Map> = chunks.flat_map(Map::from_str).collect();

        let maps_by_source: HashMap<String, Map> =
            maps.into_iter().map(|m| (m.source.clone(), m)).collect();

        Ok(Input {
            start: name.to_string(),
            seeds,
            maps_by_source,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_map_from_str() -> Result<()> {
        let input = "seed-to-soil map:
50 98 2
52 50 5";

        let map = Map::from_str(input)?;
        assert_eq!(map.source, "seed");
        assert_eq!(map.dest, "soil");

        assert_eq!(
            map.inner,
            vec![Mapping::new(98..100, -48), Mapping::new(50..55, 2)]
        );

        Ok(())
    }

    #[test]
    fn test_input_from_str() -> Result<()> {
        let input = Input::from_str(EXAMPLE_INPUT)?;

        assert_eq!(input.start, "seed");
        assert_eq!(input.seeds, vec![79, 14, 55, 13]);
        assert_eq!(input.maps_by_source.len(), 7);
        let mut keys = input.maps_by_source.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "fertilizer",
                "humidity",
                "light",
                "seed",
                "soil",
                "temperature",
                "water"
            ]
        );

        Ok(())
    }

    #[test]
    fn test_next_seed() -> Result<()> {
        let input = Input::from_str(EXAMPLE_INPUT)?;
        assert_eq!(input.next_seed("seed", 79), Some(("soil".to_owned(), 81)));
        assert_eq!(
            input.next_seed("soil", 81),
            Some(("fertilizer".to_owned(), 81))
        );
        assert_eq!(
            input.next_seed("fertilizer", 81),
            Some(("water".to_owned(), 81))
        );
        assert_eq!(input.next_seed("water", 81), Some(("light".to_owned(), 74)));
        assert_eq!(
            input.next_seed("light", 74),
            Some(("temperature".to_owned(), 78))
        );
        assert_eq!(
            input.next_seed("temperature", 78),
            Some(("humidity".to_owned(), 78))
        );
        assert_eq!(
            input.next_seed("humidity", 78),
            Some(("location".to_owned(), 82))
        );
        assert_eq!(input.next_seed("location", 82), None);
        Ok(())
    }

    #[test]
    fn test_walk() -> Result<()> {
        let input = Input::from_str(EXAMPLE_INPUT)?;
        let output = input.map_seeds();
        assert_eq!(output, vec![82, 43, 86, 35]);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let expected_output = 35;

        assert_eq!(part1(EXAMPLE_INPUT)?, expected_output);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> Result<()> {
        let expected_output = "";

        assert_eq!(part2(EXAMPLE_INPUT)?, expected_output);
        Ok(())
    }
}
