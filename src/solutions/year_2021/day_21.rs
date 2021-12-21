pub fn solve(input: &str) {
    let (player_1, player_2) = parse(input);
    println!(
        "Part 1: {}",
        play(Player::create_at(player_1), Player::create_at(player_2))
    );
}

type Position = u64;
const WIN_SCORE: u64 = 1000;
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

fn play(player_1: Player, player_2: Player) -> u64 {
    let mut dice = Dice::new();
    let mut current_player_idx = 0;
    let mut players = [player_1, player_2];
    while !players[(players.len() + current_player_idx - 1) % players.len()].has_won() {
        let rolled = dice.roll() + dice.roll() + dice.roll();
        players[current_player_idx].add_score(rolled);
        
        // println!(
        //     "Player {}\trolled: {}\tnow_at: {}\tscore: {}",
        //     current_player_idx,
        //     rolled,
        //     players[current_player_idx].position,
        //     players[current_player_idx].score
        // );

        current_player_idx += 1;
        current_player_idx %= players.len();
    }
    let lost_player = players.iter().find(|p| !p.has_won()).unwrap();

    println!("{} / {}", dice.counter, lost_player.score());
    dice.counter * lost_player.score()
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn play_works() {
        assert_eq!(play(Player::create_at(4), Player::create_at(8)), 739785);
    }
}
