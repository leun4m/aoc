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
    depths
        .iter()
        .zip(&depths[1..])
        .filter(|(a, b)| a < b)
        .count()
}

fn part_two(depths: &[u32]) -> usize {
    let window_sums: Vec<u32> = depths
        .iter()
        .zip(&depths[1..])
        .zip(&depths[2..])
        .map(|((a, b), c)| a + b + c)
        .collect();
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
