use std::collections::HashSet;

use crate::{
    euclidic::{coord::Coord2D, direction::Direction},
    parser,
};

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
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
    let mut guard = matrix.find_guard();
    let mut visited = HashSet::new();

    while matrix.in_bounds(guard.position) {
        visited.insert(guard.position);
        let new_pos = guard.position + guard.direction.coordinates();

        if matrix.field(new_pos) == Field::Obstacle {
            guard.rotate();
        } else {
            guard.move_to(new_pos);
        }
    }

    visited.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Obstacle,
    Guard,
}

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
}
