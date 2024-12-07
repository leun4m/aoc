use itertools::Itertools;

use crate::euclidic::coord::Coord2D;
use crate::euclidic::direction::Direction;
use crate::parser;

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

const SPACE: char = '.';

struct Matrix {
    grid: Vec<Vec<char>>,
    size: isize,
}

impl Matrix {
    fn field(&self, pos: Coord2D) -> char {
        if pos.0 < 0 || pos.0 >= self.size || pos.1 < 0 || pos.1 >= self.size {
            SPACE
        } else {
            self.grid[pos.1 as usize][pos.0 as usize]
        }
    }

    fn field_next(&self, pos: Coord2D, dir: Direction) -> char {
        self.field_next_factor(pos, dir, 1)
    }

    fn field_next_factor(&self, pos: Coord2D, dir: Direction, factor: isize) -> char {
        self.field(pos + factor * dir.coordinates())
    }

    fn surrounded_by(&self, pos: Coord2D, dir: Direction, prev: char, next: char) -> bool {
        (self.field_next(pos, dir) == prev && self.field_next(pos, dir.opposing()) == next)
            || (self.field_next(pos, dir) == next && self.field_next(pos, dir.opposing()) == prev)
    }
}

fn parse(input: &str) -> Matrix {
    let grid = parser::lines_custom(input, |line| line.chars().collect::<Vec<char>>());
    let size = grid.len() as isize;

    Matrix { grid, size }
}

fn part_one(matrix: &Matrix) -> usize {
    (0..matrix.size)
        .cartesian_product(0..matrix.size)
        .map(|(x, y)| Coord2D(x, y))
        .cartesian_product(Direction::ALL)
        .filter(|&(pos, dir)| {
            matrix.field_next_factor(pos, dir, 0) == 'X'
                && matrix.field_next_factor(pos, dir, 1) == 'M'
                && matrix.field_next_factor(pos, dir, 2) == 'A'
                && matrix.field_next_factor(pos, dir, 3) == 'S'
        })
        .count()
}

fn part_two(matrix: &Matrix) -> usize {
    (1..matrix.size - 1)
        .cartesian_product(1..matrix.size - 1)
        .map(|(x, y)| Coord2D(x, y))
        .filter(|&pos| {
            matrix.field(pos) == 'A'
                && matrix.surrounded_by(pos, Direction::NorthWest, 'S', 'M')
                && matrix.surrounded_by(pos, Direction::NorthEast, 'S', 'M')
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(matrix.size, 10);
    }

    #[test]
    fn test_part_one() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(part_one(&matrix), 18);
    }

    #[test]
    fn test_part_two() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(part_two(&matrix), 9);
    }
}
