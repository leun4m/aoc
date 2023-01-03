pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

const SEARCH_PREFIX: &str = "00000";
const INDEX_OF_INTEREST: usize = 5;
const CHARS_PASSWORD: usize = 8;
const BLANK_SPACE: char = '_';

fn part_one(input: &str) -> String {
    let trimmed_input = input.trim();

    let mut password = BLANK_SPACE.to_string().repeat(CHARS_PASSWORD);
    let mut start = 0;

    for i in 0..CHARS_PASSWORD {
        let (character, new_start) = find_next(trimmed_input, start);
        password = replace_char(&password, i, character);
        start = new_start;
        print_progress(&password);
    }

    password
}

fn part_two(input: &str) -> String {
    let trimmed_input = input.trim();

    let mut password = BLANK_SPACE.to_string().repeat(CHARS_PASSWORD);
    let mut start = 0;
    let mut positions = Vec::new();

    while password.contains(BLANK_SPACE) {
        let (character, position, new_start) = find_next2(trimmed_input, start, &positions);
        if password.chars().nth(position) == Some(BLANK_SPACE) {
            password = replace_char(&password, position, character);
            start = new_start;
            positions.push(position);
            print_progress(&password);
        }
    }

    password
}

fn print_progress(password: &str) {
    println!(
        "Progress: {:.2}% [{}]",
        (password.chars().filter(|c| *c != BLANK_SPACE).count()) as f64 / CHARS_PASSWORD as f64
            * 100.0,
        password
    );
}

fn find_next(input: &str, start: usize) -> (char, usize) {
    for i in start.. {
        let result = format!("{:x}", md5::compute(format!("{input}{i}")));
        if result.starts_with(SEARCH_PREFIX) {
            return (result.chars().nth(INDEX_OF_INTEREST).unwrap(), i + 1);
        }
    }

    panic!("Could not find anything!");
}

fn find_next2(input: &str, start: usize, positions: &[usize]) -> (char, usize, usize) {
    for i in start.. {
        let result = format!("{:x}", md5::compute(format!("{input}{i}")));
        if result.starts_with(SEARCH_PREFIX) {
            let pos = result
                .chars()
                .nth(INDEX_OF_INTEREST)
                .unwrap()
                .to_digit(16)
                .unwrap() as usize;
            if pos < CHARS_PASSWORD && !positions.contains(&pos) {
                return (
                    result.chars().nth(INDEX_OF_INTEREST + 1).unwrap(),
                    pos,
                    i + 1,
                );
            }
        }
    }

    panic!("Could not find anything!");
}

fn replace_char(input: &str, index: usize, character: char) -> String {
    format!("{}{character}{}", &input[..index], &input[index + 1..])
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
    #[ignore]
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
