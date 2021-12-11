use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let heights = parse(input);
    println!("Part 1: {}", part_one(&heights));
    println!("Part 2: {}", part_two(&heights));
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

const EMPTY_VEC: Vec<u32> = Vec::new();
const DEFAULT_HEIGHT: u32 = 10;

fn get_height(x: i32, y: i32, heights: &[Vec<u32>]) -> u32 {
    if x < 0 || y < 0 {
        DEFAULT_HEIGHT
    } else {
        *heights
            .get(x as usize)
            .unwrap_or(&EMPTY_VEC)
            .get(y as usize)
            .unwrap_or(&10)
    }
}

fn is_lowest(x: i32, y: i32, heights: &[Vec<u32>]) -> Option<u32> {
    let this = get_height(x, y, heights);
    
    let left = get_height(x - 1, y, heights);
    let right = get_height(x + 1, y, heights);
    let up = get_height(x, y - 1, heights);
    let down = get_height(x, y + 1, heights);

    if this < left && this < right && this < up && this < down {
        Some(this)
    } else {
        None
    }
}

fn find_low_points(heights: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut sum = Vec::new();
    for x in 0..heights.len() {
        for y in 0..heights[x].len() {
            if is_lowest(x as i32, y as i32, heights).is_some() {
                sum.push((x, y));
            }
        }
    }
    sum
}

fn find_basin(x: i32, y: i32, heights: &[Vec<u32>], basins: &mut HashSet<(usize, usize)>) {
    let this = get_height(x, y, heights);

    if this > 8 {
        return;
    }

    let x_usize = x as usize;
    let y_usize = y as usize;

    basins.insert((x_usize, y_usize));

    let left = get_height(x - 1, y, heights);
    let right = get_height(x + 1, y, heights);
    let up = get_height(x, y - 1, heights);
    let down = get_height(x, y + 1, heights);

    if 9 > left && !basins.contains(&(x_usize - 1, y_usize)) {
        find_basin(x - 1, y, heights, basins);
    }
    if 9 > up && !basins.contains(&(x_usize, y_usize - 1)) {
        find_basin(x, y - 1, heights, basins);
    }
    if 9 > right && !basins.contains(&(x_usize + 1, y_usize)) {
        find_basin(x + 1, y, heights, basins);
    }
    if 9 > down && !basins.contains(&(x_usize, y_usize + 1)) {
        find_basin(x, y + 1, heights, basins);
    }
}

fn part_one(heights: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for x in 0..heights.len() {
        for y in 0..heights[x].len() {
            if let Some(b) = is_lowest(x as i32, y as i32, heights) {
                sum += risk_value(b);
            }
        }
    }
    sum
}

fn part_two(heights: &[Vec<u32>]) -> usize {
    let basin_lengths: Vec<usize> = find_low_points(heights)
        .iter()
        .map(|(x, y)| {
            let mut basins = HashSet::new();
            find_basin(*x as i32, *y as i32, heights, &mut basins);
            basins
        })
        .map(|x| x.len())
        .sorted_by(|a, b| Ord::cmp(b, a))
        .collect();

    basin_lengths[0] * basin_lengths[1] * basin_lengths[2]
}

fn risk_value(a: u32) -> u32 {
    a + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn parse_works() {
        let numbers = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(parse("123\n456"), numbers);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT)), 15);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(INPUT)), 1134);
    }
}
