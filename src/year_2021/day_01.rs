pub fn solve(input: &str) {
    let depths = parse(input);
    println!("Part 1: {}", part_one(&depths));
    println!("Part 2: {}", part_two(&depths));
}

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_one(depths: &[u32]) -> usize {
    depths.windows(2).filter(|x| x[0] < x[1]).count()
}

fn part_two(depths: &[u32]) -> usize {
    let window_sums: Vec<_> = depths.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    part_one(&window_sums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&vec![1]), 0);
        assert_eq!(part_one(&vec![1, 0]), 0);
        assert_eq!(part_one(&vec![1, 4]), 1);
        assert_eq!(part_one(&vec![1, 2, 3]), 2);
        assert_eq!(part_one(&vec![1, 2, 3, 2, 5]), 3);

        assert_eq!(
            part_one(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
