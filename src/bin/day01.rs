#![feature(test)]

use std::collections::HashMap;

use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(1)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            let mut s = String::new();
            s.push(*first);
            s.push(*last);

            s.parse::<i32>().unwrap()
        })
        .sum::<i32>())
}

fn part2(input: &str) -> Result<i32> {
    let sum = input
        .lines()
        .map(|line| {
            let digits = find_digits_2(line);

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let mut s = String::new();
            s.push(*first);
            s.push(*last);

            s.parse::<i32>().unwrap()
        })
        .sum();

    Ok(sum)
}

#[allow(dead_code)]
fn find_digits(line: &str) -> Vec<char> {
    let digit_strings: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);

    let mut digits = vec![];

    let mut i = 0;
    let mut j = i;

    while i <= line.len() {
        let start = j;
        while j < i {
            let word = &line[j..i];
            if let Some(digit) = digit_strings.get(word) {
                digits.push(*digit);
            }
            j += 1;
        }
        i += 1;
        j = start;
    }

    digits
}

fn find_digits_2(s: &str) -> Vec<char> {
    let pairs = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ];

    let mut digits = vec![];

    for i in 0..s.len() {
        let slice = &s[i..];
        for (word, digit) in pairs.iter() {
            if slice.starts_with(word) {
                digits.push(*digit);
            }
        }
    }

    digits
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected_output = 142;

        assert_eq!(part1(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected_output = 281;

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_find_digits() {
        let example_input = "xtwone3four";
        let expected_output = vec!['2', '1', '3', '4'];

        assert_eq!(find_digits_2(example_input), expected_output);
    }

    #[bench]
    fn bench_find_digits(b: &mut Bencher) {
        b.iter(|| find_digits("5xkqkjfjgksflfcqrgrhmfxflscsxsrdhxgpfivetxgpzzlfzj1"));
    }

    #[bench]
    fn bench_find_digits_2(b: &mut Bencher) {
        b.iter(|| find_digits_2("5xkqkjfjgksflfcqrgrhmfxflscsxsrdhxgpfivetxgpzzlfzj1"));
    }
}
