use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut counter1 = 0;
    let mut counter2 = 0;
    for line in input.lines() {
        if is_nice_old(line) {
            counter1 += 1;
        }
        if is_nice(line) {
            counter2 += 1;
        }
    }
    println!("Part 1: {}", counter1);
    println!("Part 2: {}", counter2);
}

fn is_nice(text: &str) -> bool {
    contains_pair_twice(text) && contains_axa(text)
}

const NULL_CHAR: char = '_';

fn contains_axa(text: &str) -> bool {
    let mut pre_last_char = NULL_CHAR;
    let mut last_char = NULL_CHAR;
    for c in text.chars() {
        if pre_last_char == c {
            return true;
        } else {
            pre_last_char = last_char;
            last_char = c;
        }
    }
    false
}

fn contains_pair_twice(text: &str) -> bool {
    let mut last_char = NULL_CHAR;
    let mut pairs = HashMap::new();
    let mut i = 0;
    for c in text.chars() {
        let pair = format!("{}{}", last_char, c);
        let option = pairs.get(&pair);
        if option.is_some() && *option.unwrap() < (i - 1) {
            return true;
        } else {
            pairs.entry(pair).or_insert(i);
            last_char = c;
            i += 1;
        }
    }
    false
}

fn is_nice_old(text: &str) -> bool {
    !text.contains("ab")
        && !text.contains("cd")
        && !text.contains("pq")
        && !text.contains("xy")
        && contains_double_letter(text)
        && contains_vowels(text, 3)
}

fn contains_double_letter(text: &str) -> bool {
    let mut last_char = '\0';
    for c in text.chars() {
        if c == last_char {
            return true;
        }
        last_char = c;
    }
    false
}

fn contains_vowels(text: &str, min: u8) -> bool {
    let mut counter = 0;
    for c in text.chars() {
        if is_vowel(c) {
            counter += 1;
            if counter >= min {
                return true;
            }
        }
    }
    false
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

#[cfg(test)]
mod tests {
    use super::{contains_axa, contains_pair_twice, is_nice, is_nice_old};

    #[test]
    fn example() {
        assert!(is_nice_old("ugknbfddgicrmopn"));
        assert!(is_nice_old("aaa"));
        assert!(!is_nice_old("jchzalrnumimnmhp"));
        assert!(!is_nice_old("haegwjzuvuyypxyu"));
        assert!(!is_nice_old("dvszwmarrgswjxmb"));
    }

    #[test]
    fn example2() {
        assert!(contains_pair_twice("xyxy"));
        assert!(contains_pair_twice("aabcdefgaa"));
        assert!(!contains_pair_twice("aaa"));
        assert!(!contains_pair_twice("ieodomkazucvgmuy"));
        assert!(contains_pair_twice("jfhobjxionolnouc"));
        assert!(contains_pair_twice("uurcxstgmygtbstg"));

        assert!(contains_axa("xyx"));
        assert!(contains_axa("abcdefeghi"));
        assert!(contains_axa("efe"));
        assert!(contains_axa("aaa"));
        assert!(!contains_axa("uurcxstgmygtbstg"));
        assert!(contains_axa("ieodomkazucvgmuy"));

        assert!(is_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_nice("xxyxx"));
        assert!(!is_nice("uurcxstgmygtbstg"));
        assert!(!is_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn input_file() {
        assert!(!is_nice("uxcplgxnkwbdwhrp"));
        assert!(!is_nice("suerykeptdsutidb"));
        assert!(!is_nice("dmrtgdkaimrrwmej"));
        assert!(!is_nice("ztxhjwllrckhakut"));
        assert!(!is_nice("gdnzurjbbwmgayrg"));

        assert!(!is_nice("gjdzbtrcxwprtery"));
        assert!(!is_nice("fbuqqaatackrvemm"));
        assert!(!is_nice("pcjhsshoveaodyko"));
        assert!(!is_nice("lrpprussbesniilv"));
        assert!(!is_nice("mmsebhtqqjiqrusd"));

        assert!(is_nice("rxexcbwhiywwwwnu"));
    }
}
