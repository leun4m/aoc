pub fn main(input: &str) {
    let depths = parse(input);
    println!("Part 1: {}", part_one(&depths));
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
        .map(|(a, b)| a < b)
        .filter(|x| *x)
        .count()
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
    }
}
