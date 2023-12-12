use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &str) {
    let games = parse(input);
    println!("Part 1: {}", part_one(&games));
    println!("Part 2: {}", part_two(&games));
}

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

fn part_one(games: &[Game]) -> u32 {
    games.iter().filter(|x| can_play(x)).map(|x| x.id).sum()
}

fn part_two(games: &[Game]) -> u32 {
    games.iter().map(minimal_cubes).sum()
}

fn parse_line(input: &str) -> Game {
    let a = input.split(':').collect_vec();
    let id = a[0][4..].trim().parse().unwrap();
    let subsets = a[1].split(';').map(parse_subset).collect();
    Game::new(id, subsets)
}

fn parse_subset(input: &str) -> Subset {
    Subset::new()
        .r(try_find_capture(r"(\d+) red", input))
        .g(try_find_capture(r"(\d+) green", input))
        .b(try_find_capture(r"(\d+) blue", input))
}

fn try_find_capture(regex: &str, input: &str) -> u32 {
    if let Some(captures) = Regex::new(regex).unwrap().captures(input) {
        captures.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    }
}

fn can_play(game: &Game) -> bool {
    game.subsets
        .iter()
        .all(|subset| subset.reds <= MAX_R && subset.greens <= MAX_G && subset.blues <= MAX_B)
}

fn minimal_cubes(game: &Game) -> u32 {
    let mut r_min = 0;
    let mut g_min = 0;
    let mut b_min = 0;

    for subset in &game.subsets {
        r_min = std::cmp::max(r_min, subset.reds);
        g_min = std::cmp::max(g_min, subset.greens);
        b_min = std::cmp::max(b_min, subset.blues);
    }

    r_min * g_min * b_min
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn new(id: u32, subsets: Vec<Subset>) -> Self {
        Self { id, subsets }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Subset {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Subset {
    pub fn new() -> Self {
        Self {
            reds: 0,
            greens: 0,
            blues: 0,
        }
    }

    pub fn r(&mut self, red: u32) -> Self {
        self.reds = red;
        *self
    }

    pub fn g(&mut self, green: u32) -> Self {
        self.greens = green;
        *self
    }

    pub fn b(&mut self, blue: u32) -> Self {
        self.blues = blue;
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Game::new(
                    1,
                    vec![
                        Subset::new().b(3).r(4),
                        Subset::new().r(1).g(2).b(6),
                        Subset::new().g(2)
                    ]
                ),
                Game::new(
                    2,
                    vec![
                        Subset::new().b(1).g(2),
                        Subset::new().g(3).b(4).r(1),
                        Subset::new().g(1).b(1),
                    ]
                ),
                Game::new(
                    3,
                    vec![
                        Subset::new().g(8).b(6).r(20),
                        Subset::new().b(5).r(4).g(13),
                        Subset::new().g(5).r(1),
                    ]
                ),
                Game::new(
                    4,
                    vec![
                        Subset::new().g(1).r(3).b(6),
                        Subset::new().g(3).r(6),
                        Subset::new().g(3).b(15).r(14),
                    ]
                ),
                Game::new(
                    5,
                    vec![Subset::new().r(6).b(1).g(3), Subset::new().b(2).r(1).g(2),]
                )
            ],
            parse(EXAMPLE_INPUT),
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(8, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2286, part_two(&parse(EXAMPLE_INPUT)));
    }
}
