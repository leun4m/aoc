use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
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
        println!("Progress: {:.2} %", (x + 1) as f64 / CHARS_PASSWORD as f64)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example() {
        assert_eq!("18f47a30", &part_one("abc"));
    }
}
