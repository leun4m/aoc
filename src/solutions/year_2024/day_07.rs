use std::vec;

use crate::parser;

pub fn solve(input: &str) {
    let equations = parse(input);
    println!("Part 1: {}", part_one(&equations));
    println!("Part 2: {}", part_two(&equations));
}

fn parse(input: &str) -> Vec<Equation> {
    parser::lines_custom(input, parse_line)
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
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
                .split(" ")
                .map(|x| x.trim())
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
    equations
        .iter()
        .filter(|x| is_valid_equation(x, chains))
        .map(|x| x.result)
        .sum()
}

fn part_two(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter(|x| is_valid_equation(x, chains_with_concat))
        .map(|x| x.result)
        .sum()
}

fn is_valid_equation<F>(equation: &Equation, operator_retriever: F) -> bool
where
    F: Fn(usize) -> Vec<Vec<Operator>>,
{
    for operator_chain in operator_retriever(equation.values.len()) {
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
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

fn chains(len: usize) -> Vec<Vec<Operator>> {
    if len == 0 {
        Vec::new()
    } else if len == 1 {
        vec![vec![Operator::Add], vec![Operator::Multiply]]
    } else {
        let mut v: Vec<Vec<Operator>> = Vec::new();

        let a = chains(len - 1);

        for operator in [Operator::Add, Operator::Multiply] {
            for prev_line in &a {
                let mut line: Vec<Operator> = Vec::new();
                line.append(&mut prev_line.clone());
                line.push(operator);
                v.push(line);
            }
        }
        v
    }
}

fn chains_with_concat(len: usize) -> Vec<Vec<Operator>> {
    if len == 0 {
        Vec::new()
    } else if len == 1 {
        vec![
            vec![Operator::Add],
            vec![Operator::Multiply],
            vec![Operator::Concatenation],
        ]
    } else {
        let mut v: Vec<Vec<Operator>> = Vec::new();

        let a = chains_with_concat(len - 1);

        for operator in [Operator::Add, Operator::Multiply, Operator::Concatenation] {
            for prev_line in &a {
                let mut line: Vec<Operator> = Vec::new();
                line.append(&mut prev_line.clone());
                line.push(operator);
                v.push(line);
            }
        }
        v
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
    fn test_chains() {
        assert_eq!(
            vec![vec![Operator::Add], vec![Operator::Multiply]],
            chains(1)
        );
        assert_eq!(
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Multiply, Operator::Add],
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply]
            ],
            chains(2)
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(3749, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(11387, part_two(&parse(EXAMPLE_INPUT)));
    }
}
