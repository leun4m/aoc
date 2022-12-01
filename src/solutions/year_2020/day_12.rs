use std::cmp::Ordering;

pub fn solve(input: &str) {
    let directions = parse_input(input);
    let (x1, y1) = move_it_one(&directions);
    println!("Part 1: {} ({} / {})", manhattan_distance(x1, y1), x1, y1);
    let (x2, y2) = move_it_two(&directions);
    println!("Part 2: {} ({} / {})", manhattan_distance(x2, y2), x2, y2);
}

const START_DIRECTION: Direction = Direction::East;
const WAYPOINT_EAST: i32 = 10;
const WAYPOINT_NORTH: i32 = 1;

fn move_it_one(instructions: &[Instruction]) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_dir = START_DIRECTION;
    for instruction in instructions {
        let a = if let Instruction::Forward(u) = instruction {
            match current_dir {
                Direction::North => Instruction::North(*u),
                Direction::East => Instruction::East(*u),
                Direction::South => Instruction::South(*u),
                Direction::West => Instruction::West(*u),
            }
        } else {
            *instruction
        };

        match a {
            Instruction::North(u) => {
                x += u;
            }
            Instruction::South(u) => {
                x -= u;
            }
            Instruction::West(u) => {
                y -= u;
            }
            Instruction::East(u) => {
                y += u;
            }
            Instruction::Left(u) => {
                current_dir = turn_around(current_dir, -u);
            }
            Instruction::Right(u) => {
                current_dir = turn_around(current_dir, u);
            }
            Instruction::Forward(_) => {}
        }
    }
    (x, y)
}

fn move_it_two(instructions: &[Instruction]) -> (i32, i32) {
    let mut n = 0;
    let mut e = 0;
    let mut waypoint_north = WAYPOINT_NORTH;
    let mut waypoint_east = WAYPOINT_EAST;

    for instruction in instructions {
        match instruction {
            Instruction::North(u) => {
                waypoint_north += u;
            }
            Instruction::South(u) => {
                waypoint_north -= u;
            }
            Instruction::West(u) => {
                waypoint_east -= u;
            }
            Instruction::East(u) => {
                waypoint_east += u;
            }
            Instruction::Left(u) => {
                let new_waypoint = turn_waypoint((waypoint_north, waypoint_east), -*u);
                waypoint_north = new_waypoint.0;
                waypoint_east = new_waypoint.1;
            }
            Instruction::Right(u) => {
                let new_waypoint = turn_waypoint((waypoint_north, waypoint_east), *u);
                waypoint_north = new_waypoint.0;
                waypoint_east = new_waypoint.1;
            }
            Instruction::Forward(u) => {
                n += waypoint_north * u;
                e += waypoint_east * u;
            }
        }
    }

    (n, e)
}

fn turn_around(direction: Direction, degree: i32) -> Direction {
    assert!(degree % 90 == 0, "UNEXPECTED DEGREE: {}", degree);

    match degree.cmp(&0) {
        Ordering::Less => turn_around(
            match direction {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
            },
            degree + 90,
        ),
        Ordering::Equal => direction,
        Ordering::Greater => turn_around(
            match direction {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            },
            degree - 90,
        ),
    }
}

fn turn_waypoint((north, east): (i32, i32), degree: i32) -> (i32, i32) {
    assert!(degree % 90 == 0, "UNEXPECTED DEGREE: {}", degree);

    match degree.cmp(&0) {
        Ordering::Less => turn_waypoint((east, -north), degree + 90),
        Ordering::Equal => (north, east),
        Ordering::Greater => turn_waypoint((-east, north), degree - 90),
    }
}

fn manhattan_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let units = number.parse::<i32>().unwrap();
        result.push(match direction {
            "N" => Instruction::North(units),
            "E" => Instruction::East(units),
            "S" => Instruction::South(units),
            "W" => Instruction::West(units),
            "L" => Instruction::Left(units),
            "R" => Instruction::Right(units),
            "F" => Instruction::Forward(units),
            _ => panic!("Unexpected direction: {}", direction),
        });
    }
    result
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "F10
N3
F7
R90
F11";

        let instructions = parse_input(input);
        let one = move_it_one(&instructions);
        assert_eq!(25, manhattan_distance(one.0, one.1));

        let two = move_it_two(&instructions);
        assert_eq!(286, manhattan_distance(two.0, two.1));
    }
}
