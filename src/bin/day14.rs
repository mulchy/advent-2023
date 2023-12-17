use std::str::FromStr;

use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(14)?;
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

fn tilt_north(grid: &mut Grid<char>) {
    let mut changed = true;
    let mut iters = 0;
    while changed {
        iters += 1;
        if iters > 10 {
            break;
        }

        changed = false;
        for row in 1..grid.height {
            for col in 0..grid.width {
                println!("row: {}, col: {}", row, col);
                println!("current: {:?}", grid.get(row, col));
                println!("above: {:?}", grid.get(row, col));

                if grid.get(row, col) == Some(&'O') && grid.get(row - 1, col) == Some(&'.') {
                    grid.set(row - 1, col, 'O');
                    grid.set(row, col, '.');
                    changed = true
                }
            }
        }
    }
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

    fn set(&mut self, row: usize, column: usize, value: T) {
        if row <= 0 || row >= self.height || column <= 0 || column >= self.width {
            return;
        }

        self.data[row * self.width + column] = value;
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
    #[ignore]
    fn test_tilt_north() -> Result<()> {
        let mut input = Grid::from_str(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )?;
        let expected = Grid::from_str(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
        )?;

        tilt_north(&mut input);
        assert_eq!(input, expected);
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
