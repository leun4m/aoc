use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
}

pub fn part_one(input: &str) -> usize {
    apply_reactions(input).chars().count()
}

pub fn apply_reactions(input: &str) -> String {
    let mut new_string = input.trim().to_string();

    let mut chars = input.chars().collect_vec();
    let mut i = 0;

    while !chars.is_empty() && i < chars.len() - 1 {
        if has_reaction(chars[i], chars[i + 1]) {
            new_string.remove(i);
            new_string.remove(i);
            chars = new_string.chars().collect_vec();
            i = if i < 2 { 0 } else { i - 1 };
        } else {
            i += 1;
        }
    }
    
    new_string
}

fn has_reaction(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

#[cfg(test)]
mod tests {
    use super::{apply_reactions, has_reaction, part_one};

    #[test]
    fn part_one_works() {
        assert_eq!(part_one("aA"), 0);
        assert_eq!(part_one("abBA"), 0);
        assert_eq!(part_one("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn apply_reactions_works() {
        assert_eq!(&apply_reactions("aA"), "");
        assert_eq!(&apply_reactions("abBA"), "");
        assert_eq!(&apply_reactions("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn has_reaction_works() {
        assert!(has_reaction('a', 'A'));
        assert!(has_reaction('C', 'c'));

        assert!(!has_reaction('A', 'A'));
        assert!(!has_reaction('b', 'b'));
        assert!(!has_reaction('a', 'b'));
        assert!(!has_reaction('a', 'B'));
        assert!(!has_reaction('A', 'b'));
        assert!(!has_reaction('A', 'B'));
    }
}
