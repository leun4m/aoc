use regex::Regex;
use std::collections::HashMap;

const SEARCHED: &str = "shiny gold";

pub fn solve(input: &str) {
    let rules = parse_file(input);
    let part_one = check(&rules);
    let part_two = bags_to_be_contained(&rules, SEARCHED);

    // for (key, value) in rules.iter() {
    //     println!("{}: {:?}", key, value);
    // }

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

fn bags_to_be_contained(rules: &HashMap<String, HashMap<String, u32>>, key: &str) -> u32 {
    if let Some(inside) = rules.get(key) {
        if !inside.is_empty() {
            let i = inside
                .iter()
                .map(|x| x.1 + x.1 * bags_to_be_contained(rules, x.0))
                .sum();
            return i;
        }
    }
    0
}

fn check(rules: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut result = 0;

    for key in rules.keys() {
        if can_contain(rules, key) {
            result += 1;
        }
    }

    result
}

fn can_contain(rules: &HashMap<String, HashMap<String, u32>>, key: &str) -> bool {
    if let Some(contained) = rules.get(key) {
        if contained.contains_key(SEARCHED) {
            true
        } else {
            contained.iter().any(|a| can_contain(rules, a.0))
        }
    } else {
        false
    }
}

fn parse_file(input: &str) -> HashMap<String, HashMap<String, u32>> {
    let mut result = HashMap::new();

    let reg_left = Regex::new(r"(\w+ \w+) bags contain ").unwrap();
    // let reg_contains = Regex::new(r"(\w+ \w+) bags contain ((\d+ \w+ \w+ bags?(, |.))+)").unwrap();
    let reg_right = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    for line in input.lines() {
        if let Some(a) = reg_left.captures(line) {
            let color = a.get(1).unwrap().as_str().to_string();
            let mut map = HashMap::new();
            for expression in reg_left.replace_all(line, "").split(',') {
                if let Some(captures) = reg_right.captures(expression) {
                    map.insert(
                        captures.get(2).unwrap().as_str().to_string(),
                        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    );
                }
            }
            result.insert(color, map);
        }
    }

    result
}
