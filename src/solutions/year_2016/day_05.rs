use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

const SEARCH_PREFIX: &str = "00000";
const INDEX_OF_INTEREST: usize = 5;
const CHARS_PASSWORD: usize = 8;

fn part_one(input: &str) -> String {
    let trimmed_input = input.trim();

    let mut password = String::with_capacity(CHARS_PASSWORD);
    let mut start = 0;

    for x in 0..CHARS_PASSWORD {
        let (character, new_start) = find_next(trimmed_input, start);
        password.push(character);
        start = new_start;
        println!("Progress: {:.2} %", (x + 1) as f64 / CHARS_PASSWORD as f64 * 100.0)
    }

    password
}

fn part_two(input: &str) -> String {
    let trimmed_input = input.trim();

    let mut password = "_".repeat(CHARS_PASSWORD);
    let mut start = 0;
    let mut positions = Vec::new();

    while password.contains('_') {
        let (character, position, new_start) = find_next2(trimmed_input, start, &positions);
        if password.chars().nth(position) == Some('_') {
            password = replace_char(&password, position, character);
            start = new_start;
            positions.push(position);
            println!("Progress: {:.2}% {}", (password.chars().filter(|c| *c != '_').count() + 1) as f64 / CHARS_PASSWORD as f64 * 100.0, password);
        }
    }

    password
}

fn find_next(input: &str, start: usize) -> (char, usize) {
    let mut md5 = Md5::new();
    md5.input_str(input);

    for i in start..usize::MAX {
        md5.input_str(&format!("{}{}", input, i));

        let result = md5.result_str();
        if result.starts_with(SEARCH_PREFIX) {
            return (result.chars().nth(INDEX_OF_INTEREST).unwrap(), i);
        }

        md5.reset();
    }

    panic!("Could not find anything!");
}

fn find_next2(input: &str, start: usize, positions: &[usize]) -> (char, usize, usize) {
    let mut md5 = Md5::new();
    md5.input_str(input);

    for i in start..usize::MAX {
        md5.input_str(&format!("{}{}", input, i));

        let result = md5.result_str();
        if result.starts_with(SEARCH_PREFIX) {
            let pos = result.chars().nth(INDEX_OF_INTEREST).unwrap().to_digit(16).unwrap() as usize;
            if pos < CHARS_PASSWORD && !positions.contains(&pos) {
                return (result.chars().nth(INDEX_OF_INTEREST + 1).unwrap(), pos, i);
            }
        }

        md5.reset();
    }

    panic!("Could not find anything!");
}

fn replace_char(input: &str, index: usize, character: char) -> String {
    format!("{}{}{}", &input[..index], character, &input[index + 1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part_one_works() {
        assert_eq!("18f47a30", &part_one("abc"));
    }

    #[test]
    // #[ignore]
    fn part_two_works() {
        assert_eq!("05ace8e3", &part_two("abc"));
    }

    #[test]
    fn replace_char_works() {
        assert_eq!("x___", &replace_char("____", 0, 'x'));
        assert_eq!("_x__", &replace_char("____", 1, 'x'));
        assert_eq!("__x_", &replace_char("____", 2, 'x'));
        assert_eq!("___x", &replace_char("____", 3, 'x'));
    }
}
