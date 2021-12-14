use crate::util;
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

fn parse(input: &str) -> (&str, InsertionMap) {
    let origin = input
        .lines()
        .filter(|line| !line.is_empty())
        .find(|line| !line.contains(ARROW))
        .unwrap();

    let insertions = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| line.contains(ARROW))
        .map(|line| parse_replacement(line))
        .collect();

    (origin, insertions)
}

fn parse_replacement(line: &str) -> ((char, char), char) {
    let parts: Vec<_> = line.split(ARROW).map(|part| part.trim()).collect();
    (
        (
            parts[0].chars().next().unwrap(),
            parts[0].chars().nth(1).unwrap(),
        ),
        parts[1].chars().next().unwrap(),
    )
}

fn iterate(origin: &str, insertions: &InsertionMap, iterations: usize) -> u32 {
    let mut result = origin.to_owned();
    for i in 0..iterations {
        result = process(&result, insertions);
        println!("Iteration: {}", i);
    }

    let char_counts = util::count_chars(&result);
    let max = char_counts.iter().map(|(_, v)| v).max().unwrap();
    let min = char_counts.iter().map(|(_, v)| v).min().unwrap();
    max - min
}

fn process(origin: &str, insertions: &InsertionMap) -> String {
    let mut new: String = origin
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|c| match insertions.get(&(c[0], c[1])) {
            Some(x) => format!("{}{}", c[0], x),
            None => format!("{}", c[0]),
        })
        .collect();
    new.push(origin.chars().last().unwrap());
    new
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
    fn process_works() {
        let insertions = HashMap::from(INSERTIONS);
        assert_eq!(process("NNCB", &insertions), "NCNBCHB");
        assert_eq!(process("NCNBCHB", &insertions), "NBCCNBBBCBHCB");
        assert_eq!(
            process("NBCCNBBBCBHCB", &insertions),
            "NBBBCNCCNBBNBNBBCHBHHBCHB"
        );
    }

    #[test]
    fn iterate_works() {
        let (start, insertions) = parse(INPUT);
        assert_eq!(iterate(start, &insertions, 10), 1588);
    }
}
