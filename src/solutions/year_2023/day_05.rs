use itertools::Itertools;

pub fn solve(input: &str) {
    let (seeds, blocks) = parse(input);
    print!("Part 1: {}", part_one(&seeds, &blocks));
}

fn parse(input: &str) -> (Vec<usize>, Vec<Block>) {
    let blocks = input.split("\n\n").collect_vec();
    println!("{}", blocks[0]);
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
    dest_range_start: usize,
    src_range_start: usize,
    range_length: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Block {
    lines: Vec<InstLine>,
}

fn part_one(seeds: &[usize], blocks: &[Block]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::solutions::year_2023::day_05::parse;

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
        println!("{:?}", parse(EXAMPLE_INPUT))
    }
}
