use std::collections::HashMap;

pub fn solve(input: &str) {
    let fish = parse(input);
    println!("Part 1: {}", simulate(&fish, 80));
    println!("Part 2: {}", simulate(&fish, 256));
}

fn parse(input: &str) -> Vec<u8> {
    input
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

const FISH_RESET: u8 = 6;
const FISH_NEW: u8 = 8;

fn simulate(fish: &[u8], days: u64) -> u64 {
    let mut current_fish = to_hash_map(fish);
    for _ in 0..days {
        pass_day(&mut current_fish);
    }
    current_fish.values().sum()
}

fn to_hash_map(fish: &[u8]) -> HashMap<u8, u64> {
    let mut result: HashMap<u8, u64> = HashMap::with_capacity((FISH_NEW + 1) as usize);
    for f in fish {
        *result.entry(*f).or_default() += 1;
    }
    result
}

fn pass_day(fish: &mut HashMap<u8, u64>) {
    let zeros = fish.remove(&0).unwrap_or(0);
    for i in 1..FISH_NEW+1 {
        let fish_in_state = fish.remove(&i).unwrap_or(0);
        fish.insert(i - 1, fish_in_state);
    }
    *fish.entry(FISH_RESET).or_default() += zeros;
    fish.insert(FISH_NEW, zeros);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_works() {
        let output = parse(INPUT);
        assert_eq!(output, vec![3, 4, 3, 1, 2])
    }

    #[test]
    fn pass_day_works() {
        let mut input = HashMap::from([(1, 1), (2, 1), (3, 2), (4, 3)]);
        let output = HashMap::from([(0, 1), (1, 1), (2, 2), (3, 3), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)]);
        pass_day(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn part_one_works() {
        let parsed = parse(INPUT);
        assert_eq!(simulate(&parsed, 18), 26);
        assert_eq!(simulate(&parsed, 80), 5934);
    }

    #[test]
    #[ignore]
    fn part_two_works() {
        let parsed = parse(INPUT);
        assert_eq!(simulate(&parsed, 256), 26984457539);
    }
}
