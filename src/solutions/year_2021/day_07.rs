pub fn solve(input: &str) {
    let heights = parse(input);
    println!("Part 1: {}", part_one(&heights));
    println!("Part 2: {}", part_two(&heights));
}

fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn part_one(heights: &[u32]) -> u32 {
    let fuel_cost = abs_diff;
    find_best_position(heights, fuel_cost)
}

fn part_two(heights: &[u32]) -> u32 {
    let fuel_cost = |a: u32, b: u32| step_costs(abs_diff(a, b));
    find_best_position(heights, fuel_cost)
}

fn abs_diff(a: u32, b: u32) -> u32 {
    b.abs_diff(a)
}

fn step_costs(a: u32) -> u32 {
    match a {
        0 => 0,
        1 => 1,
        x => x + step_costs(x - 1),
    }
}

fn find_best_position<F>(heights: &[u32], fuel_cost: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    assert!(!heights.is_empty(), "heights is empty");

    let min = *heights.iter().min().unwrap();
    let max = *heights.iter().max().unwrap();
    (min..=max)
        .collect::<Vec<u32>>()
        .iter()
        .map(|to| {
            (
                to,
                heights
                    .iter()
                    .map(|from| fuel_cost(*from, *to))
                    .sum::<u32>(),
            )
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    const NUMBERS: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn parse_works() {
        assert_eq!(parse(INPUT), &NUMBERS);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&NUMBERS), 37);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&NUMBERS), 168);
    }
}
