use crate::parser;

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

struct Matrix {
    grid: Vec<Vec<char>>,
    size: isize,
}

const SPACE: char = '.';

impl Matrix {
    fn field(&self, x: isize, y: isize) -> char {
        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            SPACE
        } else {
            self.grid[y as usize][x as usize]
        }
    }

    fn field_next(&self, x: isize, y: isize, dir: &Direction) -> char {
        self.field_next_factor(x, y, dir, 1)
    }

    fn field_next_factor(&self, x: isize, y: isize, dir: &Direction, factor: isize) -> char {
        let (dx, dy) = dir.coordinates();
        self.field(x + dx * factor, y + dy * factor)
    }

    fn surrounded_by(&self, x: isize, y: isize, dir: Direction, prev: char, next: char) -> bool {
        (self.field_next(x, y, &dir) == prev && self.field_next(x, y, &dir.opposing()) == next)
            || (self.field_next(x, y, &dir) == next
                && self.field_next(x, y, &dir.opposing()) == prev)
    }
}

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
    fn coordinates(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (1, -1),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (1, 1),
        }
    }

    fn all() -> [Direction; 8] {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::SouthWest,
            Direction::SouthEast,
        ]
    }

    fn opposing(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::NorthWest => Direction::SouthEast,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::SouthEast => Direction::NorthEast,
        }
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
            for dir in Direction::all() {
                if matrix.field_next_factor(x, y, &dir, 0) == 'X'
                    && matrix.field_next_factor(x, y, &dir, 1) == 'M'
                    && matrix.field_next_factor(x, y, &dir, 2) == 'A'
                    && matrix.field_next_factor(x, y, &dir, 3) == 'S'
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
            if matrix.field(x, y) == 'A'
                && matrix.surrounded_by(x, y, Direction::NorthWest, 'S', 'M')
                && matrix.surrounded_by(x, y, Direction::NorthEast, 'S', 'M')
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
