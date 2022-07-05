use std::collections::HashMap;

pub fn solve(input: &str) {
    let aim = input.parse().unwrap();
    println!("Part 1: {}", part_one(aim));
    println!("Part 2: {}", part_two(aim));
}

fn part_one(aim: u32) -> i32 {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut boundaries = Boundaries::default();

    for _ in 1..aim {
        point = next_step(&point, &mut direction, &mut boundaries);
    }

    manhatten_distance(point)
}

fn part_two(aim: u32) -> u32 {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut boundaries = Boundaries::default();

    let mut cache: HashMap<Point, u32> = HashMap::new();
    let mut sum = 1;

    while sum <= aim {
        cache.insert(point, sum);
        point = next_step(&point, &mut direction, &mut boundaries);
        sum = sum_neighbours(point, &cache);
    }

    sum
}

fn sum_neighbours(point: Point, cache: &HashMap<Point, u32>) -> u32 {
    cache.get(&point.left()).unwrap_or(&0)
        + cache.get(&point.top_left()).unwrap_or(&0)
        + cache.get(&point.top()).unwrap_or(&0)
        + cache.get(&point.top_right()).unwrap_or(&0)
        + cache.get(&point.right()).unwrap_or(&0)
        + cache.get(&point.bottom_right()).unwrap_or(&0)
        + cache.get(&point.bottom()).unwrap_or(&0)
        + cache.get(&point.bottom_left()).unwrap_or(&0)
}

fn next_step(point: &Point, direction: &mut Direction, boundaries: &mut Boundaries) -> Point {
    match *direction {
        Direction::Left => {
            if point.x == boundaries.min.x {
                boundaries.min = boundaries.min.left();
                *direction = Direction::Down;
            }
            point.left()
        }
        Direction::Right => {
            if point.x == boundaries.max.x {
                boundaries.max = boundaries.max.right();
                *direction = Direction::Up;
            }
            point.right()
        }
        Direction::Down => {
            if point.y == boundaries.max.y {
                boundaries.max = boundaries.max.bottom();
                *direction = Direction::Right;
            }
            point.bottom()
        }
        Direction::Up => {
            if point.y == boundaries.min.y {
                boundaries.min = boundaries.min.top();
                *direction = Direction::Left;
            }
            point.top()
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

#[derive(Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn top_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    fn top_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    fn bottom(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn bottom_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn bottom_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
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

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(1), 2);
        assert_eq!(part_two(3), 4);
        assert_eq!(part_two(12), 23);
        assert_eq!(part_two(23), 25);
        assert_eq!(part_two(747), 806);
    }
}
