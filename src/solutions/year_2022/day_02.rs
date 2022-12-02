use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let matches = parse(input);
    println!("Part 1: {}", part_one(&matches));
    println!("Part 2: {}", part_two(&matches));
}

fn parse(input: &str) -> Vec<(OpponentSymbol, PlayerSymbol)> {
    parser::lines_custom(input, |line| {
        let symbols = line.trim().split(' ').map(parse_symbol).collect_vec();

        assert!(symbols.len() == 2);

        (symbols[0].get_opponent(), symbols[1].get_player())
    })
}

fn parse_symbol(input: &str) -> Symbol {
    match input {
        "A" => Symbol::Opponent(OpponentSymbol::A),
        "B" => Symbol::Opponent(OpponentSymbol::B),
        "C" => Symbol::Opponent(OpponentSymbol::C),
        "X" => Symbol::Player(PlayerSymbol::X),
        "Y" => Symbol::Player(PlayerSymbol::Y),
        "Z" => Symbol::Player(PlayerSymbol::Z),
        _ => panic!("Unexpected: {}", input),
    }
}

fn part_one(matches: &[(OpponentSymbol, PlayerSymbol)]) -> u32 {
    part_generic(matches, |(o, p)| (o.to_shape(), p.to_shape()))
}

fn part_two(matches: &[(OpponentSymbol, PlayerSymbol)]) -> u32 {
    part_generic(matches, |(o, p)| {
        (o.to_shape(), choose_shape(o.to_shape(), p.to_outcome()))
    })
}

fn part_generic<F>(matches: &[(OpponentSymbol, PlayerSymbol)], symbols_to_shapes: F) -> u32
where
    F: Fn(&(OpponentSymbol, PlayerSymbol)) -> (Shape, Shape),
{
    matches
        .iter()
        .map(symbols_to_shapes)
        .map(|(opponent, player)| player.play_against(opponent).score() + player.score())
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Symbol {
    Opponent(OpponentSymbol),
    Player(PlayerSymbol),
}

impl Symbol {
    fn get_opponent(self) -> OpponentSymbol {
        match self {
            Self::Opponent(x) => x,
            Self::Player(_) => panic!("Not opponend"),
        }
    }

    fn get_player(self) -> PlayerSymbol {
        match self {
            Self::Player(x) => x,
            Self::Opponent(_) => panic!("Not opponend"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpponentSymbol {
    A,
    B,
    C,
}

impl OpponentSymbol {
    fn to_shape(self) -> Shape {
        match self {
            OpponentSymbol::A => Shape::Rock,
            OpponentSymbol::B => Shape::Paper,
            OpponentSymbol::C => Shape::Scissors,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PlayerSymbol {
    X,
    Y,
    Z,
}

impl PlayerSymbol {
    fn to_shape(self) -> Shape {
        match self {
            PlayerSymbol::X => Shape::Rock,
            PlayerSymbol::Y => Shape::Paper,
            PlayerSymbol::Z => Shape::Scissors,
        }
    }

    fn to_outcome(self) -> Outcome {
        match self {
            PlayerSymbol::X => Outcome::Lose,
            PlayerSymbol::Y => Outcome::Draw,
            PlayerSymbol::Z => Outcome::Win,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_against(self, opponent: Shape) -> Outcome {
        match (self, opponent) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (a, b) => {
                if a == b {
                    Outcome::Draw
                } else {
                    Outcome::Lose
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn choose_shape(opponent: Shape, outcome: Outcome) -> Shape {
    *[Shape::Rock, Shape::Paper, Shape::Scissors]
        .iter()
        .find(|x| x.play_against(opponent) == outcome)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let input = "
        A Y
        B X
        C Z";
        assert_eq!(
            vec![
                (OpponentSymbol::A, PlayerSymbol::Y),
                (OpponentSymbol::B, PlayerSymbol::X),
                (OpponentSymbol::C, PlayerSymbol::Z)
            ],
            parse(input)
        );
    }

    #[test]
    fn part_one_works() {
        let matches = vec![
            (OpponentSymbol::A, PlayerSymbol::Y),
            (OpponentSymbol::B, PlayerSymbol::X),
            (OpponentSymbol::C, PlayerSymbol::Z),
        ];
        assert_eq!(15, part_one(&matches));
    }

    #[test]
    fn part_two_works() {
        let matches = vec![
            (OpponentSymbol::A, PlayerSymbol::Y),
            (OpponentSymbol::B, PlayerSymbol::X),
            (OpponentSymbol::C, PlayerSymbol::Z),
        ];
        assert_eq!(12, part_two(&matches));
    }
}
