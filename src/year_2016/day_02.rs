use std::cmp;

pub fn main(input: &str) {
    let instructions = parse(input);
    println!("{}", calc_number(&instructions));
    println!("{}", calc_number2(&instructions));
}

pub fn calc_number(instructions: &[Vec<Direction>]) -> String {
    let mut result = String::new();
    let mut pos = (1, 1);

    for number in instructions {
        for dir in number {
            pos = match dir {
                Direction::Up => (pos.0, pos.1 - 1),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };
            pos = clamp(&pos);
        }
        result.push(number_of_pos(&pos));
    }
    result
}

pub fn calc_number2(instructions: &[Vec<Direction>]) -> String {
    let mut result = String::new();
    let mut field = Starfield::Five;

    for number in instructions {
        for dir in number {
            field = field.move_to(dir);
        }

        result.push(field.as_char());
    }

    result
}

#[derive(Clone, Copy)]
enum Starfield {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl Starfield {
    fn move_to(&self, dir: &Direction) -> Starfield {
        match self {
            Starfield::One => match dir {
                Direction::Down => Starfield::Three,
                _ => *self,
            },
            Starfield::Two => match dir {
                Direction::Right => Starfield::Three,
                Direction::Down => Starfield::Six,
                _ => *self,
            },
            Starfield::Three => match dir {
                Direction::Up => Starfield::One,
                Direction::Left => Starfield::Two,
                Direction::Right => Starfield::Four,
                Direction::Down => Starfield::Seven,
            },
            Starfield::Four => match dir {
                Direction::Left => Starfield::Three,
                Direction::Down => Starfield::Eight,
                _ => *self,
            },
            Starfield::Five => match dir {
                Direction::Right => Starfield::Six,
                _ => *self,
            },
            Starfield::Six => match dir {
                Direction::Up => Starfield::Two,
                Direction::Down => Starfield::A,
                Direction::Left => Starfield::Five,
                Direction::Right => Starfield::Seven,
            },
            Starfield::Seven => match dir {
                Direction::Up => Starfield::Three,
                Direction::Down => Starfield::B,
                Direction::Left => Starfield::Six,
                Direction::Right => Starfield::Eight,
            },
            Starfield::Eight => match dir {
                Direction::Up => Starfield::Four,
                Direction::Down => Starfield::C,
                Direction::Left => Starfield::Seven,
                Direction::Right => Starfield::Nine,
            },
            Starfield::Nine => match dir {
                Direction::Left => Starfield::Eight,
                _ => *self,
            },
            Starfield::A => match dir {
                Direction::Up => Starfield::Six,
                Direction::Right => Starfield::B,
                _ => *self,
            },
            Starfield::B => match dir {
                Direction::Up => Starfield::Seven,
                Direction::Down => Starfield::D,
                Direction::Left => Starfield::A,
                Direction::Right => Starfield::C,
            },
            Starfield::C => match dir {
                Direction::Up => Starfield::Eight,
                Direction::Left => Starfield::B,
                _ => *self,
            },
            Starfield::D => match dir {
                Direction::Up => Starfield::B,
                _ => *self,
            },
        }
    }

    fn as_char(&self) -> char {
        match self {
            Starfield::One => '1',
            Starfield::Two => '2',
            Starfield::Three => '3',
            Starfield::Four => '4',
            Starfield::Five => '5',
            Starfield::Six => '6',
            Starfield::Seven => '7',
            Starfield::Eight => '8',
            Starfield::Nine => '9',
            Starfield::A => 'A',
            Starfield::B => 'B',
            Starfield::C => 'C',
            Starfield::D => 'D',
        }
    }
}

pub fn parse(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    x => panic!("Unexpected symbol: {}", x),
                })
                .collect()
        })
        .collect()
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn number_of_pos(pos: &(i32, i32)) -> char {
    match pos {
        (0, 0) => '1',
        (1, 0) => '2',
        (2, 0) => '3',
        (0, 1) => '4',
        (1, 1) => '5',
        (2, 1) => '6',
        (0, 2) => '7',
        (1, 2) => '8',
        (2, 2) => '9',
        _ => panic!("Unexpected pos: {:?}", pos),
    }
}

fn clamp(pos: &(i32, i32)) -> (i32, i32) {
    (
        cmp::min(2, cmp::max(0, pos.0)),
        cmp::min(2, cmp::max(0, pos.1)),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example() {
        let input = "ULL
RRDDD
LURDL
UUUUD";
        assert_eq!("1985", calc_number(&parse(input)));
        assert_eq!("5DB3", calc_number2(&parse(input)));
    }
}
