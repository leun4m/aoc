use crate::util;

pub fn solve(input: &str) {
    let multiples = parse(input);
    let lines = input.lines().collect::<Vec<&str>>();
    println!("Part 1: {}", part_one(&multiples));
    println!("Part 2: {}", part_two(&lines));
}

fn parse(input: &str) -> Vec<Multiples> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Multiples {
    let occurrences = util::count_chars(line);
    Multiples::new(
        occurrences.values().any(|x| x == &2),
        occurrences.values().any(|x| x == &3),
    )
}

fn part_one(multiples: &[Multiples]) -> u32 {
    let (twice, trice) = multiples
        .iter()
        .map(|a| a.multiples())
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .expect("Multiples not empty");
    twice * trice
}

fn part_two(words: &[&str]) -> String {
    for a in words {
        for b in words.iter().skip(1) {
            if let Some(x) = diff_by_one(a, b) {
                return x;
            }
        }
    }

    panic!("Could not find a match!");
}

fn diff_by_one(word_a: &str, word_b: &str) -> Option<String> {
    if word_a.len() != word_b.len() {
        return None;
    }

    let mut common_part = String::new();
    let mut difference = 0;

    for (a_char, b_char) in word_a.chars().zip(word_b.chars()) {
        if a_char == b_char {
            common_part.push(a_char);
        } else {
            difference += 1;
        }

        if difference > 1 {
            break;
        }
    }

    if difference == 1 {
        Some(common_part)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Multiples {
    twice: bool,
    trice: bool,
}

impl Multiples {
    pub fn new(twice: bool, trice: bool) -> Self {
        Multiples { twice, trice }
    }

    pub fn multiples(&self) -> (u32, u32) {
        (
            if self.twice { 1 } else { 0 },
            if self.trice { 1 } else { 0 },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(parse_line("abcdef"), Multiples::new(false, false));
        assert_eq!(parse_line("bababc"), Multiples::new(true, true));
        assert_eq!(parse_line("abbcde"), Multiples::new(true, false));
        assert_eq!(parse_line("abcccd"), Multiples::new(false, true));
        assert_eq!(parse_line("aabcdd"), Multiples::new(true, false));
        assert_eq!(parse_line("abcdee"), Multiples::new(true, false));
        assert_eq!(parse_line("ababab"), Multiples::new(false, true));
    }

    #[test]
    fn part_one_works() {
        let input = vec![
            Multiples::new(false, false),
            Multiples::new(true, true),
            Multiples::new(true, false),
            Multiples::new(false, true),
            Multiples::new(true, false),
            Multiples::new(true, false),
            Multiples::new(false, true),
        ];
        assert_eq!(part_one(&input), 12);
    }

    #[test]
    fn part_two_works() {
        let input = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(part_two(&input), "fgij");
    }

    #[test]
    fn diff_by_one_works() {
        assert_eq!(diff_by_one("", ""), None);
        assert_eq!(diff_by_one("", "b"), None);
        assert_eq!(diff_by_one("a", "ab"), None);
        assert_eq!(diff_by_one("aby", "zbx"), None);

        assert_eq!(diff_by_one("a", "b"), Some(String::new()));
        assert_eq!(diff_by_one("ab", "ac"), Some("a".to_string()));
        assert_eq!(diff_by_one("abf", "acf"), Some("af".to_string()));
        assert_eq!(diff_by_one("cfb", "cfa"), Some("cf".to_string()));
        assert_eq!(diff_by_one("abc", "zbc"), Some("bc".to_string()));
    }
}
