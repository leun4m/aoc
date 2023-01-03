pub fn solve(input: &str) {
    let depths = parse(input);
    println!("Part 1: {}", part_one(&depths));
    println!("Part 2: {}", part_two(&depths));
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> Direction {
    let parts: Vec<_> = input.trim().split(' ').collect();
    let dir = parts[0];
    let value = parts[1].parse().unwrap();
    match dir {
        "forward" => Direction::Forward(value),
        "down" => Direction::Down(value),
        "up" => Direction::Up(value),
        x => panic!("Unexpected direction: {x}"),
    }
}

fn part_one(directions: &[Direction]) -> i32 {
    let coordinates = move_without_aim(directions);
    coordinates.0 * coordinates.1
}

fn part_two(directions: &[Direction]) -> i32 {
    let coordinates = move_with_aim(directions);
    coordinates.0 * coordinates.1
}

fn move_without_aim(directions: &[Direction]) -> (i32, i32) {
    directions
        .iter()
        .map(|&dir| match dir {
            Direction::Forward(x) => (x, 0),
            Direction::Up(x) => (0, -x),
            Direction::Down(x) => (0, x),
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap()
}

fn move_with_aim(directions: &[Direction]) -> (i32, i32) {
    let position = directions
        .iter()
        .fold(Position::default(), |acc, dir| match dir {
            Direction::Forward(x) => Position {
                horizontal: acc.horizontal + x,
                depth: acc.depth + acc.aim * x,
                aim: acc.aim,
            },
            Direction::Up(x) => Position {
                horizontal: acc.horizontal,
                depth: acc.depth,
                aim: acc.aim - x,
            },
            Direction::Down(x) => Position {
                horizontal: acc.horizontal,
                depth: acc.depth,
                aim: acc.aim + x,
            },
        });

    (position.horizontal, position.depth)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const DIRECTIONS: &[Direction] = &[
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];

    #[test]
    fn parse_works() {
        assert_eq!(parse(INPUT), DIRECTIONS);
    }

    #[test]
    fn move_without_aim_works() {
        assert_eq!(move_without_aim(DIRECTIONS), (15, 10));
    }

    #[test]
    fn move_with_aim_works() {
        assert_eq!(move_with_aim(DIRECTIONS), (15, 60));
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(DIRECTIONS), 150);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(DIRECTIONS), 900);
    }
}
