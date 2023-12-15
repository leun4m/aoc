use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(&parse_one(input)));
    println!("Part 2: {}", part_two(&parse_two(input)));
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    duration: usize,
    record_distance: usize,
}

fn parse_one(input: &str) -> Vec<Race> {
    let lines = parser::lines_as_strings(input);
    let durations = lines[0]
        .split_ascii_whitespace()
        .into_iter()
        .map(|x| x.parse())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect_vec();
    let record_distances = lines[1]
        .split_ascii_whitespace()
        .into_iter()
        .map(|x| x.parse())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect_vec();

    (0..durations.len())
        .map(|i| Race {
            duration: durations[i],
            record_distance: record_distances[i],
        })
        .collect()
}

fn parse_two(input: &str) -> Race {
    let lines = parser::lines_as_strings(input);

    let duration = lines[0]
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    let record_distance = lines[1]
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or_default();

    Race {
        duration,
        record_distance,
    }
}

fn part_one(races: &[Race]) -> usize {
    races.iter().map(calc_race).product()
}

fn part_two(race: &Race) -> usize {
    calc_race(race)
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
            parse_one(EXAMPLE_INPUT),
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
        assert_eq!(part_one(&parse_one(EXAMPLE_INPUT)), 288)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse_two(EXAMPLE_INPUT)), 71503)
    }
}
