use std::{fmt::Debug, str::FromStr};

use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(13)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn reflection_about_horizontal<T>(grid: &Grid<T>) -> Option<usize>
where
    T: PartialEq,
{
    for i in 1..grid.width {
        let above = (i..0).map(|i| grid.row(i));
        let below = (i..grid.height).map(|i| grid.row(i));
        if above
            .zip(below)
            .filter(|(a, b)| a.is_some() && b.is_some())
            .all(|(a, b)| a == b)
        {
            return Some(i);
        }
    }
    None
}

fn reflection_about_vertical<T>(grid: &Grid<T>) -> Option<usize>
where
    T: PartialEq + Copy + Debug,
{
    for i in 0..grid.width {
        let left = (0..=i).rev().map(|c| {
            println!("i: {}, left: {}", i, c);
            grid.column(c)
        });
        let right = (i + 1..grid.width).map(|c| {
            println!("i: {}, right: {}", i, c);
            grid.column(c)
        });
        if left
            .zip(right)
            .filter(|(a, b)| {
                println!("{:?} {:?}", a, b);
                a.is_some() && b.is_some()
            })
            .all(|(a, b)| a == b)
        {
            return Some(i);
        }
    }
    None
}

fn part1(_input: &str) -> Result<usize> {
    // Ok(input
    //     .split("\n\n")
    //     .map(|group| {
    //         let grid = Grid::from_str(group)?;

    //     })
    //     .sum()?)

    Ok(42)
}

fn part2(_input: &str) -> Result<String> {
    unimplemented!("You have to solve the puzzle first!")
}

#[derive(Debug, PartialEq, Eq)]
struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl FromStr for Grid<char> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in s.lines() {
            width = line.len();
            height += 1;
            data.extend(line.chars());
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

impl<T> Grid<T> {
    fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.data.get(row * self.width + column)
    }

    fn row(&self, row: usize) -> Option<&[T]> {
        let start = row * self.width;
        let end = start + self.width;
        self.data.get(start..end)
    }

    fn column(&self, column: usize) -> Option<Vec<T>>
    where
        T: Copy,
    {
        let mut result = Vec::new();
        for row in 0..self.height {
            result.push(*self.get(row, column)?);
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() -> Result<()> {
        let example_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let grid = Grid::from_str(example_input)?;

        assert_eq!(
            grid.row(0),
            Some(&['#', '.', '#', '#', '.', '.', '#', '#', '.'][..])
        );

        assert_eq!(
            grid.row(2),
            Some(&['#', '#', '.', '.', '.', '.', '.', '.', '#'][..])
        );

        assert_eq!(grid.row(7), None);

        assert_eq!(
            grid.column(0),
            Some(vec!['#', '.', '#', '#', '.', '.', '#'])
        );

        assert_eq!(
            grid.column(8),
            Some(vec!['.', '.', '#', '#', '.', '.', '.'])
        );

        assert_eq!(grid.column(9), None);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_reflect_about_vertical() -> Result<()> {
        let example_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let expected_output = 5;
        assert_eq!(
            reflection_about_vertical(&Grid::from_str(example_input)?),
            Some(expected_output)
        );

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1() -> Result<()> {
        let example_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let expected_output = 405;

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
