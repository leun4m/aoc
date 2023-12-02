pub fn solve(input: &str) {
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part_one(input: &str) -> u32 {
    calculate_sum(&parse(input))
}

fn part_two(input: &str) -> u32 {
    calculate_sum(&parse(&pre_parse(input)))
}

fn calculate_sum(numbers: &[Vec<u32>]) -> u32 {
    numbers
        .iter()
        .map(|x| {
            String::from_iter([
                x.first().unwrap().to_string(),
                x.last().unwrap().to_string(),
            ])
        })
        .map(|x| x.parse::<u32>().unwrap())
        .sum()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(char::is_ascii_digit)
        .filter_map(|x| x.to_digit(10))
        .collect()
}

fn pre_parse(input: &str) -> String {
    input.lines().map(pre_parse_line).collect()
}

fn pre_parse_line(input: &str) -> String {
    let mut i = 0;
    let mut new_line = input.to_string();

    while i < new_line.len() {
        let mut temp = new_line[..i].to_string();
        let mut replace = new_line[i..].to_string();

        if replace_first_digit(&mut replace) {
            i += 1;
        }

        temp.push_str(&replace);
        new_line = temp;

        i += 1;
    }

    new_line.push('\n');
    new_line
}

fn replace_first_digit(input: &mut String) -> bool {
    for (digit_spelled, digit_value) in DIGITS {
        if input.starts_with(digit_spelled) {
            let mut replacement = digit_value.to_string();
            replacement.push_str(digit_spelled);

            let temp = input.replacen(digit_spelled, &replacement, 1);
            *input = temp;

            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_B: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 8], vec![1, 2, 3, 4, 5], vec![7]],
            parse(EXAMPLE_A)
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(142, part_one(EXAMPLE_A));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(281, part_two(EXAMPLE_B));
    }

    #[test]
    fn test_replace_nums() {
        assert_eq!("1one\n", pre_parse_line("one"));
        assert_eq!("8eigh2two\n", pre_parse_line("eightwo"));
    }
}
