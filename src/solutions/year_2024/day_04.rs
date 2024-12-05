use std::ops::{Add, Mul, Neg};

use crate::parser;

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

const SPACE: char = '.';

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position(isize, isize);

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position(-self.0, -self.1)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<Position> for isize {
    type Output = Position;

    fn mul(self, rhs: Position) -> Self::Output {
        Position(self * rhs.0, self * rhs.1)
    }
}

struct Matrix {
    grid: Vec<Vec<char>>,
    size: isize,
}

impl Matrix {
    fn field(&self, pos: Position) -> char {
        if pos.0 < 0 || pos.0 >= self.size || pos.1 < 0 || pos.1 >= self.size {
            SPACE
        } else {
            self.grid[pos.1 as usize][pos.0 as usize]
        }
    }

    fn field_next(&self, pos: Position, dir: Direction) -> char {
        self.field_next_factor(pos, dir, 1)
    }

    fn field_next_factor(&self, pos: Position, dir: Direction, factor: isize) -> char {
        self.field(pos + factor * dir.coordinates())
    }

    fn surrounded_by(&self, pos: Position, dir: Direction, prev: char, next: char) -> bool {
        (self.field_next(pos, dir) == prev && self.field_next(pos, dir.opposing()) == next)
            || (self.field_next(pos, dir) == next && self.field_next(pos, dir.opposing()) == prev)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthWest,
        Direction::SouthEast,
    ];

    const NORTH: Position = Position(0, 1);
    const WEST: Position = Position(1, 0);

    fn coordinates(self) -> Position {
        match self {
            Direction::North => Direction::NORTH,
            Direction::South => -Direction::NORTH,
            Direction::West => Direction::WEST,
            Direction::East => -Direction::WEST,
            Direction::NorthWest => Direction::NORTH + Direction::WEST,
            Direction::NorthEast => Direction::NORTH + (-Direction::WEST),
            Direction::SouthWest => -Direction::NORTH + Direction::WEST,
            Direction::SouthEast => -Direction::NORTH + (-Direction::WEST),
        }
    }

    fn opposing(self) -> Direction {
        *Direction::ALL
            .iter()
            .find(|dir| -dir.coordinates() == self.coordinates())
            .unwrap()
    }
}

fn parse(input: &str) -> Matrix {
    let grid = parser::lines_custom(input, |line| line.chars().collect::<Vec<char>>());
    let size = grid.len() as isize;

    Matrix { grid, size }
}

fn part_one(matrix: &Matrix) -> usize {
    let mut count = 0;

    for x in 0..matrix.size {
        for y in 0..matrix.size {
            for dir in Direction::ALL {
                let pos = Position(x, y);

                if matrix.field_next_factor(pos, dir, 0) == 'X'
                    && matrix.field_next_factor(pos, dir, 1) == 'M'
                    && matrix.field_next_factor(pos, dir, 2) == 'A'
                    && matrix.field_next_factor(pos, dir, 3) == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_two(matrix: &Matrix) -> usize {
    let mut count = 0;

    for x in 1..matrix.size - 1 {
        for y in 1..matrix.size - 1 {
            let pos = Position(x, y);

            if matrix.field(pos) == 'A'
                && matrix.surrounded_by(pos, Direction::NorthWest, 'S', 'M')
                && matrix.surrounded_by(pos, Direction::NorthEast, 'S', 'M')
            {
                count += 1;
            }
        }
    }

    count
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
