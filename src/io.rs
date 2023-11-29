use std::fs::read_to_string;
use std::io::Error;
use std::usize;

pub fn for_day(day: usize) -> Result<String, Error> {
    if day == 0 || day > 25 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid day: {}", day),
        ));
    }

    let path = format!("input/day/{}/input", day);

    read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_day() -> Result<(), Error> {
        let input = for_day(1)?;
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        let last = lines.last().unwrap();

        assert_eq!(first, "4601");
        assert_eq!(last, "6089");

        let input = for_day(25)?;
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        let last = lines.last().unwrap();

        assert_eq!(first, "2=1");
        assert_eq!(last, "2-=2-110012");

        Ok(())
    }

    use quickcheck::Arbitrary;

    #[derive(Debug, Clone, Copy)]
    struct ChristmasDay(usize);

    impl Arbitrary for ChristmasDay {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let day = usize::arbitrary(g) % 25 + 1;
            ChristmasDay(day)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct NonChristmasDay(usize);
    impl Arbitrary for NonChristmasDay {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let day = usize::arbitrary(g) % usize::MAX + 26;
            NonChristmasDay(day)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_for_day_invalid(input: NonChristmasDay) -> bool {
        for_day(input.0).is_err()
    }

    #[quickcheck_macros::quickcheck]
    fn test_for_day_valid(input: ChristmasDay) -> bool {
        for_day(input.0).is_ok()
    }
}
