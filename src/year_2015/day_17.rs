use std::collections::HashSet;
use itertools::Itertools;

type BucketSize = u32;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Bucket {
    id: usize,
    litre: BucketSize,
}

const GOAL: BucketSize = 150;

pub fn main(input: &str) {
    let buckets = parse(input);

    println!("Part 1: {}", part_one(&buckets));
    println!("Part 2: {}", part_two(&buckets));
}

fn parse(input: &str) -> Vec<Bucket> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<BucketSize>().unwrap())
        .sorted_by(|a, b| b.cmp(&a))
        .zip(0..)
        .map(|(a, b)| Bucket { id: b, litre: a })
        .collect()
}

fn part_one(buckets: &[Bucket]) -> usize {
    find_fitting(buckets, GOAL)
}

fn part_two(buckets: &[Bucket]) -> u32 {
    0
}

fn find_fitting(buckets: &[Bucket], goal: BucketSize) -> usize {
    find_possibilities(&buckets[0..1], &buckets[1..], goal).len()
}

fn find_possibilities(a: &[Bucket], b: &[Bucket], goal: BucketSize) -> HashSet<Vec<Bucket>> {
    let mut result = HashSet::new();

    let sum_a = a.iter().map(|x| x.litre).sum::<BucketSize>();
    if b.is_empty() || sum_a >= goal {
        return result;
    }

    let x = sum_a + b[0].litre;
    if x == goal {
        let mut vec = Vec::from(a);
        vec.push(b[0]);
        result.insert(vec);
    } else if x < goal {
        let mut vec = Vec::from(a);
        vec.push(b[0]);
        result.extend(find_possibilities(&vec, &b[1..], goal));
    }

    result.extend(find_possibilities(a, &b[1..], goal));

    let mut vec = Vec::from(&a[1..]);
    vec.push(b[0]);
    result.extend(find_possibilities(&vec, &b[1..], goal));

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_fitting_works() {
        let a = parse(
            "
20
15
10
5
5",
        );
        assert_eq!(
            find_fitting(&a, 25),
            4
        );
    }
}
