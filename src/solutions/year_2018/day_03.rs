use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

use crate::parser;

pub fn solve(input: &str) {
    let claims = parser::lines_custom(input, parse_line);
    let grid = build_grid(&claims);
    println!("Part 1: {}", part_one(&grid));
    println!("Part 2: {}", part_two(&grid));
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

fn build_grid(claims: &[Claim]) -> Vec<HashSet<u32>> {
    let mut grid: Vec<HashSet<u32>> = Vec::with_capacity(ARRAY_SIZE);

    for _ in 0..ARRAY_SIZE {
        grid.push(HashSet::new());
    }

    for claim in claims {
        for (x, y) in calc_claimed(claim) {
            grid[get_index(x, y)].insert(claim.id);
        }
    }

    grid
}

fn get_index(x: u32, y: u32) -> usize {
    (MAX_X * y + x) as usize
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
const ARRAY_SIZE: usize = (MAX_X * MAX_Y) as usize;

fn part_one(grid: &[HashSet<u32>]) -> usize {
    grid.iter().filter(|x| x.len() > 1).count()
}

fn part_two(grid: &[HashSet<u32>]) -> u32 {
    grid.iter()
        .filter(|ids| ids.len() == 1)
        .flat_map(|ids| ids.iter().copied())
        .unique()
        .find(|id| grid.iter().filter(|g| g.contains(id)).all(|g| g.len() == 1))
        .unwrap()
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
mod tests {
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
        let grid = build_grid(&claims);
        assert_eq!(part_one(&grid), 4);
    }
}
