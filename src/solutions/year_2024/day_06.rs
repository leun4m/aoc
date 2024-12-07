use std::collections::HashSet;

use crate::{
    euclidic::{coord::Coord2D, direction::Direction},
    parser,
};

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

fn parse(input: &str) -> Matrix {
    let grid = parser::lines_custom(input, |line| {
        line.chars()
            .map(|c| match c {
                '#' => Field::Obstacle,
                '^' => Field::Guard,
                _ => Field::Empty,
            })
            .collect::<Vec<Field>>()
    });
    let size = grid.len() as isize;

    Matrix { grid, size }
}

fn part_one(matrix: &Matrix) -> usize {
    find_visited(matrix).len()
}

fn part_two(matrix: &Matrix) -> usize {
    let mut count = 0;
    let original = find_visited(matrix);

    for pos in original {
        if matrix.field(pos) != Field::Guard {
            let mut m = matrix.clone();
            m.set_field(pos, Field::Obstacle);
            if find_circle(&m) {
                count += 1;
            }
        }
    }

    count
}

fn find_visited(matrix: &Matrix) -> HashSet<Coord2D> {
    let mut guard = matrix.find_guard();
    let mut visited = HashSet::new();

    while matrix.in_bounds(guard.position) {
        visited.insert(guard.position);

        guard.take_step(matrix);
    }

    visited
}

fn find_circle(matrix: &Matrix) -> bool {
    let overflow = matrix.size * matrix.size;
    let mut count = 0;
    let mut guard = matrix.find_guard();

    while matrix.in_bounds(guard.position) {
        count += 1;
        if overflow < count {
            return true;
        }

        guard.take_step(matrix);
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Matrix {
    grid: Vec<Vec<Field>>,
    size: isize,
}

impl Matrix {
    fn field(&self, pos: Coord2D) -> Field {
        if self.in_bounds(pos) {
            self.grid[pos.1 as usize][pos.0 as usize]
        } else {
            Field::Empty
        }
    }

    fn set_field(&mut self, pos: Coord2D, field: Field) {
        self.grid[pos.1 as usize][pos.0 as usize] = field
    }

    fn find_guard(&self) -> Guard {
        for x in 0..self.size {
            for y in 0..self.size {
                let position = Coord2D(x, y);
                if self.field(position) == Field::Guard {
                    return Guard {
                        position,
                        direction: Direction::North,
                    };
                }
            }
        }

        panic!("No Guard found!");
    }

    fn in_bounds(&self, pos: Coord2D) -> bool {
        0 <= pos.0 && pos.0 < self.size && 0 <= pos.1 && pos.1 < self.size
    }
}

struct Guard {
    position: Coord2D,
    direction: Direction,
}
impl Guard {
    fn move_to(&mut self, new_pos: Coord2D) {
        self.position = new_pos;
    }

    fn rotate(&mut self) {
        self.direction = self.direction.rotate_clockwise();
    }

    fn take_step(&mut self, matrix: &Matrix) {
        let new_pos = self.position + self.direction.coordinates();
        if matrix.field(new_pos) == Field::Obstacle {
            self.rotate();
        } else {
            self.move_to(new_pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        assert_eq!(41, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(6, part_two(&parse(EXAMPLE_INPUT)));
    }
}
