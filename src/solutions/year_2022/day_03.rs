use crate::parser;

pub fn solve(input: &str) {
    let rucksacks = parser::lines_as_strings(input);
    println!("Part 1: {}", part_one(&rucksacks));
    println!("Part 2: {}", part_two(&rucksacks));
}

fn part_one(rucksacks: &[&str]) -> u32 {
    rucksacks
        .iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(find_common_element_in)
        .map(priority)
        .sum()
}

fn part_two(rucksacks: &[&str]) -> u32 {
    rucksacks
        .chunks(3)
        .collect::<Vec<_>>()
        .into_iter()
        .map(find_common_element_in2)
        .map(priority)
        .sum()
}

fn find_common_element_in(compartments: (&str, &str)) -> char {
    compartments
        .0
        .chars()
        .find(|elem| compartments.1.contains(*elem))
        .unwrap()
}

fn find_common_element_in2(compartments: &[&str]) -> char {
    compartments[0]
        .chars()
        .find(|elem| compartments[1].contains(*elem) && compartments[2].contains(*elem))
        .unwrap()
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32 - 'a' as u32) + 1
    } else {
        (c as u32 - 'A' as u32) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_RUCKSACK: [&str; 6] = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn priority_works() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn part_one_works() {
        assert_eq!(157, part_one(&EXAMPLE_RUCKSACK));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(70, part_two(&EXAMPLE_RUCKSACK));
    }
}
