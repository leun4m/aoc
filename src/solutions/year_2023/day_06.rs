use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    duration: usize,
    record_distance: usize,
}

fn parse(input: &str) -> Vec<Race> {
    let lines = parser::lines_custom(input, |line| {
        line.split_ascii_whitespace()
            .map(str::parse)
            .filter(std::result::Result::is_ok)
            .map(std::result::Result::unwrap)
            .collect_vec()
    });

    (0..lines[0].len())
        .map(|i| Race {
            duration: lines[0][i],
            record_distance: lines[1][i],
        })
        .collect()
}

fn pre_parse(input: &str) -> String {
    input.replace(' ', "").replace(':', " ")
}

fn part_one(input: &str) -> usize {
    multiply_races(&parse(input))
}

fn part_two(input: &str) -> usize {
    multiply_races(&parse(&pre_parse(input)))
}

fn multiply_races(races: &[Race]) -> usize {
    races.iter().map(calc_race).product()
}

fn calc_race(race: &Race) -> usize {
    (0..race.duration)
        .filter(|&x| x * (race.duration - x) > race.record_distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            vec![
                Race {
                    duration: 7,
                    record_distance: 9
                },
                Race {
                    duration: 15,
                    record_distance: 40
                },
                Race {
                    duration: 30,
                    record_distance: 200
                },
            ]
        )
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 288)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), 71503)
    }
}
