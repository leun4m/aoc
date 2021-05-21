pub fn main(input: &str) {
    println!("{}", distance(run(input)));
}

pub fn run(input: &str) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::North;
    for line in input.lines() {
        for instruction in line.split(", ") {
             if instruction.chars().count() < 2 {
                 break;
             }
             let first = instruction.chars().nth(0).unwrap();
             let num = instruction[1..].parse().unwrap();
             
             dir = rotate(&dir, first == 'R');
             let pos = move_it(&dir, (x, y), num);
             x = pos.0;
             y = pos.1;
        }
    }
    (x, y)
}

pub fn distance((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
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
    West
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_position() {
        assert_eq!((3, 2), run("R2, L3"));
        assert_eq!((-2, 0), run("R2, R2, R2"));
    }

    #[test]
    pub fn example_distance() {
        assert_eq!(5, distance((2, 3)));
        assert_eq!(2, distance((2, 0)));
        assert_eq!(12, distance(run("R5, L5, R5, R3")));
    }
}
