use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(20)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(_input: &str) -> Result<String> {
    unimplemented!("You have to solve the puzzle first!")
}

fn part2(_input: &str) -> Result<String> {
    unimplemented!("You have to solve the puzzle first!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> Result<()> {
        let example_input = "";
        let expected_output = "";

        assert_eq!(part1(example_input)?, expected_output);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> Result<()> {
        let example_input = "";
        let expected_output = "";

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }
}
