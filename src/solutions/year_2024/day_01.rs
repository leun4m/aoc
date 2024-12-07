pub fn solve(input: &str) {
    let (left, right) = parse(input);
    println!(
        "Part 1: {}",
        part_one(&mut left.clone(), &mut right.clone())
    );
    println!("Part 2: {}", part_two(&left, &right));
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if let Some((l, r)) = line.split_once("   ") {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    }

    (left, right)
}

fn part_one(left: &mut Vec<i64>, right: &mut Vec<i64>) -> i64 {
    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        sum += i64::abs(left[i] - right[i]);
    }

    sum
}

fn part_two(left: &[i64], right: &[i64]) -> i64 {
    let mut sum = 0;

    for elem in left {
        let mut factor = 0;

        for j in 0..right.len() {
            if *elem == right[j] {
                factor += 1;
            }
        }

        sum += *elem * factor;
    }

    sum
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
        assert_eq!(11, part_one(&mut left.clone(), &mut right.clone()));
    }

    #[test]
    fn test_part_two() {
        let (left, right) = parse(EXAMPLE_INPUT);
        assert_eq!(31, part_two(&left, &right));
    }
}
