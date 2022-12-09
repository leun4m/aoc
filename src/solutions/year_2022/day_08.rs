use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let forest = parse(input);
    println!("Part 1: {}", part_one(&forest));
    println!("Part 2: {}", part_two(&forest));
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

fn part_two(forest: &[Vec<TreeSize>]) -> u32 {
    let mut max = 0;

    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            max = std::cmp::max(max, scenic_score(forest, x, y));
        }
    }

    max
}

fn is_visble(forest: &[Vec<TreeSize>], x: usize, y: usize) -> bool {
    is_visible_left(forest, x, y)
        || is_visible_top(forest, x, y)
        || is_visible_right(forest, x, y)
        || is_visible_bottom(forest, x, y)
}

fn is_visible_left(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == 0 {
        return true;
    }

    is_visible_generic(forest, x, y, (x + 1)..forest[y].len(), y..=y)
}

fn is_visible_right(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == forest[y].len() - 1 {
        return true;
    }

    is_visible_generic(forest, x, y, 0..x, y..=y)
}

fn is_visible_top(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }

    is_visible_generic(forest, x, y, x..=x, 0..y)
}

fn is_visible_bottom(forest: &[Vec<u32>], x: usize, y: usize) -> bool {
    if y == forest.len() - 1 {
        return true;
    }

    is_visible_generic(forest, x, y, x..=x, (y + 1)..forest.len())
}

fn is_visible_generic<T, R>(
    forest: &[Vec<TreeSize>],
    x: usize,
    y: usize,
    x_range: T,
    y_range: R,
) -> bool
where
    T: IntoIterator<Item = usize> + Clone,
    R: IntoIterator<Item = usize> + Clone,
{
    x_range.into_iter().all(|x0| {
        y_range
            .clone()
            .into_iter()
            .all(|y0| forest[y0][x0] < forest[y][x])
    })
}

fn scenic_score(forest: &[Vec<TreeSize>], x: usize, y: usize) -> u32 {
    scenic_score_left(forest, x, y)
        * scenic_score_top(forest, x, y)
        * scenic_score_right(forest, x, y)
        * scenic_score_bottom(forest, x, y)
}

fn scenic_score_left(forest: &[Vec<u32>], x: usize, y: usize) -> u32 {
    scenic_score_generic(forest, x, y, (0..x).rev(), y..=y)
}

fn scenic_score_right(forest: &[Vec<u32>], x: usize, y: usize) -> u32 {
    scenic_score_generic(forest, x, y, (x + 1)..forest[y].len(), y..=y)
}

fn scenic_score_top(forest: &[Vec<u32>], x: usize, y: usize) -> u32 {
    scenic_score_generic(forest, x, y, x..=x, (0..y).rev())
}

fn scenic_score_bottom(forest: &[Vec<u32>], x: usize, y: usize) -> u32 {
    scenic_score_generic(forest, x, y, x..=x, (y + 1)..forest.len())
}

fn scenic_score_generic<T, R>(
    forest: &[Vec<TreeSize>],
    x: usize,
    y: usize,
    x_range: T,
    y_range: R,
) -> u32
where
    T: IntoIterator<Item = usize> + Clone,
    R: IntoIterator<Item = usize> + Clone,
{
    let mut score = 0;

    for x0 in x_range {
        for y0 in y_range.clone() {
            score += 1;
            if forest[y0][x0] >= forest[y][x] {
                return score;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_one_works() {
        assert_eq!(21, part_one(&parse(EXAMPLE)));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(8, part_two(&parse(EXAMPLE)));
    }
}
