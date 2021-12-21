use std::cmp::max;

pub fn solve(input: &str) {
    let (player_1, player_2) = parse(input);
    println!(
        "Part 1: {}",
        part_one(Player::create_at(player_1), Player::create_at(player_2))
    );
    println!(
        "Part 2: {}",
        part_two(Player::create_at(player_1), Player::create_at(player_2))
    );
}

type Position = u64;
const WIN_SCORE: u64 = 1000;
const WIN_SCORE_2: u64 = 21;
const DICE_MAX: u64 = 100;
const BOARD_MIN: Position = 1;
const BOARD_MAX: Position = 10;

fn parse(input: &str) -> (Position, Position) {
    let players: Vec<Position> = input
        .lines()
        .filter(|line| line.contains("starting position"))
        .map(|line| line.trim())
        .map(|line| parse_line(line))
        .collect();
    (players[0], players[1])
}

fn parse_line(input: &str) -> Position {
    input
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn part_one(player_1: Player, player_2: Player) -> u64 {
    let mut dice = Dice::new();
    let mut current_player_idx = 0;
    let mut players = [player_1, player_2];
    while !players[(players.len() + current_player_idx - 1) % players.len()].has_won() {
        let rolled = dice.roll() + dice.roll() + dice.roll();
        players[current_player_idx].add_score(rolled);

        current_player_idx += 1;
        current_player_idx %= players.len();
    }
    let lost_player = players.iter().find(|p| !p.has_won()).unwrap();

    dice.counter * lost_player.score()
}

fn part_two(player_1: Player, player_2: Player) -> u64 {
    let (p1, p2) = play_quantum(player_1, player_2, true);
    max(p1, p2)
}

const QUANTUM_DICE_RESULT_FREQUENCY: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_quantum(p1: Player, p2: Player, p1_turn: bool) -> (u64, u64) {
    let mut result_1 = 0;
    let mut result_2 = 0;
    
    if p1_turn {
        for (dice_result, freq) in QUANTUM_DICE_RESULT_FREQUENCY {
            let mut p1_clone = p1.clone();
            p1_clone.add_score(dice_result);

            if p1_clone.has_won2() {
                result_1 += freq;
            } else {
                let (x1, x2) = play_quantum(p1_clone, p2.clone(), false);
                result_1 += x1 * freq;
                result_2 += x2 * freq;
            }
        }
    } else {
        let (x1, x2) = play_quantum(p2, p1, true);
        result_1 = x2;
        result_2 = x1;
    }

    (result_1, result_2)
}

struct Dice {
    score: u64,
    counter: u64,
}

impl Dice {
    fn new() -> Self {
        Self {
            score: DICE_MAX,
            counter: 0,
        }
    }

    fn roll(&mut self) -> u64 {
        self.counter += 1;
        self.score += 1;
        if self.score > DICE_MAX {
            self.score = 1;
        }
        self.score
    }
}

#[derive(Clone)]
struct Player {
    position: Position,
    score: u64,
}

impl Player {
    fn create_at(position: Position) -> Self {
        Self { position, score: 0 }
    }

    fn add_score(&mut self, score: u64) {
        for _ in 0..score {
            self.position += 1;
            if self.position > BOARD_MAX {
                self.position = BOARD_MIN;
            }
        }
        self.score += self.position;
    }

    fn score(&self) -> u64 {
        self.score
    }

    fn has_won(&self) -> bool {
        WIN_SCORE <= self.score
    }

    fn has_won2(&self) -> bool {
        WIN_SCORE_2 <= self.score
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(Player::create_at(4), Player::create_at(8)), 739785);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two(Player::create_at(4), Player::create_at(8)),
            444356092776315
        );
    }
}
