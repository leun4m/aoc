use std::collections::HashSet;

use crate::parser;

pub fn solve(input: &str) {
    let numbers = parser::parse_numbers(input);
    println!("Part 1: {}", part_one(&numbers));
    println!("Part 2: {}", part_two(&numbers));
}

fn part_one(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn part_two(numbers: &[i32]) -> i32 {
    let mut set = HashSet::from([0]);
    let mut sum = 0;

    loop {
        for i in numbers {
            sum += i;

            if set.contains(&sum) {
                return sum;
            }

            set.insert(sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&[1, 1, 1]), 3);
        assert_eq!(part_one(&[1, 1, -2]), 0);
        assert_eq!(part_one(&[-1, -2, -3]), -6);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&[1, -1]), 0);
        assert_eq!(part_two(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part_two(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part_two(&[7, 7, -2, -7, -4]), 14);
    }
}
