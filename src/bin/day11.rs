use std::collections::HashSet;

use advent::io;
use anyhow::Result;

fn main() -> Result<()> {
    let input = io::for_day(11)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    calculate_distances(input, 1)
}

fn part2(input: &str) -> Result<usize> {
    calculate_distances(input, 1_000_000 - 1)
}

fn calculate_distances(input: &str, expansion_factor: usize) -> Result<usize> {
    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().chars().count();

    let mut row = 0;
    let mut column = 0;

    let mut empty_rows = (0..rows).collect::<HashSet<usize>>();
    let mut empty_columns = (0..columns).collect::<HashSet<usize>>();

    let mut points = Vec::new();

    for c in input.chars() {
        if c == '\n' {
            row += 1;
            column = 0;
        } else {
            if c == '#' {
                empty_rows.remove(&row);
                empty_columns.remove(&column);
                points.push((row, column));
            }
            column += 1;
        }
    }

    expand(&mut points, &empty_rows, &empty_columns, expansion_factor);

    let mut total = 0;

    for i in 0..points.len() {
        let p1 = points[i];
        for j in i + 1..points.len() {
            let p2 = points[j];
            total += manhattan_distance(p1.0 as isize, p1.1 as isize, p2.0 as isize, p2.1 as isize);
        }
    }

    Ok(total)
}

fn expand(
    points: &mut [(usize, usize)],
    empty_rows: &HashSet<usize>,
    empty_columns: &HashSet<usize>,
    expansion_factor: usize,
) {
    for p in points {
        let &mut (row, col) = p;
        p.0 += empty_rows.iter().copied().filter(|&r| r < row).count() * expansion_factor;
        p.1 += empty_columns.iter().copied().filter(|&c| c < col).count() * expansion_factor;
    }
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let example_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part1(example_input)?, 374);
        Ok(())
    }

    #[test]
    fn test_calculate_distances() -> Result<()> {
        let example_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(calculate_distances(example_input, 10 - 1)?, 1030);
        assert_eq!(calculate_distances(example_input, 100 - 1)?, 8410);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> Result<()> {
        let example_input = "";
        let expected_output = 4;

        assert_eq!(part2(example_input)?, expected_output);
        Ok(())
    }
}
