use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

use advent::io;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = io::for_day(7)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

trait Card: PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Hash + Debug + FromStr {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum CardWithoutJoker {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card for CardWithoutJoker {}

impl FromStr for CardWithJoker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Joker),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(anyhow::anyhow!("Invalid card: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum CardWithJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card for CardWithJoker {}

impl FromStr for CardWithoutJoker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(anyhow::anyhow!("Invalid card: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand<T: Card> {
    cards: Vec<T>,
    rank: Rank,
}

impl<T: Card> Hand<T> {}

impl<T: Card> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            Less => Less,
            Greater => Greater,
            Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl<T: Card> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Card> PartialEq for Hand<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl<T: Card> Eq for Hand<T> {}

// todo specialize for joker
fn rank_hand<T: Card>(cards: &[T]) -> Result<Rank> {
    let counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });

    let mut dupes: Vec<&i32> = counts.values().collect();
    dupes.sort();

    match dupes.as_slice() {
        [1, 1, 1, 1, 1] => Ok(Rank::HighCard),
        [1, 1, 1, 2] => Ok(Rank::OnePair),
        [1, 2, 2] => Ok(Rank::TwoPairs),
        [1, 1, 3] => Ok(Rank::ThreeOfAKind),
        [2, 3] => Ok(Rank::FullHouse),
        [1, 4] => Ok(Rank::FourOfAKind),
        [5] => Ok(Rank::FiveOfAKind),
        _ => Err(anyhow!("Invalid hand: {:?}", cards)),
    }
}

impl<T: Card> FromStr for Hand<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cards: Vec<T> = s
            .chars()
            .map(|c| {
                c.to_string().as_str().parse::<T>().map_err(|_e| {
                    anyhow::anyhow!("idk enough about rust to coerce this error to the right type")
                })
            })
            .collect::<Result<Vec<T>>>()?;

        let rank = rank_hand(&cards)?;

        Ok(Self { cards, rank })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct HandAndBid<T: Card> {
    hand: Hand<T>,
    bid: i32,
}

impl<T: Card> FromStr for HandAndBid<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_ascii_whitespace();
        let hand = parts.next().ok_or(anyhow!("No data"))?;
        let bid = parts.next().ok_or(anyhow!("No data"))?.parse()?;

        Ok(Self {
            hand: hand.parse()?,
            bid,
        })
    }
}

fn do_it<T: Card>(input: &str) -> Result<i32> {
    let mut hands_and_bids = input
        .lines()
        .map(HandAndBid::<T>::from_str)
        .collect::<Result<Vec<_>>>()?;
    hands_and_bids.sort();

    Ok(hands_and_bids
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum())
}

fn part1(input: &str) -> Result<i32> {
    do_it::<CardWithoutJoker>(input)
}

fn part2(input: &str) -> Result<i32> {
    do_it::<CardWithJoker>(input)
}

#[cfg(test)]
mod tests {
    use super::Rank::*;
    use super::*;

    #[test]
    fn test_hand_from_str() -> Result<()> {
        let input = vec![
            ("32T3K", OnePair),
            ("T55J5", ThreeOfAKind),
            ("KK677", TwoPairs),
            ("KTJJT", TwoPairs),
            ("QQQJA", ThreeOfAKind),
        ];

        for (hand, expected_rank) in input {
            let hand = hand.parse::<Hand<CardWithoutJoker>>()?;
            assert_eq!(hand.rank, expected_rank);
        }

        Ok(())
    }

    #[test]
    fn test_hand_and_bid_from_str() -> Result<()> {
        let input = "32T3K 765";

        assert_eq!(
            input.parse::<HandAndBid<CardWithoutJoker>>()?,
            HandAndBid::<CardWithoutJoker> {
                hand: Hand {
                    cards: vec![
                        CardWithoutJoker::Three,
                        CardWithoutJoker::Two,
                        CardWithoutJoker::Ten,
                        CardWithoutJoker::Three,
                        CardWithoutJoker::King
                    ],
                    rank: OnePair,
                },
                bid: 765,
            }
        );
        Ok(())
    }

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 6440);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 5905);
        Ok(())
    }
}
