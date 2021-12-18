use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let matrix = CharMatrix::build(&parse(input));
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|x| !x.is_empty()).collect()
}

fn part_one(matrix: &CharMatrix) -> String {
    matrix.most_common()
}

fn part_two(matrix: &CharMatrix) -> String {
    matrix.least_common()
}

#[derive(Debug, PartialEq, Eq)]
struct CharMatrix {
    field: Vec<HashMap<char, usize>>,
    size: usize,
}

impl CharMatrix {
    pub fn build(words: &[&str]) -> Self {
        let first = words[0];
        let mut matrix = CharMatrix::new(first.len());
        for word in words {
            matrix.add(word);
        }
        matrix
    }

    pub fn least_common(&self) -> String {
        self.field
            .iter()
            .map(|x| Self::calc_least_common(x))
            .collect()
    }

    pub fn most_common(&self) -> String {
        self.field
            .iter()
            .map(|x| Self::calc_most_common(x))
            .collect()
    }

    fn new(size: usize) -> Self {
        let mut field = Vec::new();
        for _ in 0..size {
            field.push(HashMap::new());
        }

        Self { field, size }
    }

    fn add(&mut self, word: &str) {
        if word.len() != self.size {
            panic!("Word has different length");
        }

        for (i, c) in word.chars().enumerate() {
            *self.field[i].entry(c).or_insert(0) += 1;
        }
    }

    fn calc_least_common(chars: &HashMap<char, usize>) -> char {
        chars
            .iter()
            .sorted_by_key(|(_, value)| *value)
            .map(|(key, _)| *key)
            .next()
            .unwrap()
    }

    fn calc_most_common(chars: &HashMap<char, usize>) -> char {
        chars
            .iter()
            .sorted_by_key(|(_, value)| *value)
            .map(|(key, _)| *key)
            .last()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(
            part_one(&CharMatrix::build(&vec![
                "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
                "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar"
            ])),
            "easter"
        );
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two(&CharMatrix::build(&vec![
                "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
                "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar"
            ])),
            "advent"
        );
    }
}
