use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

use crate::parser;

#[derive(Debug, PartialEq, Eq)]
struct Aunt {
    id: u32,
    data: AuntData,
}

type AuntData = HashMap<String, u32>;

static SUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Sue (\d+): ([^$]+)").unwrap());
static ATTRIBUTE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\w+): (\d+)").unwrap());

pub fn solve(input: &str) {
    let aunts = parser::lines_custom(input, parse_aunt);

    println!("Part 1: {}", part_one(&aunts));
    println!("Part 2: {}", part_two(&aunts));
}

fn parse_aunt(line: &str) -> Aunt {
    let mut result = HashMap::new();
    // Sue 1: goldfish: 6, trees: 9, akitas: 0
    let captures = SUE_REGEX.captures(line).expect("Looks weird");

    let id = captures[1].parse().unwrap();
    let attributes: String = captures[2].parse().unwrap();

    for attribute in attributes.split(',') {
        let attribute_captures = ATTRIBUTE_REGEX.captures(attribute).unwrap();
        let key = attribute_captures[1].parse().unwrap();
        let value = attribute_captures[2].parse().unwrap();
        result.insert(key, value);
    }

    Aunt { id, data: result }
}

fn part_one(aunts: &[Aunt]) -> u32 {
    aunts.iter().find(|aunt| matches_aunt(aunt)).unwrap().id
}

fn part_two(aunts: &[Aunt]) -> u32 {
    aunts.iter().find(|aunt| matches_aunt_v2(aunt)).unwrap().id
}

fn matches_aunt(aunt: &Aunt) -> bool {
    get_searched_for()
        .iter()
        .all(|(key, desired_value)| match aunt.data.get(key) {
            None => true,
            Some(v) => v == desired_value,
        })
}

fn matches_aunt_v2(aunt: &Aunt) -> bool {
    get_searched_for()
        .iter()
        .all(|(key, desired_value)| match aunt.data.get(key) {
            None => true,
            Some(v) => match key.as_str() {
                "cats" | "trees" => v > desired_value,
                "pomeranians" | "goldfish" => v < desired_value,
                _ => v == desired_value,
            },
        })
}

fn get_searched_for() -> AuntData {
    HashMap::from([
        (String::from("children"), 3),
        (String::from("cats"), 7),
        (String::from("samoyeds"), 2),
        (String::from("pomeranians"), 3),
        (String::from("akitas"), 0),
        (String::from("vizslas"), 0),
        (String::from("goldfish"), 5),
        (String::from("trees"), 3),
        (String::from("cars"), 2),
        (String::from("perfumes"), 1),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_aunt_works() {
        assert_eq!(
            parse_aunt("Sue 1: goldfish: 6, trees: 9, akitas: 0"),
            Aunt {
                id: 1,
                data: HashMap::from([
                    ("goldfish".into(), 6),
                    ("trees".into(), 9),
                    ("akitas".into(), 0)
                ])
            }
        );
    }
}
