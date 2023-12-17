use std::{collections::VecDeque, str::FromStr};

use advent::io;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = io::for_day(10)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let grid = input.parse::<Grid<char>>()?;
    let (x, y) = grid.find(&'S').unwrap();
    let path = cycle(&grid, x, y).ok_or(anyhow!("No cycle found"))?;
    Ok(path.len() / 2)
}

fn part2(input: &str) -> Result<usize> {
    let grid = input.parse::<Grid<char>>()?;
    let (x, y) = grid.find(&'S').unwrap();
    let path = cycle(&grid, x, y).ok_or(anyhow!("No cycle found"))?;
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if point_in_polygon(&grid, x, y, &path) {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn values() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }

    fn move_from(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (dx, dy) = self.delta();
        let x = (x as i32) + dx;
        let y = (y as i32) + dy;

        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;

        Some((x, y))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn find(&self, value: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .position(|v| v == value)
            .map(|i| (i % self.width, i / self.width))
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y * self.width + x)
    }

    fn von_neumann_neighbors(&self, x: usize, y: usize) -> Vec<(Direction, &T)> {
        Direction::values()
            .iter()
            .flat_map(|dir| {
                let (new_x, new_y) = dir.move_from(x, y)?;
                Some((*dir, self.get(new_x, new_y)?))
            })
            .collect::<Vec<(Direction, &T)>>()
    }
}

impl Grid<char> {
    fn valid_neighbor(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let Some(current) = self.get(x, y) else {
            return false;
        };

        let (dx, dy) = direction.delta();
        let x = (x as i32) + dx;
        let y = (y as i32) + dy;

        let Ok(x) = usize::try_from(x) else {
            return false;
        };

        let Ok(y) = usize::try_from(y) else {
            return false;
        };

        let Some(neighbor) = self.get(x, y) else {
            return false;
        };

        match (current, direction) {
            ('S', Direction::Up) => matches!(neighbor, '|' | 'F' | '7'),
            ('S', Direction::Down) => matches!(neighbor, '|' | 'J' | 'L'),
            ('S', Direction::Left) => matches!(neighbor, '-' | 'F' | 'L'),
            ('S', Direction::Right) => matches!(neighbor, '-' | 'J' | '7'),

            ('|', Direction::Up) => matches!(neighbor, '|' | 'F' | '7' | 'S'),
            ('|', Direction::Down) => matches!(neighbor, '|' | 'J' | 'L' | 'S'),

            ('-', Direction::Left) => matches!(neighbor, '-' | 'F' | 'L' | 'S'),
            ('-', Direction::Right) => matches!(neighbor, '-' | 'J' | '7' | 'S'),

            ('F', Direction::Right) => matches!(neighbor, '-' | 'J' | '7' | 'S'),
            ('F', Direction::Down) => matches!(neighbor, '|' | 'J' | 'L' | 'S'),

            ('7', Direction::Left) => matches!(neighbor, '-' | 'F' | 'L' | 'S'),
            ('7', Direction::Down) => matches!(neighbor, '|' | 'J' | 'L' | 'S'),

            ('J', Direction::Up) => matches!(neighbor, '|' | 'F' | '7' | 'S'),
            ('J', Direction::Left) => matches!(neighbor, '-' | 'F' | 'L' | 'S'),

            ('L', Direction::Up) => matches!(neighbor, '|' | 'F' | '7' | 'S'),
            ('L', Direction::Right) => matches!(neighbor, '-' | 'J' | '7' | 'S'),

            _ => false,
        }
    }

    fn valid_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.von_neumann_neighbors(x, y)
            .iter()
            .filter(|(dir, _)| self.valid_neighbor(x, y, dir))
            .flat_map(|(dir, _)| dir.move_from(x, y))
            .collect()
    }
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

fn cycle(grid: &Grid<char>, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
    let mut count = 0;
    let start = (x, y);

    // only walk in one direction, doesn't matter which
    let mut frontier = VecDeque::from([*grid.valid_neighbors(x, y).first()?]);
    let mut path = Vec::new();
    path.push((x, y));

    while !frontier.is_empty() {
        count += 1;

        let (x, y) = frontier.pop_front().unwrap();
        path.push((x, y));

        for neighbor in grid.valid_neighbors(x, y) {
            // don't go back right away
            if neighbor == start && count > 1 {
                path.push(neighbor);
                return Some(path);
            }

            if path.contains(&neighbor) || frontier.contains(&neighbor) {
                continue;
            }

            frontier.push_back(neighbor)
        }
    }

    None
}

