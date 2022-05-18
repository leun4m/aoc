use regex::Regex;
use std::cmp::max;
use std::cmp::Ordering;

pub fn solve(input: &str) {
    let target = parse(input);
    let (highest_y, hitting_vectors) = find_best(&target);
    println!("Part 1: {}", highest_y);
    println!("Part 2: {}", hitting_vectors);
}

const TARGET_AREA_KEY: &str = "target area: ";
const GUESSED_Y_FACTOR_RANGE: i32 = 100;

fn parse(input: &str) -> TargetArea {
    input
        .lines()
        .filter(|line| line.trim().starts_with(TARGET_AREA_KEY))
        .map(parse_target)
        .next()
        .unwrap()
}

fn find_best(target: &TargetArea) -> (i32, usize) {
    let mut y_best = -1;
    let mut count = 0;
    // Yeah, uses bruteforce and some random GUESSED_Y_FACTOR_RANGE ...
    let y_min = GUESSED_Y_FACTOR_RANGE * target.y_max;
    let y_max = GUESSED_Y_FACTOR_RANGE * target.y_max.abs();

    for y in y_min..y_max {
        for x in 1..=target.x_max {
            if let Outcome::Hit(y_reached) = check_hit_target((x, y), target) {
                y_best = max(y_best, y_reached);
                count += 1;
            }
        }
    }

    (y_best, count)
}

fn parse_target(line: &str) -> TargetArea {
    let regex = Regex::new(r"x=([-]?\d+)..([-]?\d+), y=([-]?\d+)..([-]?\d+)").unwrap();
    let captures = regex.captures(line).unwrap();
    let x_min = captures.get(1).unwrap().as_str().parse().unwrap();
    let x_max = captures.get(2).unwrap().as_str().parse().unwrap();
    let y_min = captures.get(3).unwrap().as_str().parse().unwrap();
    let y_max = captures.get(4).unwrap().as_str().parse().unwrap();

    TargetArea {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Outcome {
    TooLow,
    TooSlow,
    TooFar,
    Hit(i32),
}

fn check_hit_target((mut x_vec, mut y_vec): (i32, i32), target: &TargetArea) -> Outcome {
    if x_vec < 0 {
        return Outcome::TooSlow;
    }

    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut y_max = 0;

    while x_pos <= target.x_max {
        x_pos += x_vec;
        y_pos += y_vec;

        y_max = max(y_max, y_pos);

        if target.hit_by(x_pos, y_pos) {
            return Outcome::Hit(y_max);
        }

        if y_pos < target.y_min && y_vec <= 0 {
            return Outcome::TooLow;
        }
        if x_pos < target.x_min && x_vec <= 0 {
            return Outcome::TooSlow;
        }

        x_vec += match x_vec.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        y_vec -= 1;
    }

    Outcome::TooFar
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn hit_by(&self, x: i32, y: i32) -> bool {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(
            parse("target area: x=20..30, y=-10..-5"),
            TargetArea {
                x_min: 20,
                x_max: 30,
                y_min: -10,
                y_max: -5
            }
        )
    }

    #[test]
    fn check_hit_target_works() {
        let target = TargetArea {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5,
        };

        assert_eq!(check_hit_target((7, 2), &target), Outcome::Hit(3));
        assert_eq!(check_hit_target((6, 3), &target), Outcome::Hit(6));
        assert_eq!(check_hit_target((9, 0), &target), Outcome::Hit(0));
        assert_eq!(check_hit_target((17, -4), &target), Outcome::TooFar);
    }

    #[test]
    fn find_best_works() {
        let target = TargetArea {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5,
        };

        assert_eq!(find_best(&target), (45, 112));
    }
}
