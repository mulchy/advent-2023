use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use advent::io;
use anyhow::{anyhow, bail, Error, Result};

fn main() -> Result<()> {
    let input = io::for_day(16)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    count_energized(
        &grid,
        Beam {
            position: (0, 0),
            direction: Direction::Right,
        },
    )
}

fn part2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    (0..grid.height as isize)
        .map(|row| Beam {
            position: (row, 0),
            direction: Direction::Right,
        })
        .chain((0..grid.width as isize).map(|column| Beam {
            position: (0, column),
            direction: Direction::Down,
        }))
        .chain((0..grid.height as isize).map(|row| Beam {
            position: (row, grid.width as isize - 1),
            direction: Direction::Left,
        }))
        .chain((0..grid.width as isize).map(|column| Beam {
            position: (grid.height as isize - 1, column),
            direction: Direction::Up,
        }))
        .flat_map(|beam| count_energized(&grid, beam))
        .max()
        .ok_or(anyhow!("No data"))
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    LeftMirror,
    RightMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Tile::Empty),
            '\\' => Ok(Tile::LeftMirror),
            '/' => Ok(Tile::RightMirror),
            '|' => Ok(Tile::VerticalSplitter),
            '-' => Ok(Tile::HorizontalSplitter),
            _ => bail!("Unknown character: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Beam {
    position: (isize, isize),
    direction: Direction,
}

impl Beam {
    fn next(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0, self.position.1 + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Tile>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in s.lines() {
            width = line.len();
            height += 1;
            data.extend(
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>>>()?,
            );
        }

        Ok(Grid {
            width,
            height,
            data,
        })
    }
}

impl Grid {
    fn get(&self, row: isize, column: isize) -> Option<&Tile> {
        if row < 0 || column < 0 {
            return None;
        }

        let row = row as usize;
        let column = column as usize;

        self.data.get(row * self.width + column)
    }
}

// keep track of each grid point that has been hit by a beam going a particular direction
// if a beam hits a point that has already been hit, then we have a cycle
// if a beam hits a point that is outside the bounds of the grid, then we have hit the edge of the map
fn count_energized(grid: &Grid, starting_beam: Beam) -> Result<usize> {
    let mut history: HashMap<(isize, isize), Vec<Direction>> = HashMap::new();

    let mut beams = VecDeque::from([starting_beam]);

    while let Some(mut beam) = beams.pop_front() {
        // println!(
        //     "Processing beam starting at {:?}, facing {:?}",
        //     beam.position, beam.direction
        // );

        loop {
            // make sure we haven't gone out of bounds
            if beam.position.0 < 0
                || beam.position.1 < 0
                || beam.position.0 >= grid.width as isize
                || beam.position.1 >= grid.height as isize
            {
                // println!("Beam went out of bounds at: {:?}", beam.position);
                break;
            }

            // check if we've already been here
            if let Some(directions) = history.get(&beam.position) {
                if directions.contains(&beam.direction) {
                    // println!("Found cycle at: {:?}", beam.position);
                    break;
                }
            }

            // record where we currently are
            history
                .entry(beam.position)
                .or_default()
                .push(beam.direction);

            let tile = grid
                .get(beam.position.0, beam.position.1)
                .ok_or(anyhow!("Beam went out of bounds"))?;

            // println!("This beam is on a {:?} tile", tile);

            match tile {
                Tile::Empty => (),
                Tile::LeftMirror => match beam.direction {
                    Direction::Up => beam.direction = Direction::Left,
                    Direction::Down => beam.direction = Direction::Right,
                    Direction::Left => beam.direction = Direction::Up,
                    Direction::Right => beam.direction = Direction::Down,
                },
                Tile::RightMirror => match beam.direction {
                    Direction::Up => beam.direction = Direction::Right,
                    Direction::Down => beam.direction = Direction::Left,
                    Direction::Left => beam.direction = Direction::Down,
                    Direction::Right => beam.direction = Direction::Up,
                },

                Tile::VerticalSplitter => {
                    beam.direction = Direction::Up;
                    let mut split = Beam {
                        position: beam.position,
                        direction: Direction::Down,
                    };
                    split.position = split.next();
                    beams.push_back(split);
                }
                Tile::HorizontalSplitter => {
                    beam.direction = Direction::Right;
                    let mut split = Beam {
                        position: beam.position,
                        direction: Direction::Left,
                    };
                    split.position = split.next();
                    beams.push_back(split);
                }
            }

            beam.position = beam.next();

            // println!(
            //     "Beam moved to {:?} and is facing {:?}",
            //     beam.position, beam.direction
            // );
        }
    }

    Ok(history.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 46);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 51);
        Ok(())
    }
}
