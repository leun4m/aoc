use log::debug;

pub fn solve(input: &str) {
    let directions = parse(input);
    println!("Part 1: {}", part_one(&directions));
    println!("Part 2: {}", part_two(&directions));
}

fn parse(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| x.split_at(1))
        .map(|(direction, steps)| to_rotation(direction, steps))
        .collect()
}

fn to_rotation(direction: &str, steps: &str) -> Rotation {
    let num = steps.parse().expect("Should be a number");
    match direction {
        "L" => Rotation::Left(num),
        "R" => Rotation::Right(num),
        _ => panic!("Unexpected direction: {direction}"),
    }
}

const DIAL_START: isize = 50;
const DIAL_SIZE: isize = 100;

enum Rotation {
    Left(isize),
    Right(isize),
}

impl Rotation {
    fn steps(&self) -> isize {
        match self {
            Rotation::Left(x) | Rotation::Right(x) => *x,
        }
    }

    fn factor(&self) -> isize {
        match self {
            Rotation::Left(_) => -1,
            Rotation::Right(_) => 1,
        }
    }
}

fn part_one(rotations: &[Rotation]) -> usize {
    let mut dial = DIAL_START;
    let mut counter = 0;
    for rotation in rotations {
        dial = (dial + rotation.factor() * rotation.steps() + DIAL_SIZE) % DIAL_SIZE;
        if dial == 0 {
            counter += 1;
        }
        debug!("dial: {dial}");
    }
    counter
}

fn part_two(rotations: &[Rotation]) -> usize {
    let mut dial = DIAL_START;
    let mut counter = 0;
    for rotation in rotations {
        for _ in 0..rotation.steps() {
            dial = (dial + rotation.factor() * rotation.steps() + DIAL_SIZE) % DIAL_SIZE;
            if dial == 0 {
                counter += 1;
            }
            debug!("dial: {dial}");
        }
    }
    counter
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part_one_works() {
        assert_eq!(3, part_one(&parse(INPUT)));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(6, part_two(&parse(INPUT)));
    }
}