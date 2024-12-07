use itertools::Itertools;

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
    (0..=equations
        .iter()
        .map(|x| x.values.len())
        .max()
        .unwrap_or_default())
        .map(|x| all_operator_chains(x, operators))
        .collect()
}

fn is_valid_equation(equation: &Equation, chains: &[Vec<Operator>]) -> bool {
    chains.iter().any(|chain| {
        equation
            .values
            .iter()
            .skip(1)
            .zip(chain)
            .try_fold(equation.values[0], |acc, (&x, operator)| {
                let new_value = operator.apply(acc, x);
                if new_value > equation.result {
                    None
                } else {
                    Some(new_value)
                }
            })
            .map_or(false, |final_value| final_value == equation.result)
    })
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
        vec![vec![]]
    } else {
        operators
            .iter()
            .cartesian_product(all_operator_chains(len - 1, operators))
            .map(|(operator, prev_chain)| {
                let mut chain = prev_chain.to_vec();
                chain.push(*operator);

                chain
            })
            .collect()
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
