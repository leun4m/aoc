use crate::parser;

pub fn solve(input: &str) {
    let equations = parse(input);
    println!("Part 1: {}", part_one(&equations));
    println!("Part 2: {}", part_two(&equations));
}

fn parse(input: &str) -> Vec<Equation> {
    parser::lines_custom(input, parse_line)
        .into_iter()
        .flatten()
        .collect()
}

struct Equation {
    result: i64,
    values: Vec<i64>,
}

fn parse_line(line: &str) -> Option<Equation> {
    if let Some((result, values)) = line.split_once(':') {
        let equation = Equation {
            result: result.parse().unwrap(),
            values: values
                .split(' ')
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect(),
        };

        Some(equation)
    } else {
        None
    }
}

fn part_one(equations: &[Equation]) -> i64 {
    let chains = generate_chains(equations, &[Operator::Add, Operator::Multiply]);
    equations
        .iter()
        .filter(|x| is_valid_equation(x, &chains[x.values.len()]))
        .map(|x| x.result)
        .sum()
}

fn part_two(equations: &[Equation]) -> i64 {
    let chains = generate_chains(
        equations,
        &[Operator::Add, Operator::Multiply, Operator::Concatenation],
    );
    equations
        .iter()
        .filter(|x| is_valid_equation(x, &chains[x.values.len()]))
        .map(|x| x.result)
        .sum()
}

fn generate_chains(equations: &[Equation], operators: &[Operator]) -> Vec<Vec<Vec<Operator>>> {
    let mut map = Vec::new();

    for i in 0..=equations
        .iter()
        .map(|x| x.values.len())
        .max()
        .unwrap_or_default()
    {
        map.push(all_operator_chains(i, operators));
    }

    map
}

fn is_valid_equation(equation: &Equation, chains: &[Vec<Operator>]) -> bool {
    for operator_chain in chains {
        let mut value = equation.values[0];
        for i in 1..equation.values.len() {
            value = operator_chain[i - 1].apply(value, equation.values[i]);
        }
        if value == equation.result {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => format!("{a}{b}").parse().unwrap(),
        }
    }
}

fn all_operator_chains(len: usize, operators: &[Operator]) -> Vec<Vec<Operator>> {
    if len == 0 {
        Vec::new()
    } else if len == 1 {
        let mut vec = Vec::new();
        for operator in operators {
            vec.push(vec![*operator]);
        }
        vec
    } else {
        let mut all_chains: Vec<Vec<Operator>> = Vec::new();

        let prev_chains = {
            let len = len - 1;
            all_operator_chains(len, operators)
        };

        for operator in operators {
            for prev_chain in &prev_chains {
                let mut chain: Vec<Operator> = Vec::new();
                chain.append(&mut prev_chain.clone());
                chain.push(*operator);
                all_chains.push(chain);
            }
        }
        all_chains
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        assert_eq!(3749, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(11387, part_two(&parse(EXAMPLE_INPUT)));
    }
}
