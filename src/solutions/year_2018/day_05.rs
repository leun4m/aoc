use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

pub fn part_one(input: &str) -> usize {
    apply_reactions(input).chars().count()
}

pub fn part_two(input: &str) -> usize {
    input
        .to_ascii_lowercase()
        .chars()
        .unique()
        .map(|c| {
            apply_reactions(&input.replace(|k: char| k.eq_ignore_ascii_case(&c), ""))
                .chars()
                .count()
        })
        .min()
        .unwrap_or(0)
}

pub fn apply_reactions(input: &str) -> String {
    let mut new_string = input.trim().to_string();

    let mut i = 0;

    while !new_string.is_empty() && i < new_string.len() - 1 {
        if has_reaction(
            new_string.chars().nth(i).unwrap(),
            new_string.chars().nth(i + 1).unwrap(),
        ) {
            new_string.replace_range(i..i + 2, "");
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
    use super::{apply_reactions, has_reaction, part_one, part_two};

    #[test]
    fn part_one_works() {
        assert_eq!(part_one("aA"), 0);
        assert_eq!(part_one("abBA"), 0);
        assert_eq!(part_one("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_one("aA"), 0);
        assert_eq!(part_one("abBA"), 0);
        assert_eq!(part_two("dabAcCaCBAcCcaDA"), 4);
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
