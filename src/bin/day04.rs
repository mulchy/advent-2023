use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use advent::io;
use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let input = io::for_day(4)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    input
        .lines()
        .map(|l| {
            let card = Card::from_str(l)?;
            Ok(card.score())
        })
        .sum::<Result<i32>>()
}

fn part2(input: &str) -> Result<i32> {
    let cards: Vec<Card> = input.lines().flat_map(Card::from_str).collect();
    let cards_by_id = cards
        .iter()
        .map(|c| (c.id, c.generate_copies()))
        .collect::<HashMap<i32, Vec<i32>>>();

    fn recur(cards_by_id: &HashMap<i32, Vec<i32>>, card_ids: Vec<i32>, count: i32) -> i32 {
        if card_ids.is_empty() {
            return count;
        }

        let originals = card_ids.len() as i32;

        let mut copies: Vec<i32> = card_ids
            .into_iter()
            .flat_map(|id| cards_by_id.get(&id).cloned())
            .flatten()
            .collect();
        copies.sort();

        recur(cards_by_id, copies, count + originals)
    }

    Ok(recur(
        &cards_by_id,
        cards.iter().map(|c| c.id).collect::<Vec<_>>(),
        0,
    ))
}

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    numbers: Vec<i32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self> {
        s = s
            .strip_prefix("Card")
            .ok_or(anyhow!("missing prefix; {}", s))?;
        let parts = s.trim().split(':').collect::<Vec<_>>();
        let [id, rest] = parts.as_slice() else {
            bail!("couldn't split into two parts: {}", s);
        };

        let id = id.parse::<i32>()?;

        let parts = rest.split('|').collect::<Vec<_>>();
        let [winning_numbers, numbers] = parts.as_slice() else {
            bail!("couldn't split into winning numbers and numbers: {}", rest);
        };

        let winning_numbers = winning_numbers
            .split_whitespace()
            .flat_map(|s| s.parse::<i32>())
            .collect::<HashSet<_>>();

        let numbers = numbers
            .split_whitespace()
            .flat_map(|s| s.parse::<i32>())
            .collect::<Vec<_>>();

        Ok(Card {
            id,
            winning_numbers,
            numbers,
        })
    }
}

impl Card {
    fn score(&self) -> i32 {
        let winners = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        if winners == 0 {
            return 0;
        }

        2i32.pow((winners - 1) as u32)
    }

    fn generate_copies(&self) -> Vec<i32> {
        let winners = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        (1..=winners)
            .map(|i| self.id + i as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input)?;
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        Ok(())
    }

    #[test]
    fn test_parse_real_input() -> Result<()> {
        let input = "Card 1: 99 71 95 70 36 79 78 84 31 10 |  5 45 54 83  3 38 89 35 80 49 76 15 63 20 21 94 65 55 44  4 75 56 85 92 90";
        let card = Card::from_str(input)?;
        assert_eq!(card.id, 1);
        assert_eq!(
            card.winning_numbers,
            HashSet::from([99, 70, 36, 79, 84, 31, 71, 95, 10, 78])
        );
        assert_eq!(
            card.numbers,
            vec![
                5, 45, 54, 83, 3, 38, 89, 35, 80, 49, 76, 15, 63, 20, 21, 94, 65, 55, 44, 4, 75,
                56, 85, 92, 90
            ]
        );
        Ok(())
    }

    #[test]
    fn test_score() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input)?;
        assert_eq!(card.score(), 8);
        Ok(())
    }

    #[test]
    fn test_score_no_winners() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 1 2 3 4 5 6 7 8 9";
        let card = Card::from_str(input)?;
        assert_eq!(card.score(), 0);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected_output = 13;

        assert_eq!(part1(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_generate_copies() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input)?;
        assert_eq!(card.generate_copies(), vec![2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected_output = 30;

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }
}
