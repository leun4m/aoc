use itertools::Itertools;

pub fn solve(input: &str) {
    let lines = parse(input);
    println!("Part 1: {}", part_one(&lines));
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|x| x.split_whitespace().collect_vec())
        .collect_vec()
}

fn part_one(passphrases: &[Vec<&str>]) -> usize {
    passphrases
        .iter()
        .filter(|passphrase| contains_no_doubles(passphrase))
        .count()
}

fn contains_no_doubles(passphrase: &[&str]) -> bool {
    passphrase.iter().unique().count() == passphrase.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_no_doubles_works() {
        assert!(contains_no_doubles(&vec!["aa", "bb", "cc", "dd", "ee"]));
        assert!(!contains_no_doubles(&vec!["aa", "bb", "cc", "dd", "aa"]));
        assert!(contains_no_doubles(&vec!["aa", "bb", "cc", "dd", "aaa"]));
    }
}
