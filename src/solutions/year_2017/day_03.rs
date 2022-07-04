pub fn solve(input: &str) {
    let aim = input.parse().unwrap();
    println!("Part 1: {}", part_one(aim));
}

fn part_one(start: u32) -> i32 {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut boundaries = Boundaries::default();

    for _ in 1..start {
        next_step(&mut point, &mut direction, &mut boundaries);
    }

    manhatten_distance(point)
}

fn next_step(point: &mut Point, direction: &mut Direction, boundaries: &mut Boundaries) {
    match *direction {
        Direction::Left => {
            if point.x == boundaries.min.x {
                boundaries.min.x -= 1;
                *direction = Direction::Down;
            }
            point.x -= 1;
        }
        Direction::Right => {
            if point.x == boundaries.max.x {
                boundaries.max.x += 1;
                *direction = Direction::Up;
            }
            point.x += 1;
        }
        Direction::Down => {
            if point.y == boundaries.max.y {
                boundaries.max.y += 1;
                *direction = Direction::Right;
            }
            point.y += 1;
        }
        Direction::Up => {
            if point.y == boundaries.min.y {
                boundaries.min.y -= 1;
                *direction = Direction::Left;
            }
            point.y -= 1;
        }
    }
}

fn manhatten_distance(point: Point) -> i32 {
    point.x.abs() + point.y.abs()
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default)]
struct Boundaries {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Default)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(1), 0);
        assert_eq!(part_one(12), 3);
        assert_eq!(part_one(23), 2);
        assert_eq!(part_one(1024), 31);
    }
}
