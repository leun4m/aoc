use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let forest = parse(input);
    println!("Part 1: {}", part_one(&forest));
}

type TreeSize = u32;

fn parse(input: &str) -> Vec<Vec<TreeSize>> {
    parser::lines_custom(input, |line| {
        line.chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect_vec()
    })
}

fn part_one(forest: &[Vec<TreeSize>]) -> u32 {
    let mut sum = 0;
    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            if is_visble(forest, x, y) {
                sum += 1;
            }
        }
    }
    sum
}

fn is_visble(forest: &[Vec<TreeSize>], x: usize, y: usize) -> bool {
    let is_visible_left = is_visible_left(forest, x, y);
    let is_visible_top = is_visible_top(forest, x, y);
    let is_visible_right = is_visible_right(forest, x, y);
    let is_visible_bottom = is_visible_bottom(forest, x, y);

    is_visible_left || is_visible_top || is_visible_right || is_visible_bottom
}

fn is_visible_left(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == 0 {
        return true;
    }

    for x0 in 0..x {
        if forest[y][x0] >= forest[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_right(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == forest[y].len() - 1 {
        return true;
    }

    for x0 in (x + 1)..forest[y].len() {
        if forest[y][x0] >= forest[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_top(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }

    for y0 in 0..y {
        if forest[y0][x] >= forest[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_bottom(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if y == forest.len() - 1 {
        return true;
    }

    for y0 in (y + 1)..forest.len() {
        if forest[y0][x] >= forest[y][x] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let example = "30373
25512
65332
33549
35390";

        assert_eq!(21, part_one(&parse(example)));
    }
}
