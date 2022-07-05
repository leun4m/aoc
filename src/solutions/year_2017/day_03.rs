use std::collections::HashMap;

pub fn solve(input: &str) {
    let aim = input.parse().unwrap();
    println!("Part 1: {}", part_one(aim));
    println!("Part 2: {}", part_two(aim));
}

fn part_one(aim: u32) -> i32 {
    let mut point = Point::default();
    let mut direction = Direction::default();
    let mut boundaries = Boundaries::default();

    for _ in 1..aim {
        point = point.move_this(&direction);
        direction = change_dir(&point, &direction, &mut boundaries);
    }

    manhatten_distance(&point)
}

fn part_two(aim: u32) -> u32 {
    let mut point = Point::default();
    let mut direction = Direction::default();
    let mut boundaries = Boundaries::default();

    let mut cache: HashMap<Point, u32> = HashMap::new();
    let mut sum = 1;

    while sum <= aim {
        cache.insert(point, sum);

        point = point.move_this(&direction);
        direction = change_dir(&point, &direction, &mut boundaries);

        sum = sum_neighbours(point, &cache);
    }

    sum
}

fn sum_neighbours(point: Point, cache: &HashMap<Point, u32>) -> u32 {
    [
        point.left(),
        point.right(),
        point.top(),
        point.top().left(),
        point.top().right(),
        point.bottom(),
        point.bottom().left(),
        point.bottom().right(),
    ]
    .iter()
    .map(|x| cache.get(x).unwrap_or(&0))
    .sum()
}

fn change_dir(point: &Point, direction: &Direction, boundaries: &mut Boundaries) -> Direction {
    match *direction {
        Direction::Left => {
            if boundaries.has_reached_left(point) {
                boundaries.set_min(boundaries.min().left());
                Direction::Down
            } else {
                *direction
            }
        }
        Direction::Right => {
            if boundaries.has_reached_right(point) {
                boundaries.set_max(boundaries.max().right());
                Direction::Up
            } else {
                *direction
            }
        }
        Direction::Down => {
            if boundaries.has_reached_bottom(point) {
                boundaries.set_max(boundaries.max().bottom());
                Direction::Right
            } else {
                *direction
            }
        }
        Direction::Up => {
            if boundaries.has_reached_top(point) {
                boundaries.set_min(boundaries.min().top());
                Direction::Left
            } else {
                *direction
            }
        }
    }
}

fn manhatten_distance(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Debug, Default)]
struct Boundaries {
    min: Point,
    max: Point,
}

impl Boundaries {
    fn min(&self) -> &Point {
        &self.min
    }
    fn set_min(&mut self, min: Point) {
        self.min = min;
    }

    fn max(&self) -> &Point {
        &self.max
    }
    fn set_max(&mut self, max: Point) {
        self.max = max;
    }

    fn has_reached_left(&self, point: &Point) -> bool {
        point.x() < self.min().x()
    }
    fn has_reached_right(&self, point: &Point) -> bool {
        point.x() > self.max().x()
    }
    fn has_reached_top(&self, point: &Point) -> bool {
        point.y() < self.min().y()
    }
    fn has_reached_bottom(&self, point: &Point) -> bool {
        point.y() > self.max().y()
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }

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
    fn bottom(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn move_this(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.bottom(),
            Direction::Up => self.top(),
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
