use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

type BucketSize = u32;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Bucket {
    id: usize,
    size: BucketSize,
}

const GOAL: BucketSize = 150;

pub fn solve(input: &str) {
    let buckets = parse(input);
    let arrangements = find_arrangements(&buckets, GOAL);

    println!("Part 1: {}", part_one(&arrangements));
    println!("Part 2: {}", part_two(&arrangements));
}

fn parse(input: &str) -> Vec<Bucket> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<BucketSize>().unwrap())
        .sorted_by(|a, b| b.cmp(a))
        .zip(0..)
        .map(|(a, b)| Bucket { id: b, size: a })
        .collect()
}

fn part_one(arrangements: &HashSet<Vec<Bucket>>) -> usize {
    arrangements.len()
}

fn part_two(arrangements: &HashSet<Vec<Bucket>>) -> usize {
    let minimum_amount = arrangements
        .iter()
        .map(|x| x.len())
        .min()
        .unwrap_or_default();
    arrangements
        .iter()
        .filter(|x| x.len() == minimum_amount)
        .count()
}

fn find_arrangements(buckets: &[Bucket], goal: BucketSize) -> HashSet<Vec<Bucket>> {
    find_sub_arrangements(&buckets[0..1], &buckets[1..], goal)
}

fn find_sub_arrangements(a: &[Bucket], b: &[Bucket], goal: BucketSize) -> HashSet<Vec<Bucket>> {
    let mut result = HashSet::new();

    let a_sum = a.iter().map(|x| x.size).sum::<BucketSize>();

    if b.is_empty() || a_sum >= goal {
        return result;
    }

    match (a_sum + b[0].size).cmp(&goal) {
        Ordering::Equal => {
            let mut vec = Vec::from(a);
            vec.push(b[0]);
            result.insert(vec);
        }
        Ordering::Less => {
            let mut vec = Vec::from(a);
            vec.push(b[0]);
            result.extend(find_sub_arrangements(&vec, &b[1..], goal));
        }
        Ordering::Greater => {}
    }

    result.extend(find_sub_arrangements(a, &b[1..], goal));

    let mut vec = Vec::from(&a[1..]);
    vec.push(b[0]);
    result.extend(find_sub_arrangements(&vec, &b[1..], goal));

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
            find_arrangements(&a, 25),
            HashSet::from([
                vec![Bucket { id: 0, size: 20 }, Bucket { id: 3, size: 5 }],
                vec![Bucket { id: 0, size: 20 }, Bucket { id: 4, size: 5 }],
                vec![Bucket { id: 1, size: 15 }, Bucket { id: 2, size: 10 }],
                vec![
                    Bucket { id: 1, size: 15 },
                    Bucket { id: 3, size: 5 },
                    Bucket { id: 4, size: 5 }
                ],
            ])
        );
    }
}
