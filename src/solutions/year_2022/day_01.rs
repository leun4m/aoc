use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let calories = parse(input).iter().map(|x| x.iter().sum()).collect_vec();
    println!("Part 1: {}", part_one(&calories));
    println!("Part 2: {}", part_two(&calories));
}

type Calories = u32;

fn parse(input: &str) -> Vec<Vec<Calories>> {
    input
        .split("\n\n")
        .map(parser::lines_as_numbers)
        .collect_vec()
}

fn part_one(input: &[Calories]) -> Calories {
    input.iter().sorted().rev().take(1).sum()
}

fn part_two(input: &[Calories]) -> Calories {
    input.iter().sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn parse_works() {
        let input = "
        1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        let output = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(parse(input), output);
    }

    #[test]
    fn part_one_works() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn part_two_works() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(part_two(&input), 45000);
    }
}
