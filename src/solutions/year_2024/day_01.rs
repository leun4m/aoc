use itertools::Itertools;

pub fn solve(input: &str) {
    let (left, right) = parse(input);
    println!("Part 1: {}", part_one(&left, &right));
    println!("Part 2: {}", part_two(&left, &right));
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .unzip()
}

fn part_one(left: &[i64], right: &[i64]) -> i64 {
    left.iter()
        .sorted()
        .zip(right.iter().sorted())
        .map(|(a, b)| i64::abs(a - b))
        .sum()
}

fn part_two(left: &[i64], right: &[i64]) -> i64 {
    left.iter()
        .map(|&x| x * (right.iter().filter(|&&y| y == x).count() as i64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        let (left, right) = parse(EXAMPLE_INPUT);
        assert_eq!(11, part_one(&left, &right));
    }

    #[test]
    fn test_part_two() {
        let (left, right) = parse(EXAMPLE_INPUT);
        assert_eq!(31, part_two(&left, &right));
    }
}
