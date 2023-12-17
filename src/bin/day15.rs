use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(15)?;
    println!("{}", part1(&input));
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> u64 {
    input.trim().split(',').map(hash).map(|c| c as u64).sum()
}

fn part2(_input: &str) -> Result<String> {
    unimplemented!("You have to solve the puzzle first!")
}

fn hash(input: &str) -> u8 {
    input.bytes().fold(0, |h, c| {
        let h = h as u64;
        let c = c as u64;
        (((h + c) * 17) % 256) as u8
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected_output = 1320;

        assert_eq!(part1(example_input), expected_output);
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
