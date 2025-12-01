use std::cmp::max;

pub fn solve(input: &str) {
    let player_pos = parse(input);
    println!(
        "Part 1: {}",
        part_one(
            Player::create_at(player_pos.0),
            Player::create_at(player_pos.1)
        )
    );
    println!(
        "Part 2: {}",
        part_two(
            Player::create_at(player_pos.0),
            Player::create_at(player_pos.1)
        )
    );
}

type Position = u64;
const WIN_SCORE_PART_1: u64 = 1000;
const WIN_SCORE_PART_2: u64 = 21;
const DICE_MAX: u64 = 100;
const BOARD_MAX: Position = 10;

fn parse(input: &str) -> (Position, Position) {
    let players: Vec<Position> = input
        .lines()
        .filter(|line| line.contains("starting position"))
        .map(str::trim)
        .map(parse_line)
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

    let mut last_idx = 1;
    let mut current_idx = 0;

    let mut players = [player_1, player_2];

    while !players[last_idx].has_reached(WIN_SCORE_PART_1) {
        let rolled = dice.roll() + dice.roll() + dice.roll();
        players[current_idx].add_score(rolled);

        last_idx = current_idx;
        current_idx += 1;
        current_idx %= players.len();
    }

    let lost_player = players
        .iter()
        .find(|p| !p.has_reached(WIN_SCORE_PART_1))
        .unwrap();

    dice.counter * lost_player.score
}

fn part_two(player_1: Player, player_2: Player) -> u64 {
    let (player_1_wins, player_2_wins) = play_quantum(player_1, player_2, true);
    max(player_1_wins, player_2_wins)
}

const QUANTUM_DICE_RESULT_FREQUENCY: [(u64, u64); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_quantum(p1: Player, p2: Player, p1_turn: bool) -> (u64, u64) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    if p1_turn {
        for (dice_result, freq) in QUANTUM_DICE_RESULT_FREQUENCY {
            let p1_clone = p1.clone_add_score(dice_result);

            if p1_clone.has_reached(WIN_SCORE_PART_2) {
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
        self.position += score;
        if self.position.is_multiple_of(BOARD_MAX) {
            self.position = BOARD_MAX;
        } else {
            self.position %= BOARD_MAX;
        }
        self.score += self.position;
    }

    fn has_reached(&self, value: u64) -> bool {
        value <= self.score
    }

    fn clone_add_score(&self, score: u64) -> Self {
        let mut clone = self.clone();
        clone.add_score(score);
        clone
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(
            part_one(Player::create_at(4), Player::create_at(8)),
            739_785
        );
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two(Player::create_at(4), Player::create_at(8)),
            444_356_092_776_315
        );
    }
}
