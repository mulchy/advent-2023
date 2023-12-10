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
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

trait Rankable: Ord + Clone + Debug {
    fn rank(&self) -> Rank;
}

fn count_to_rank<T>(cards: &[T]) -> Rank
where
    T: Eq + Hash + Debug,
{
    let counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });

    let mut dupes: Vec<&i32> = counts.values().collect();
    dupes.sort();

    match dupes.as_slice() {
        [1, 1, 1, 1, 1] => Rank::HighCard,
        [1, 1, 1, 2] => Rank::OnePair,
        [1, 2, 2] => Rank::TwoPairs,
        [1, 1, 3] => Rank::ThreeOfAKind,
        [2, 3] => Rank::FullHouse,
        [1, 4] => Rank::FourOfAKind,
        [5] => Rank::FiveOfAKind,
        _ => panic!("Invalid hand: {:?}", cards),
    }
}

impl Rankable for Hand<CardWithoutJoker> {
    fn rank(&self) -> Rank {
        count_to_rank(&self.cards)
    }
}

impl Rankable for Hand<CardWithJoker> {
    fn rank(&self) -> Rank {
        let counts = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        let jokers = *counts.get(&CardWithJoker::Joker).unwrap_or(&0);

        if jokers == 0 {
            return count_to_rank(&self.cards);
        }

        if jokers == 5 {
            return Rank::FiveOfAKind;
        }

        let mut cards_and_counts: Vec<(&&CardWithJoker, &i32)> = counts.iter().collect();
        cards_and_counts.sort_by_key(|(_, count)| -**count);

        let card_to_convert_to = if cards_and_counts[0].0 == &&CardWithJoker::Joker {
            **cards_and_counts[1].0
        } else {
            **cards_and_counts[0].0
        };

        let mut non_jokers: Vec<CardWithJoker> = self
            .cards
            .iter()
            .filter(|&&c| c != CardWithJoker::Joker)
            .cloned()
            .collect();

        for _ in 0..jokers {
            non_jokers.push(card_to_convert_to);
        }

        count_to_rank(&non_jokers)
    }
}

#[derive(Debug, Clone)]
struct Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
{
    cards: Vec<T>,
}

impl<T> Ord for Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
    Self: Rankable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank().cmp(&other.rank()) {
            Less => Less,
            Greater => Greater,
            Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
    Self: Rankable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
    Self: Rankable,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl<T> Eq for Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
    Self: Rankable,
{
}

impl<T> FromStr for Hand<T>
where
    T: Ord + Copy + Debug + FromStr,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cards = s
            .chars()
            .map(|c| {
                c.to_string().as_str().parse::<T>().map_err(|_e| {
                    anyhow::anyhow!("idk enough about rust to coerce this error to the right type")
                })
            })
            .collect::<Result<Vec<T>>>()?;

        Ok(Self { cards })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct HandAndBid<T>
where
    T: Ord + Copy + Debug + FromStr,
    Hand<T>: Rankable,
{
    hand: Hand<T>,
    bid: i32,
}

impl<T> FromStr for HandAndBid<T>
where
    T: FromStr + Ord + Copy + Debug,
    Hand<T>: Rankable,
{
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

fn do_it<T>(input: &str) -> Result<i32>
where
    T: FromStr + Ord + Copy + Debug,
    Hand<T>: Rankable,
{
    let mut hands_and_bids = input
        .lines()
        .map(HandAndBid::from_str)
        .collect::<Result<Vec<HandAndBid<T>>>>()?;
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
            assert_eq!(hand.rank(), expected_rank);
        }

        Ok(())
    }

    #[test]
    fn test_hand_and_bid_from_str() -> Result<()> {
        let input = "32T3K 765";

        assert_eq!(
            input.parse::<HandAndBid<CardWithoutJoker>>()?,
            HandAndBid {
                hand: Hand {
                    cards: vec![
                        CardWithoutJoker::Three,
                        CardWithoutJoker::Two,
                        CardWithoutJoker::Ten,
                        CardWithoutJoker::Three,
                        CardWithoutJoker::King,
                    ],
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
    fn test_rank_jokers() -> Result<()> {
        assert_eq!(
            Hand::<CardWithJoker>::from_str("JTTJJ")?.rank(),
            FiveOfAKind
        );
        assert_eq!(
            Hand::<CardWithJoker>::from_str("T55J5")?.rank(),
            FourOfAKind
        );
        assert_eq!(
            Hand::<CardWithJoker>::from_str("KTJJT")?.rank(),
            FourOfAKind
        );
        assert_eq!(
            Hand::<CardWithJoker>::from_str("QQQJA")?.rank(),
            FourOfAKind
        );
        assert_eq!(
            Hand::<CardWithJoker>::from_str("JJJJJ")?.rank(),
            FiveOfAKind
        );

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 6440);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 5905);
        Ok(())
    }
}
