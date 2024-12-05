use itertools::Itertools;

type Page = u32;

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    x: Page,
    y: Page,
}

type RuleSet = Vec<Rule>;
type PageUpdate = Vec<Page>;

pub fn solve(input: &str) {
    let (rules, updates) = parse(input);
    println!("Part 1: {}", part_one(&rules, &updates));
}

fn parse(input: &str) -> (RuleSet, Vec<PageUpdate>) {
    if let Some(parts) = input.split_once("\n\n") {
        (
            parts.0.lines().map(parse_rule).collect_vec(),
            parts.1.lines().map(parse_update).collect_vec(),
        )
    } else {
        panic!("Could not parse!")
    }
}

fn parse_rule(rule: &str) -> Rule {
    if let Some(numbers) = rule.split_once('|') {
        Rule {
            x: numbers.0.parse().unwrap(),
            y: numbers.1.parse().unwrap(),
        }
    } else {
        panic!("Cannot parse rule")
    }
}

fn parse_update(update: &str) -> PageUpdate {
    update
        .split(',')
        .map(|page| page.parse().unwrap())
        .collect()
}

fn part_one(rules: &RuleSet, updates: &[PageUpdate]) -> u32 {
    updates
        .iter()
        .filter(|update| update_ok(rules, update))
        .map(middle_page)
        .sum()
}

fn update_ok(rules: &RuleSet, update: &PageUpdate) -> bool {
    for a in 0..update.len() {
        for b in a + 1..update.len() {
            if against_rules(update[a], update[b], rules) {
                return false;
            }
        }
    }

    true
}

fn against_rules(a: Page, b: Page, rules: &RuleSet) -> bool {
    rules
        .iter()
        .filter(|rule| rule.x == b)
        .any(|rule| rule.y == a)
}

fn middle_page(update: &PageUpdate) -> Page {
    update[update.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_rule() {
        assert_eq!(Rule { x: 47, y: 53 }, parse_rule("47|53"));
        assert_eq!(Rule { x: 0, y: 87 }, parse_rule("0|87"));
        assert_eq!(Rule { x: 987, y: 123 }, parse_rule("987|123"));
    }

    #[test]
    fn test_parse_update() {
        assert_eq!(vec![47, 53], parse_update("47,53"));
        assert_eq!(vec![1, 2, 3, 4, 5], parse_update("1,2,3,4,5"));
    }

    #[test]
    fn test_part_one() {
        let (rules, updates) = parse(EXAMPLE_INPUT);
        assert_eq!(143, part_one(&rules, &updates));
    }
}
