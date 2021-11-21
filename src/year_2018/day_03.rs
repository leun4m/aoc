use regex::Regex;
use std::collections::HashSet;

pub fn main(input: &str) {
    let claims = parse(input);
    let claimed_fields = calc_all_claimed(&claims);
    println!("Part 1: {}", part_one(&claimed_fields));
    println!("Part 2: {}", part_two(&claimed_fields));
}

fn parse(input: &str) -> Vec<Claim> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Claim {
    // #1 @ 1,3: 4x4
    let captures = Regex::new(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#)
        .unwrap()
        .captures(line)
        .expect("Looks weird");

    let id = captures[1].parse().unwrap();
    let left = captures[2].parse().unwrap();
    let top = captures[3].parse().unwrap();
    let width = captures[4].parse().unwrap();
    let height = captures[5].parse().unwrap();

    Claim {
        id,
        left,
        top,
        width,
        height,
    }
}

fn calc_all_claimed(claims: &[Claim]) -> Vec<HashSet<(u32, u32)>> {
    claims.iter().map(|claim| calc_claimed(claim)).collect()
}

fn calc_claimed(claim: &Claim) -> HashSet<(u32, u32)> {
    let mut result = HashSet::with_capacity((claim.width * claim.height) as usize);

    for x in claim.left..(claim.left + claim.width) {
        for y in claim.top..(claim.top + claim.height) {
            result.insert((x, y));
        }
    }

    result
}

const MAX_X: u32 = 1000;
const MAX_Y: u32 = 1000;

fn part_one(claimed_fields: &[HashSet<(u32, u32)>]) -> u32 {
    let mut result = 0;

    for x in 0..MAX_X {
        for y in 0..MAX_Y {
            if claimed_fields
                .iter()
                .filter(|m| m.contains(&(x, y)))
                .count()
                > 1
            {
                result += 1;
            }
        }
    }
    result
}

fn part_two(claimed_fields: &[HashSet<(u32, u32)>]) -> u32 {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("#1 @ 1,3: 4x4"),
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4
            }
        );
        assert_eq!(
            parse_line("#2 @ 3,1: 4x4"),
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4
            }
        );
        assert_eq!(
            parse_line("#3 @ 5,5: 2x2"),
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2
            }
        );
    }

    #[test]
    fn calc_claimed_works() {
        assert_eq!(
            HashSet::from([(1, 2)]),
            calc_claimed(&Claim {
                id: 1,
                left: 1,
                top: 2,
                width: 1,
                height: 1,
            })
        );

        assert_eq!(
            HashSet::from([(1, 2), (2, 2), (2, 1), (1, 1)]),
            calc_claimed(&Claim {
                id: 1,
                left: 1,
                top: 1,
                width: 2,
                height: 2,
            })
        );
    }

    #[test]
    fn part_one_works() {
        let claims = vec![
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            },
        ];
        let claimed_fields = calc_all_claimed(&claims);
        assert_eq!(part_one(&claimed_fields), 4);
    }
}
