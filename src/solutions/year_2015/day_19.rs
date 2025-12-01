use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let (origin, mut replacements) = parse(input);
    for r in replacements.values_mut() {
        r.sort_by_key(|x| x.len());
        r.reverse();
    }
    println!("Part 1: {}", part_one(origin, &replacements));
    println!("Part 2: {}", part_two(origin, &replacements));
}

type Replacements<'a> = HashMap<&'a str, Vec<&'a str>>;

const REPLACEMENT_ARROW: &str = "=>";
const PLACEHOLDER: &str = "@";
const START: &str = "e";

fn parse(input: &str) -> (&str, Replacements<'_>) {
    let relevant_lines = parser::lines_as_strings(input);
    let origin = relevant_lines
        .iter()
        .find(|line| !line.contains(REPLACEMENT_ARROW))
        .unwrap();

    let mut replacements: Replacements = HashMap::new();
    for (find, replace) in relevant_lines
        .iter()
        .filter(|line| line.contains(REPLACEMENT_ARROW))
        .map(|line| parse_replacement(line))
    {
        (*replacements.entry(find).or_default()).push(replace);
    }

    (origin, replacements)
}

fn parse_replacement(line: &str) -> (&str, &str) {
    let parts: Vec<_> = line.split(REPLACEMENT_ARROW).collect();
    (parts[0].trim(), parts[1].trim())
}

fn part_one(origin: &str, replacements: &Replacements) -> usize {
    let mut set = HashSet::new();
    for (find, replacements) in replacements {
        for replace in replacements {
            for i in 0..origin.matches(find).count() {
                set.insert(replace_nth(origin, find, replace, i));
            }
        }
    }
    set.len()
}

fn part_two(aim: &str, replacements: &Replacements) -> usize {
    inner_two(aim, START, &reverse_replacements(replacements))
}

fn inner_two(origin: &str, aim: &str, replacements: &Replacements) -> usize {
    let mut result = 0;
    let mut current = origin.to_string();
    let keys = replacements
        .keys()
        .sorted_by_key(|x| x.len())
        .rev()
        .collect_vec();

    while current != aim {
        for key in &keys {
            if current.contains(*key) {
                let to = replacements
                    .get(*key)
                    .unwrap()
                    .iter()
                    .sorted_by_key(|x| x.len())
                    .next()
                    .unwrap();
                current = current.replacen(*key, to, 1);
                result += 1;
                break;
            }
        }
    }

    result
}

fn reverse_replacements<'a>(replacements: &'a Replacements) -> Replacements<'a> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();

    for (key, values) in replacements {
        for v in values {
            result.entry(*v).or_default().push(*key);
        }
    }

    result
}

fn replace_nth(word: &str, from: &str, to: &str, idx: usize) -> String {
    word.replacen(from, PLACEHOLDER, idx + 1)
        .replacen(PLACEHOLDER, from, idx)
        .replace(PLACEHOLDER, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "e => H
    e => O
    H => HO
    H => OH
    O => HH

    HOH";

    #[test]
    fn parse_works() {
        let (origin, replacements) = parse(INPUT);
        assert_eq!(origin, "HOH");
        assert_eq!(
            replacements,
            HashMap::from([
                ("H", vec!["HO", "OH"]),
                ("O", vec!["HH"]),
                ("e", vec!["H", "O"])
            ])
        );
    }

    #[test]
    fn replace_nth_works() {
        assert_eq!(replace_nth("alpha", "a", "b", 0), String::from("blpha"));
        assert_eq!(replace_nth("alpha", "a", "b", 1), String::from("alphb"));
    }

    #[test]
    fn part_one_works() {
        let (origin, replacements) = parse(INPUT);
        assert_eq!(part_one(origin, &replacements), 4);
    }

    #[test]
    fn part_two_works() {
        let (aim, replacements) = parse(INPUT);
        assert_eq!(part_two(aim, &replacements), 3);
    }
}
