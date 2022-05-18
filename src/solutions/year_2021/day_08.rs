use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let lines = parse(input);
    println!("Part 1: {}", part_one(&lines));
    println!("Part 2: {}", part_two(&lines));
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Line {
    let splitted: Vec<_> = line.split('|').collect();
    Line {
        patterns: splitted[0]
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        outputs: splitted[1]
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line<'a> {
    patterns: [&'a str; 10],
    outputs: [&'a str; 4],
}

fn part_one(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(|line| line.outputs)
        .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part_two(lines: &[Line]) -> u32 {
    lines.iter().map(reconstruct_line).sum()
}

fn reconstruct_line(line: &Line) -> u32 {
    let mapping = find_mapping(line.patterns);

    let numbers: [u32; 4] = line
        .outputs
        .iter()
        .map(|segments| reconstruct_digit(segments, &mapping))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    numbers[0] * 1000 + numbers[1] * 100 + numbers[2] * 10 + numbers[3]
}

fn find_mapping(patterns: [&str; 10]) -> HashMap<char, char> {
    let mut size_patterns: HashMap<usize, Vec<&str>> = HashMap::new();
    for pattern in patterns.iter() {
        (*size_patterns.entry(pattern.len()).or_default()).push(pattern);
    }
    let all: String = patterns
        .iter()
        .flat_map(|pattern| pattern.chars())
        .collect();
    let mut char_count = HashMap::new();
    for c in "abcdefg".chars() {
        let count = all.chars().filter(|&a| a == c).count();
        char_count.insert(c, count);
    }
    let a = find_remaining(
        size_patterns.get(&3).unwrap()[0],
        size_patterns.get(&2).unwrap()[0],
    )[0];

    let c = *occurences(8, &char_count)
        .iter()
        .find(|c| **c != a)
        .unwrap();
    let g = *occurences(7, &char_count)
        .iter()
        .find(|c| {
            size_patterns
                .get(&6)
                .unwrap()
                .iter()
                .all(|pattern| pattern.contains(**c))
        })
        .unwrap();
    let d = *occurences(7, &char_count)
        .iter()
        .find(|c| **c != g)
        .unwrap();

    let mut mappings: HashMap<char, char> = HashMap::new();
    mappings.insert(a, 'a');
    mappings.insert(occurences(6, &char_count)[0], 'b');
    mappings.insert(c, 'c');
    mappings.insert(d, 'd');
    mappings.insert(occurences(4, &char_count)[0], 'e');
    mappings.insert(occurences(9, &char_count)[0], 'f');
    mappings.insert(g, 'g');

    mappings
}

fn occurences(c: usize, count: &HashMap<char, usize>) -> Vec<char> {
    count
        .iter()
        .filter(|(_, v)| **v == c)
        .map(|(k, _)| *k)
        .collect()
}

fn find_remaining(word: &str, to_remove: &str) -> Vec<char> {
    let mut result = Vec::new();

    for c in word.chars() {
        if !to_remove.contains(c) {
            result.push(c);
        }
    }

    result
}

fn reconstruct_digit(segments: &str, mapping: &HashMap<char, char>) -> u32 {
    let digit = segments
        .chars()
        .map(|c| *mapping.get(&c).unwrap())
        .sorted()
        .collect::<String>();

    match_digit(&digit)
}

fn match_digit(digit: &str) -> u32 {
    match digit {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("Unexpected digit: {}", digit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const LINE: Line = Line {
        patterns: [
            "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
        ],
        outputs: ["fdgacbe", "cefdb", "cefbgd", "gcbe"],
    };

    #[test]
    fn parse_works() {
        assert_eq!(
            parse("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), vec![LINE]);
    }
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT)), 26);
    }
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")), 5353);
        assert_eq!(part_two(&parse(INPUT)), 61229);
    }
}
