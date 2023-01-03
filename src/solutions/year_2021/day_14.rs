use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let (start, insertions) = parse(input);
    println!("Part 1: {}", iterate(start, &insertions, STEPS_PART_ONE));
    println!("Part 2: {}", iterate(start, &insertions, STEPS_PART_TWO));
}

const ARROW: &str = "->";
const STEPS_PART_ONE: usize = 10;
const STEPS_PART_TWO: usize = 40;

type InsertionMap = HashMap<(char, char), char>;
type PatternCount = HashMap<(char, char), u64>;
type CharCount = HashMap<char, u64>;

fn parse(input: &str) -> (&str, InsertionMap) {
    let origin = input
        .lines()
        .filter(|line| !line.is_empty())
        .find(|line| !line.contains(ARROW))
        .unwrap();

    let insertions = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| line.contains(ARROW))
        .map(parse_replacement)
        .collect();

    (origin, insertions)
}

fn parse_replacement(line: &str) -> ((char, char), char) {
    let parts: Vec<_> = line.split(ARROW).map(str::trim).collect();
    (
        (
            parts[0].chars().next().unwrap(),
            parts[0].chars().nth(1).unwrap(),
        ),
        parts[1].chars().next().unwrap(),
    )
}

fn iterate(origin: &str, insertions: &InsertionMap, iterations: usize) -> u64 {
    let mut pattern_count = to_pattern_count(origin);

    for _ in 0..iterations {
        pattern_count = process(&pattern_count, insertions);
    }

    if let MinMax(min, max) = count_chars(&pattern_count, origin).values().minmax() {
        max - min
    } else {
        println!("Found no min-max!");
        0
    }
}

fn to_pattern_count(origin: &str) -> PatternCount {
    let mut pattern_count = PatternCount::new();

    for pair in origin
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|c| (c[0], c[1]))
    {
        *pattern_count.entry(pair).or_default() += 1;
    }

    pattern_count
}

fn count_chars(pattern_count: &PatternCount, origin: &str) -> CharCount {
    let mut result = CharCount::new();

    for (c, n) in pattern_count.iter().map(|((a, _), n)| (a, n)) {
        *result.entry(*c).or_default() += n;
    }
    *result.entry(origin.chars().last().unwrap()).or_default() += 1;

    result
}

fn process(pattern_count: &PatternCount, insertions: &InsertionMap) -> PatternCount {
    let mut result = PatternCount::new();

    for (pair, n) in pattern_count {
        let (a, b) = (pair.0, pair.1);
        let c = *insertions.get(&(a, b)).unwrap();
        *result.entry((a, c)).or_default() += n;
        *result.entry((c, b)).or_default() += n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    const INSERTIONS: [((char, char), char); 16] = [
        (('C', 'H'), 'B'),
        (('H', 'H'), 'N'),
        (('C', 'B'), 'H'),
        (('N', 'H'), 'C'),
        (('H', 'B'), 'C'),
        (('H', 'C'), 'B'),
        (('H', 'N'), 'C'),
        (('N', 'N'), 'C'),
        (('B', 'H'), 'H'),
        (('N', 'C'), 'B'),
        (('N', 'B'), 'B'),
        (('B', 'N'), 'B'),
        (('B', 'B'), 'N'),
        (('B', 'C'), 'B'),
        (('C', 'C'), 'N'),
        (('C', 'N'), 'C'),
    ];

    #[test]

    fn parse_works() {
        let (start, insertions) = parse(INPUT);
        assert_eq!(start, "NNCB");
        assert_eq!(insertions, HashMap::from(INSERTIONS))
    }

    #[test]
    fn iterate_works() {
        let (start, insertions) = parse(INPUT);
        assert_eq!(iterate(start, &insertions, 10), 1588);
        assert_eq!(iterate(start, &insertions, 40), 2_188_189_693_529);
    }
}
