pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part_one(&parsed));
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|x| parse_numbers(x)).collect()
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

fn part_one(input: &[Vec<u32>]) -> u32 {
    input
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
        assert_eq!(142, part_one(&parse(EXAMPLE_A)));
    }


}
