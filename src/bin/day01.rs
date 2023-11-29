use advent::io;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = io::for_day(1)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let masses = calculate_masses_for_groups(input)?;

    let max = masses.iter().max().ok_or(anyhow::anyhow!("No max found"))?;

    Ok(*max)
}

fn part2(input: &str) -> Result<i32> {
    let mut masses = calculate_masses_for_groups(input)?;

    masses.sort();
    masses.reverse();

    let top3 = masses.get(0..3).ok_or(anyhow!("Not enough data"))?;

    Ok(top3.iter().sum())
}

fn calculate_masses_for_groups(input: &str) -> Result<Vec<i32>, anyhow::Error> {
    let masses = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|s| Ok(s.parse::<i32>()?))
                .sum::<Result<i32>>()
        })
        .collect::<Result<Vec<i32>>>()?;
    Ok(masses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let expected_output = 24000;

        assert_eq!(part1(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let example_input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let expected_output = 45000;

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }
}
