pub fn main(input: &str) {
    let numbers = parse(input);
    println!("Part 1: {}", part_one(&numbers));
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().filter(|line| !line.is_empty()).map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> i32 {
    line.parse().expect(&format!("Unexpected number: {}", line))
}

fn part_one(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(parse_line("0"), 0);
        assert_eq!(parse_line("+1"), 1);
        assert_eq!(parse_line("-1"), -1);
    }

    #[test]
    fn parse_works() {
        assert_eq!(parse("0\n+1\n-3"), vec![0, 1, -3]);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&[1, 1, 1]), 3);
        assert_eq!(part_one(&[1, 1, -2]), 0);
        assert_eq!(part_one(&[-1, -2, -3]), -6);
    }
}
