use itertools::Itertools;
use std::collections::HashMap;

pub fn main(input: &str) {
    let words = parse(input);
    println!("Part 1: {}", part_one(&words));
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .collect()
}

fn part_one(words: &[&str]) -> String {
    if words.is_empty() {
        return String::new();
    }
    let first = words[0];
    let mut matrix = CharMatrix::new(first.len());

    for word in words {
        matrix.add(word);
    }

    matrix.calc_best()
}

#[derive(Debug, PartialEq, Eq)]
struct CharMatrix {
    field: Vec<HashMap<char, usize>>,
    size: usize,
}

impl CharMatrix {
    pub fn new(size: usize) -> Self {
        let mut field = Vec::new();
        for _ in 0..size {
            field.push(HashMap::new());
        }

        Self { field, size }
    }

    pub fn add(&mut self, word: &str) {
        if word.len() != self.size {
            panic!("Word has different length")
        }

        for (i, c) in word.chars().enumerate() {
            *self.field[i].entry(c).or_insert(0) += 1;
        }
    }

    pub fn calc_best(&self) -> String {
        self.field.iter().map(|x| calc_most(x)).collect()
    }
}

fn calc_most(chars: &HashMap<char, usize>) -> char {
    chars
        .iter()
        .sorted_by_key(|(_, value)| *value)
        .map(|(key, _)| *key)
        .last()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_works() {
        assert_eq!(
            part_one(&vec![
                "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
                "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar"
            ]),
            "easter"
        );
    }
}
