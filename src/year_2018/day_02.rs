use crate::util;

pub fn main(input: &str) {
    let multiples = parse(input);
    println!("Part 1: {}", part_one(&multiples));
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
mod test {
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
}
