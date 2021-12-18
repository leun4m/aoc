use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::str::Lines;

pub fn solve(input: &str) {
    let (rules, own_ticket, tickets) = parse_input(input);
    let mut possibilities = get_all_possibilities(&rules, &own_ticket);
    let (error_rate, other_numbers) = analyse_tickets(&tickets, &rules);

    println!("Part 1: {}", error_rate);
    let mapping = detect_mapping(&mut possibilities, &rules, &other_numbers);
    println!("Part 2: {}", multiply_special_fields(&mapping, &own_ticket));
}

const INPUT_HEADLINE_YOUR_TICKET: &str = "your ticket:";
const INPUT_HEADLINE_OTHER_TICKETS: &str = "nearby tickets:";
const FIELD_PREFIX: &str = "departure";

type TicketNumber = u64;
type Ticket = Vec<TicketNumber>;
type RuleIntervals<'a> = HashMap<&'a str, Vec<(TicketNumber, TicketNumber)>>;
type RuleChoices<'a> = HashMap<&'a str, HashSet<usize>>;
type RuleIndex<'a> = HashMap<&'a str, usize>;
type PositionTicketNumbers = HashMap<usize, HashSet<TicketNumber>>;

fn parse_input(input: &str) -> (RuleIntervals, Ticket, Vec<Ticket>) {
    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);
    skip_lines_until(&mut lines, INPUT_HEADLINE_YOUR_TICKET);
    let own_ticket = parse_ticket(lines.next().unwrap());
    skip_lines_until(&mut lines, INPUT_HEADLINE_OTHER_TICKETS);
    let nearby_tickets = parse_tickets(&mut lines);
    (rules, own_ticket, nearby_tickets)
}

fn parse_rules<'a>(lines: &mut std::str::Lines<'a>) -> RuleIntervals<'a> {
    let regex_rule = Regex::new("([^:]*): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();
    let mut rules = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }
        let captures = regex_rule.captures(line).unwrap();

        let name = captures.get(1).unwrap().as_str();
        let i1 = parse_interval(&captures, 2, 3);
        let i2 = parse_interval(&captures, 4, 5);

        rules.insert(name, vec![i1, i2]);
    }

    rules
}

fn parse_interval(captures: &Captures, start: usize, end: usize) -> (TicketNumber, TicketNumber) {
    (
        captures.get(start).unwrap().as_str().parse().unwrap(),
        captures.get(end).unwrap().as_str().parse().unwrap(),
    )
}

fn parse_tickets(lines: &mut Lines) -> Vec<Ticket> {
    let mut tickets = Vec::new();
    for line in lines {
        tickets.push(parse_ticket(line));
    }
    tickets
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}

fn skip_lines_until(lines: &mut Lines, text: &str) {
    let mut line = lines.next();
    while !line.unwrap().starts_with(text) {
        line = lines.next();
    }
}

fn multiply_special_fields(
    rule_possibilities: &RuleIndex,
    own_ticket: &[TicketNumber],
) -> TicketNumber {
    rule_possibilities
        .iter()
        .filter(|(rule, _)| rule.starts_with(FIELD_PREFIX))
        .map(|(_, index)| own_ticket[*index])
        .product()
}

fn detect_mapping<'a>(
    rule_choices: &mut RuleChoices<'a>,
    rules: &RuleIntervals<'a>,
    position_numbers: &PositionTicketNumbers,
) -> RuleIndex<'a> {
    for (idx, numbers) in position_numbers {
        for &value in numbers {
            for (name, intervals) in rules {
                if !intervals
                    .iter()
                    .any(|interval| interval.0 <= value && value <= interval.1)
                {
                    let mut set = rule_choices.get(*name).unwrap().clone();
                    set.remove(idx);
                    rule_choices.insert(name, set);
                }
            }
        }
    }

    while rule_choices.values().any(|choices| choices.len() > 1) {
        for (rule, choices) in rule_choices.clone() {
            if choices.len() == 1 {
                rule_choices
                    .iter_mut()
                    .filter(|(&key, _)| key != rule)
                    .map(|(_, set)| set)
                    .for_each(|set| {
                        set.remove(choices.iter().next().unwrap());
                    });
            }
        }
    }

    rule_choices
        .iter()
        .map(|(name, indices)| (*name, *indices.iter().next().unwrap()))
        .collect()
}

fn get_all_possibilities<'a>(
    rules: &RuleIntervals<'a>,
    own_ticket: &[TicketNumber],
) -> RuleChoices<'a> {
    let mut possibilities = HashMap::new();
    for name in rules.keys() {
        possibilities.insert(*name, (0..own_ticket.len()).collect());
    }
    possibilities
}

fn analyse_tickets(
    tickets: &[Ticket],
    rules: &RuleIntervals,
) -> (TicketNumber, PositionTicketNumbers) {
    let mut error_numbers = Vec::new();
    let mut position_numbers: PositionTicketNumbers = HashMap::new();

    for ticket in tickets {
        let in_range = |&x| {
            if fits_any_rule(rules, x) {
                true
            } else {
                error_numbers.push(x);
                false
            }
        };

        if ticket.iter().all(in_range) {
            for (index, &number) in ticket.iter().enumerate() {
                let mut numbers_at_index = if let Some(x) = position_numbers.get(&index) {
                    x.clone()
                } else {
                    HashSet::new()
                };
                numbers_at_index.insert(number);
                position_numbers.insert(index, numbers_at_index);
            }
        }
    }

    (error_numbers.iter().sum(), position_numbers)
}

fn fits_any_rule(rules: &RuleIntervals, number: TicketNumber) -> bool {
    rules
        .values()
        .any(|v| v.iter().any(|x| x.0 <= number && number <= x.1))
}