fn point_in_polygon<T>(grid: &Grid<T>, x: usize, y: usize, path: &[(usize, usize)]) -> bool {
    let mut crossings = 0;
    let mut point = Some((x, y));

    if path.contains(&(x, y)) {
        return false; // only want points strictly inside
    }

    while let Some((x, y)) = point {
        if path.contains(&(x, y)) {
            crossings += 1;
        }

        if let Some((new_x, new_y)) = Direction::Up.move_from(x, y) {
            if grid.get(new_x, new_y).is_some() {
                point = Some((new_x, new_y));
                continue;
            }
        }

        point = None;
    }

    crossings % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    const EXAMPLE_INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_parse() -> Result<()> {
        let parsed = EXAMPLE_INPUT.parse::<Grid<char>>()?;
        assert_eq!(
            parsed,
            Grid::<char> {
                width: 5,
                height: 5,
                data: vec![
                    '7', '-', 'F', '7', '-', '.', 'F', 'J', '|', '7', 'S', 'J', 'L', 'L', '7', '|',
                    'F', '-', '-', 'J', 'L', 'J', '.', 'L', 'J',
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_find_get() -> Result<()> {
        let parsed = EXAMPLE_INPUT.parse::<Grid<char>>()?;
        assert_eq!(parsed.find(&'S'), Some((0, 2)));
        assert_eq!(parsed.get(0, 2), Some(&'S'));
        Ok(())
    }

    #[test]
    fn test_von_neumann_neighborhood() -> Result<()> {
        let parsed = EXAMPLE_INPUT.parse::<Grid<char>>()?;
        assert_eq!(
            parsed.von_neumann_neighbors(0, 0),
            vec![(Right, &'-'), (Down, &'.')]
        );
        assert_eq!(
            parsed.von_neumann_neighbors(1, 1),
            vec![(Up, &'-'), (Right, &'J'), (Down, &'J'), (Left, &'.')]
        );
        assert_eq!(
            parsed.von_neumann_neighbors(4, 4),
            vec![(Up, &'J'), (Left, &'L')]
        );
        Ok(())
    }

    #[test]
    fn test_valid_neighbor() -> Result<()> {
        let grid = EXAMPLE_INPUT.parse::<Grid<char>>()?;
        let (x, y) = grid.find(&'S').unwrap();

        assert!(!grid.valid_neighbor(x, y, &Up));
        assert!(grid.valid_neighbor(x, y, &Right));
        assert!(grid.valid_neighbor(x, y, &Down));
        assert!(!grid.valid_neighbor(x, y, &Left));

        let (x, y) = Down.move_from(x, y).unwrap();
        assert!(grid.valid_neighbor(x, y, &Up));

        let (x, y) = grid.find(&'S').unwrap();
        let (x, y) = Right.move_from(x, y).unwrap();
        assert!(grid.valid_neighbor(x, y, &Left));

        Ok(())
    }

    #[test]
    fn test_cycle_length() -> Result<()> {
        let grid = EXAMPLE_INPUT.parse::<Grid<char>>()?;
        let (x, y) = grid.find(&'S').unwrap();

        assert_eq!(
            cycle(&grid, x, y),
            Some(vec![
                (0, 2),
                (1, 2),
                (1, 1),
                (2, 1),
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 4),
                (0, 4),
                (0, 3),
                (0, 2)
            ])
        );

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_point_in_polygon() -> Result<()> {
        let grid = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            .parse::<Grid<char>>()?;
        let (x, y) = grid.find(&'S').unwrap();
        let path = cycle(&grid, x, y).unwrap();

        assert!(point_in_polygon(&grid, 2, 6, &path));
        assert!(point_in_polygon(&grid, 3, 6, &path));
        assert!(point_in_polygon(&grid, 7, 6, &path));
        assert!(point_in_polygon(&grid, 8, 6, &path));
        assert!(!point_in_polygon(&grid, 3, 3, &path));
        assert!(!point_in_polygon(&grid, 1, 8, &path)); // todo handle edges, maybe try winding number?

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let expected_output = 8;

        assert_eq!(part1(EXAMPLE_INPUT)?, expected_output);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part2(input)?, 10);
        Ok(())
    }
}
