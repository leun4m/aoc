use regex::Regex;
use std::collections::HashMap;

type BASE = u64;

pub fn main(input: &str) {
    let information = error_rate(input);
    println!("{}", information);
}

fn error_rate(input: &str) -> BASE {
    let regex_rule = Regex::new("([^:]*): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();

    let mut lines = input.lines();
    let mut line = lines.next();

    let mut rules: HashMap<String, Vec<(BASE, BASE)>> = HashMap::new();

    while !line.unwrap().is_empty() {
        let captures = regex_rule.captures(line.unwrap()).unwrap();

        let name = captures.get(1).unwrap().as_str().to_string();
        let i1 = (
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
        );
        let i2 = (
            captures.get(4).unwrap().as_str().parse().unwrap(),
            captures.get(5).unwrap().as_str().parse().unwrap(),
        );

        rules.insert(name, vec![i1, i2]);
        line = lines.next();
    }

    lines.next();
    line = lines.next();

    let _own_numbers: Vec<BASE> = line
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    lines.next();
    lines.next();
    line = lines.next();

    let mut error_numbers = Vec::new();

    while line.is_some() {
        line.unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .for_each(|x| {
                if !fits_any_rule(&rules, x) {
                    error_numbers.push(x);
                }
            });
        line = lines.next();
    }

    error_numbers.iter().sum()
}

fn fits_any_rule(rules: &HashMap<String, Vec<(BASE, BASE)>>, number: BASE) -> bool {
    rules
        .iter()
        .any(|(_, v)| v.iter().any(|x| x.0 <= number && number <= x.1))
}
