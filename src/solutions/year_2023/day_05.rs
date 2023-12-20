use itertools::Itertools;

pub fn solve(input: &str) {
    let (seeds, blocks) = parse(input);
    println!("Part 1: {}", part_one(&seeds, &blocks));
    println!("Part 2: {}", part_two(&seeds, &blocks));
}

fn parse(input: &str) -> (Vec<isize>, Vec<Block>) {
    let blocks = input.split("\n\n").collect_vec();
    let seeds = blocks[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect_vec();

    (
        seeds,
        blocks.iter().skip(1).map(|x| parse_block(x)).collect_vec(),
    )
}

fn parse_block(block: &str) -> Block {
    Block {
        lines: block
            .lines()
            .skip(1)
            .map(|x| {
                let numbers = x
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect_vec();
                InstLine {
                    dest_range_start: numbers[0],
                    src_range_start: numbers[1],
                    range_length: numbers[2],
                }
            })
            .collect_vec(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct InstLine {
    dest_range_start: isize,
    src_range_start: isize,
    range_length: isize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Block {
    lines: Vec<InstLine>,
}

fn part_one(seeds: &[isize], blocks: &[Block]) -> isize {
    seeds
        .iter()
        .map(|&seed| convert_seed(blocks, seed))
        .min()
        .unwrap_or_default()
}

// TODO: doesn't get the right result yet...
fn part_two(seeds: &[isize], blocks: &[Block]) -> isize {
    seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let c = chunk.collect_vec();
            let start = *c[0];
            let length = *c[1];
            println!(". {length}");

            (start..start + length)
                .map(|seed| convert_seed(blocks, seed))
                .min()
                .unwrap_or(isize::MAX)
        })
        .min()
        .unwrap_or_default()
}

fn convert_seed(blocks: &[Block], mut seed: isize) -> isize {
    for block in blocks {
        if let Some(element) = block
            .lines
            .iter()
            .find(|x| x.src_range_start <= seed && seed <= x.src_range_start + x.range_length)
        {
            seed += element.dest_range_start - element.src_range_start;
        }
    }

    seed
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse() {
        println!("{:?}", parse(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        let (seeds, blocks) = parse(EXAMPLE_INPUT);
        assert_eq!(35, part_one(&seeds, &blocks));
    }

    #[test]
    fn test_part_two() {
        let (seeds, blocks) = parse(EXAMPLE_INPUT);
        assert_eq!(46, part_two(&seeds, &blocks));
    }
}
