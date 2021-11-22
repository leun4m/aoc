use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

type BucketSize = u32;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Bucket {
    id: usize,
    litre: BucketSize,
}

const GOAL: BucketSize = 150;

pub fn main(input: &str) {
    let buckets = parse(input);

    let p = find_fitting(&buckets, GOAL);

    println!("Part 1: {}", part_one(&p));
    println!("Part 2: {}", part_two(&p));
}

fn parse(input: &str) -> Vec<Bucket> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<BucketSize>().unwrap())
        .sorted_by(|a, b| b.cmp(a))
        .zip(0..)
        .map(|(a, b)| Bucket { id: b, litre: a })
        .collect()
}

fn part_one(buckets: &HashSet<Vec<Bucket>>) -> usize {
    buckets.len()
}

fn part_two(buckets: &HashSet<Vec<Bucket>>) -> usize {
    let minimum_amount = buckets.iter().map(|x| x.len()).min().unwrap_or_default();
    buckets.iter().filter(|x| x.len() == minimum_amount).count()
}

fn find_fitting(buckets: &[Bucket], goal: BucketSize) -> HashSet<Vec<Bucket>> {
    find_possibilities(&buckets[0..1], &buckets[1..], goal)
}

fn find_possibilities(a: &[Bucket], b: &[Bucket], goal: BucketSize) -> HashSet<Vec<Bucket>> {
    let mut result = HashSet::new();

    let sum_a = a.iter().map(|x| x.litre).sum::<BucketSize>();
    if b.is_empty() || sum_a >= goal {
        return result;
    }

    let x = sum_a + b[0].litre;

    match x.cmp(&goal) {
        Ordering::Equal => {
            let mut vec = Vec::from(a);
            vec.push(b[0]);
            result.insert(vec);
        }
        Ordering::Less => {
            let mut vec = Vec::from(a);
            vec.push(b[0]);
            result.extend(find_possibilities(&vec, &b[1..], goal));
        }
        Ordering::Greater => {}
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
            HashSet::from([
                vec![Bucket { id: 0, litre: 20 }, Bucket { id: 3, litre: 5 }],
                vec![Bucket { id: 0, litre: 20 }, Bucket { id: 4, litre: 5 }],
                vec![Bucket { id: 1, litre: 15 }, Bucket { id: 2, litre: 10 }],
                vec![
                    Bucket { id: 1, litre: 15 },
                    Bucket { id: 3, litre: 5 },
                    Bucket { id: 4, litre: 5 }
                ],
            ])
        );
    }
}
