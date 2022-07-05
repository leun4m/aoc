use crate::{util, parser};

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let rules = parse(input);
    println!("Part 1: {}", part_one(&rules));
    println!("Part 2: {}", part_two(&rules));
}

fn parse(input: &str) -> Rules {
    let mut rules = Rules::new();
    for (from, to, happiness) in parser::parse_custom(input, parse_rule) {
        rules.add((&from, &to, happiness));
    }

    rules
}

fn parse_rule(line: &str) -> (String, String, i32) {
    // Alice would gain 54 happiness units by sitting next to Bob.
    let regex =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
    let capture = regex.captures(line).expect("Looks weird");

    let from = capture[1].parse::<String>().unwrap();
    let gain = capture[2].parse::<String>().unwrap();
    let factor = if gain == "gain" { 1 } else { -1 };
    let happiness = factor * capture[3].parse::<i32>().unwrap();
    let to = capture[4].parse::<String>().unwrap();
    (from, to, happiness)
}

fn part_one(rules: &Rules) -> i32 {
    get_best_seating(rules, false)
}

fn part_two(rules: &Rules) -> i32 {
    get_best_seating(rules, true)
}

fn get_best_seating(rules: &Rules, add_neutral: bool) -> i32 {
    let mut people = rules.people();

    if add_neutral {
        people.push("leun4m".to_string());
    }

    util::permutation_heap(&mut people)
        .iter()
        .map(|table| calc_happiness(table, rules))
        .sorted_by(|b, a| a.cmp(b))
        .next()
        .unwrap()
}

fn calc_happiness(people: &[String], rules: &Rules) -> i32 {
    let mut result = rules.get(people.iter().last().unwrap(), people.iter().next().unwrap());
    for i in 0..(people.len() - 1) {
        result += rules.get(&people[i], &people[i + 1]);
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct Rules {
    rules: HashMap<(String, String), i32>,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            rules: HashMap::new(),
        }
    }

    pub fn add(&mut self, (from, to, value): (&str, &str, i32)) {
        self.rules.insert((from.to_string(), to.to_string()), value);
    }

    fn get_single(&self, from: &str, to: &str) -> i32 {
        *self
            .rules
            .get(&(from.to_string(), to.to_string()))
            .unwrap_or(&0)
    }

    pub fn get(&self, from: &str, to: &str) -> i32 {
        self.get_single(from, to) + self.get_single(to, from)
    }

    pub fn people(&self) -> Vec<String> {
        self.rules
            .keys()
            .map(|(from, _)| from.clone())
            .unique()
            .sorted()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let input = "
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

        let mut expected_rules = Rules::new();
        expected_rules.add(("Alice", "Bob", 54));
        expected_rules.add(("Alice", "Carol", -79));
        expected_rules.add(("Alice", "David", -2));
        expected_rules.add(("Bob", "Alice", 83));
        expected_rules.add(("Bob", "Carol", -7));
        expected_rules.add(("Bob", "David", -63));
        expected_rules.add(("Carol", "Alice", -62));
        expected_rules.add(("Carol", "Bob", 60));
        expected_rules.add(("Carol", "David", 55));
        expected_rules.add(("David", "Alice", 46));
        expected_rules.add(("David", "Bob", -7));
        expected_rules.add(("David", "Carol", 41));

        assert_eq!(parse(input), expected_rules);
    }

    #[test]
    fn parse_rule_works() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        assert_eq!(
            parse_rule(input),
            ("Alice".to_string(), "Bob".to_string(), 54)
        );
    }

    #[test]
    fn rules_people_works() {
        let mut expected_rules = Rules::new();
        expected_rules.add(("Alice", "Bob", 54));
        expected_rules.add(("Alice", "Carol", -79));
        expected_rules.add(("Alice", "David", -2));
        expected_rules.add(("Bob", "Alice", 83));
        expected_rules.add(("Bob", "Carol", -7));
        expected_rules.add(("Bob", "David", -63));
        expected_rules.add(("Carol", "Alice", -62));
        expected_rules.add(("Carol", "Bob", 60));
        expected_rules.add(("Carol", "David", 55));
        expected_rules.add(("David", "Alice", 46));
        expected_rules.add(("David", "Bob", -7));
        expected_rules.add(("David", "Carol", 41));

        assert_eq!(
            expected_rules.people(),
            vec![
                "Alice".to_string(),
                "Bob".to_string(),
                "Carol".to_string(),
                "David".to_string()
            ]
        );
    }

    #[test]
    fn part_one_works() {
        let input = "
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

        assert_eq!(part_one(&parse(input)), 330);
    }
}
