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
        point = point.move_in(direction);
        direction = boundaries.change_dir(point, direction);
    }

    manhatten_distance(point)
}

fn part_two(aim: u32) -> u32 {
    let mut point = Point::default();
    let mut direction = Direction::default();
    let mut boundaries = Boundaries::default();

    let mut cache: HashMap<Point, u32> = HashMap::new();
    let mut sum = 1;

    while sum <= aim {
        cache.insert(point, sum);

        point = point.move_in(direction);
        direction = boundaries.change_dir(point, direction);

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

fn manhatten_distance(point: Point) -> i32 {
    point.x.abs() + point.y.abs()
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn has_reached_left(&self, point: Point) -> bool {
        point.x() < self.min.x()
    }
    fn has_reached_right(&self, point: Point) -> bool {
        point.x() > self.max.x()
    }
    fn has_reached_top(&self, point: Point) -> bool {
        point.y() < self.min.y()
    }
    fn has_reached_bottom(&self, point: Point) -> bool {
        point.y() > self.max.y()
    }

    fn extend_left(&mut self) {
        self.min = self.min.left();
    }
    fn extend_right(&mut self) {
        self.max = self.max.right();
    }
    fn extend_bottom(&mut self) {
        self.max = self.max.bottom();
    }
    fn extend_top(&mut self) {
        self.min = self.min.top();
    }

    fn change_dir(&mut self, point: Point, direction: Direction) -> Direction {
        if direction == Direction::Left && self.has_reached_left(point) {
            self.extend_left();
            Direction::Down
        } else if direction == Direction::Right && self.has_reached_right(point) {
            self.extend_right();
            Direction::Up
        } else if direction == Direction::Down && self.has_reached_bottom(point) {
            self.extend_bottom();
            Direction::Right
        } else if direction == Direction::Up && self.has_reached_top(point) {
            self.extend_top();
            Direction::Left
        } else {
            direction
        }
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn x(self) -> i32 {
        self.x
    }
    fn y(self) -> i32 {
        self.y
    }

    fn left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn top(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn bottom(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn move_in(self, direction: Direction) -> Point {
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
