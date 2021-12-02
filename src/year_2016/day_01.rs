use std::cmp;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let instructions = parse(input);
    println!("{}", distance(run(&instructions)));
    println!("{}", distance(first_double(&instructions)));
}

pub fn run(instructions: &[(i32, i32)]) -> (i32, i32) {
    let mut pos = (0, 0);
    for instruction in instructions {
        pos = (pos.0 + instruction.0, pos.1 + instruction.1);
    }
    pos
}

pub fn first_double(instructions: &[(i32, i32)]) -> (i32, i32) {
    let mut pos = (0, 0);
    let mut positions = HashSet::new();
    for (x, y) in instructions {
        let step_x = x.signum();
        let step_y = y.signum();
        let iterations = cmp::max(x.abs(), y.abs());
        for _ in 0..iterations {
            pos = (pos.0 + step_x, pos.1 + step_y);
            if positions.contains(&pos) {
                return pos;
            }
            positions.insert(pos);
        }
    }
    panic!("No double");
}

pub fn distance((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();
    let mut dir = Direction::North;
    for line in input.lines() {
        for instruction in line.split(", ") {
            if instruction.chars().count() < 2 {
                break;
            }
            let first = instruction.chars().next().unwrap();
            let num = instruction[1..].parse().unwrap();

            dir = rotate(&dir, first == 'R');
            let pos = move_it(&dir, (0, 0), num);

            vec.push(pos);
        }
    }
    vec
}

fn move_it(dir: &Direction, (x, y): (i32, i32), num: i32) -> (i32, i32) {
    match dir {
        Direction::North => (x + num, y),
        Direction::East => (x, y + num),
        Direction::South => (x - num, y),
        Direction::West => (x, y - num),
    }
}

fn rotate(dir: &Direction, right: bool) -> Direction {
    if right {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    } else {
        match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_position() {
        assert_eq!((3, 2), run(&parse("R2, L3")));
        assert_eq!((-2, 0), run(&parse("R2, R2, R2")));
    }

    #[test]
    pub fn example_distance() {
        assert_eq!(5, distance((2, 3)));
        assert_eq!(2, distance((2, 0)));
        assert_eq!(12, distance(run(&parse("R5, L5, R5, R3"))));
    }

    #[test]
    pub fn example_first_double() {
        assert_eq!(4, distance(first_double(&parse("R8, R4, R4, R8"))));
    }
}
