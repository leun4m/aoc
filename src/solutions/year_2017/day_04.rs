use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(input: &str) {
    let passphrases = parse(input);
    println!(
        "Part 1: {}",
        count_valid_passphrases(&passphrases, contains_no_doubles)
    );
    println!(
        "Part 2: {}",
        count_valid_passphrases(&passphrases, contains_no_double_anagrams)
    );
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec()
}

fn count_valid_passphrases<F>(passphrases: &[Vec<&str>], is_valid: F) -> usize
where
    F: Fn(&[&str]) -> bool,
{
    passphrases
        .iter()
        .filter(|passphrase| is_valid(passphrase))
        .count()
}

fn contains_no_doubles(passphrase: &[&str]) -> bool {
    passphrase.iter().unique().count() == passphrase.len()
}

fn contains_no_double_anagrams(passphrase: &[&str]) -> bool {
    let mut chars: HashSet<Vec<char>> = HashSet::new();
    for word in passphrase {
        if !chars.insert(word.chars().sorted().collect()) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_no_doubles_works() {
        assert!(contains_no_doubles(&["aa", "bb", "cc", "dd", "ee"]));
        assert!(!contains_no_doubles(&["aa", "bb", "cc", "dd", "aa"]));
        assert!(contains_no_doubles(&["aa", "bb", "cc", "dd", "aaa"]));
    }

    #[test]
    fn contains_no_double_anagram_works() {
        assert!(contains_no_double_anagrams(&["abcde", "fghij"]));
        assert!(!contains_no_double_anagrams(&["abcde", "xyz", "ecdab"]));
        assert!(contains_no_double_anagrams(&[
            "a", "ab", "abc", "abd", "abf", "abj"
        ]));
        assert!(contains_no_double_anagrams(&[
            "iiii", "oiii", "ooii", "oooi", "oooo"
        ]));
        assert!(!contains_no_double_anagrams(&[
            "oiii", "ioii", "iioi", "iiio"
        ]));
    }
}
