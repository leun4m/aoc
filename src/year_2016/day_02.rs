use std::cmp;

pub fn solve(input: &str) {
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
                Direction::U => (pos.0, pos.1 - 1),
                Direction::D => (pos.0, pos.1 + 1),
                Direction::L => (pos.0 - 1, pos.1),
                Direction::R => (pos.0 + 1, pos.1),
            };
            pos = clamp(&pos);
        }
        result.push(number_of_pos(&pos));
    }
    result
}

pub fn calc_number2(instructions: &[Vec<Direction>]) -> String {
    let mut result = String::new();
    let mut field = Starfield::N5;

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
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    NA,
    NB,
    NC,
    ND,
}

impl Starfield {
    fn move_to(&self, dir: &Direction) -> Starfield {
        match self {
            Starfield::N1 => match dir {
                Direction::D => Starfield::N3,
                _ => *self,
            },
            Starfield::N2 => match dir {
                Direction::R => Starfield::N3,
                Direction::D => Starfield::N6,
                _ => *self,
            },
            Starfield::N3 => match dir {
                Direction::U => Starfield::N1,
                Direction::L => Starfield::N2,
                Direction::R => Starfield::N4,
                Direction::D => Starfield::N7,
            },
            Starfield::N4 => match dir {
                Direction::L => Starfield::N3,
                Direction::D => Starfield::N8,
                _ => *self,
            },
            Starfield::N5 => match dir {
                Direction::R => Starfield::N6,
                _ => *self,
            },
            Starfield::N6 => match dir {
                Direction::U => Starfield::N2,
                Direction::D => Starfield::NA,
                Direction::L => Starfield::N5,
                Direction::R => Starfield::N7,
            },
            Starfield::N7 => match dir {
                Direction::U => Starfield::N3,
                Direction::D => Starfield::NB,
                Direction::L => Starfield::N6,
                Direction::R => Starfield::N8,
            },
            Starfield::N8 => match dir {
                Direction::U => Starfield::N4,
                Direction::D => Starfield::NC,
                Direction::L => Starfield::N7,
                Direction::R => Starfield::N9,
            },
            Starfield::N9 => match dir {
                Direction::L => Starfield::N8,
                _ => *self,
            },
            Starfield::NA => match dir {
                Direction::U => Starfield::N6,
                Direction::R => Starfield::NB,
                _ => *self,
            },
            Starfield::NB => match dir {
                Direction::U => Starfield::N7,
                Direction::D => Starfield::ND,
                Direction::L => Starfield::NA,
                Direction::R => Starfield::NC,
            },
            Starfield::NC => match dir {
                Direction::U => Starfield::N8,
                Direction::L => Starfield::NB,
                _ => *self,
            },
            Starfield::ND => match dir {
                Direction::U => Starfield::NB,
                _ => *self,
            },
        }
    }

    fn as_char(&self) -> char {
        match self {
            Starfield::N1 => '1',
            Starfield::N2 => '2',
            Starfield::N3 => '3',
            Starfield::N4 => '4',
            Starfield::N5 => '5',
            Starfield::N6 => '6',
            Starfield::N7 => '7',
            Starfield::N8 => '8',
            Starfield::N9 => '9',
            Starfield::NA => 'A',
            Starfield::NB => 'B',
            Starfield::NC => 'C',
            Starfield::ND => 'D',
        }
    }
}

pub fn parse(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'U' => Direction::U,
                    'D' => Direction::D,
                    'L' => Direction::L,
                    'R' => Direction::R,
                    x => panic!("Unexpected symbol: {}", x),
                })
                .collect()
        })
        .collect()
}

pub enum Direction {
    U,
    D,
    L,
    R,
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
