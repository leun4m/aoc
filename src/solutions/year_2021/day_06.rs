pub fn solve(input: &str) {
    let fish = parse(input);
    println!("Part 1: {}", simulate(&fish, 80));
    println!("Part 2: {}", simulate(&fish, 256));
}

const FISH_TIME_RESET: usize = 6;
const FISH_TIME_NEW: usize = 8;
const FISH_ARRAY_SIZE: usize = FISH_TIME_NEW + 1;

/// An array storing currently living fish map-like in their states
/// - The index represents the state (aka days left for a newborn)
/// - The value stores the amount of fish in said state.
type FishArray = [u64; FISH_ARRAY_SIZE];

fn parse(input: &str) -> Vec<u8> {
    input
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn simulate(fish: &[u8], days: u64) -> u64 {
    let mut living_fish = to_array(fish);
    for _ in 0..days {
        pass_day(&mut living_fish);
    }
    living_fish.iter().sum()
}

fn to_array(fish: &[u8]) -> FishArray {
    let mut living_fish = [0; FISH_ARRAY_SIZE];
    for f in fish {
        living_fish[*f as usize] += 1;
    }
    living_fish
}

fn pass_day(fish: &mut FishArray) {
    let zeros = fish[0];
    for i in 1..FISH_ARRAY_SIZE {
        fish[i - 1] = fish[i];
    }
    fish[FISH_TIME_RESET] += zeros;
    fish[FISH_TIME_NEW] = zeros;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_works() {
        let output = parse(INPUT);
        assert_eq!(output, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn pass_day_works() {
        let mut input = [1, 1, 1, 2, 3, 0, 0, 3, 0];
        let output = [1, 1, 2, 3, 0, 0, 4, 0, 1];
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
    fn part_two_works() {
        let parsed = parse(INPUT);
        assert_eq!(simulate(&parsed, 256), 26_984_457_539);
    }
}
