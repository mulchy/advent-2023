use advent::io;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = io::for_day(9)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i64> {
    let next_values = input
        .lines()
        .map(|l| {
            let sequence = parse(l)?;

            next_value(&sequence).ok_or(anyhow!("Invalid sequence: {:?}", sequence))
        })
        .collect::<Result<Vec<i64>>>()?;

    Ok(next_values.iter().sum())
}

fn part2(input: &str) -> Result<i64> {
    let next_values = input
        .lines()
        .map(|l| {
            let sequence = parse(l)?;
            previous_value(&sequence).ok_or(anyhow!("Invalid sequence: {:?}", sequence))
        })
        .collect::<Result<Vec<i64>>>()?;

    Ok(next_values.iter().sum())
}

fn differences(sequence: &[i64]) -> Vec<i64> {
    let mut differences = Vec::new();
    for i in 1..sequence.len() {
        differences.push(sequence[i] - sequence[i - 1]);
    }
    differences
}

fn next_value(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() {
        return None;
    }

    let mut deltas = sequence.to_vec();
    let mut last_values = vec![];

    while deltas.iter().any(|&d| d != 0) {
        if deltas.len() == 1 && deltas[0] != 0 {
            return None;
        }

        let Some(last) = deltas.last().cloned() else {
            return None;
        };

        deltas = differences(&deltas);
        last_values.push(last);
    }

    Some(last_values.into_iter().sum())
}

fn previous_value(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() {
        return None;
    }

    let mut deltas = sequence.to_vec();
    let mut first_values = vec![];

    while deltas.iter().any(|&d| d != 0) {
        if deltas.len() == 1 && deltas[0] != 0 {
            return None;
        }

        let Some(last) = deltas.first().cloned() else {
            return None;
        };

        deltas = differences(&deltas);

        first_values.push(last);
    }

    first_values.reverse();

    Some(first_values.into_iter().fold(0, |a, b| b - a))
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split_ascii_whitespace()
        .map(|s: &str| Ok(s.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differences() {
        let input = vec![1, 2, 3, 4, 5];
        let expected_output = vec![1, 1, 1, 1];

        assert_eq!(differences(&input), expected_output);
    }

    #[test]
    fn test_next_value() {
        assert_eq!(next_value(&vec![1, 2, 3, 4, 5]), Some(6));
        assert_eq!(next_value(&vec![10, 13, 16, 21, 30, 45]), Some(68));

        // non-polynomial sequence should return None
        assert_eq!(next_value(&[1, 2, 4, 8, 16, 32]), None);

        // empty sequence should return None
        assert_eq!(next_value(&[]), None);
    }

    #[test]
    fn test_previous_value() {
        assert_eq!(previous_value(&vec![1, 2, 3, 4, 5]), Some(0));
        assert_eq!(previous_value(&vec![0, 3, 6, 9, 12, 15]), Some(-3));
        assert_eq!(previous_value(&vec![1, 3, 6, 10, 15, 21]), Some(0));
        assert_eq!(previous_value(&vec![10, 13, 16, 21, 30, 45]), Some(5));
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected_output = 114;

        assert_eq!(part1(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected_output = 2;

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }
}
