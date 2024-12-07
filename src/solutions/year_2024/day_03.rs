use regex::Regex;

pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum()
}

fn part_two(input: &str) -> i32 {
    const TOKEN_DO: &str = "do()";
    const TOKEN_DONT: &str = "don't()";

    let mut sum = 0;
    let mut is_do = true;

    let regex = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut i = 0;
    while i < input.len() {
        let remaining = &input[i..];

        i += if remaining.starts_with(TOKEN_DO) {
            is_do = true;
            TOKEN_DO.len()
        } else if remaining.starts_with(TOKEN_DONT) {
            is_do = false;
            TOKEN_DONT.len()
        } else if let Some(captures) = regex.captures(remaining) {
            if is_do {
                let val1: i32 = captures[1].parse().unwrap();
                let val2: i32 = captures[2].parse().unwrap();
                sum += val1 * val2;
            }
            captures[0].len()
        } else {
            1
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(161, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(48, part_two(EXAMPLE_INPUT2));
    }
}
