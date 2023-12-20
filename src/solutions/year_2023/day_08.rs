pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part_one(parsed));
}

fn parse(input: &str) -> (Vec<Instruction>, NodeMap) {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let instructions = parse_instructions(lines[0]);
    let nodes = parse_nodes(lines[1]);

    (instructions, nodes)
}

const SIZE: usize = 26 * 26 * 26;

type NodeMap = [(usize, usize); SIZE];

fn parse_nodes(input: &str) -> NodeMap {
    let mut nodes = [(0, 0); SIZE];

    input.lines().for_each(|line| {
        let replaced = &line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let a: Vec<&str> = replaced
            .split(' ')
            .filter(|x| !x.trim().is_empty())
            .collect();

        nodes[parse_address(a[0])] = (parse_address(a[1]), parse_address(a[2]));
    });

    nodes
}

fn parse_address(addr: &str) -> usize {
    let chars: Vec<char> = addr.chars().collect();
    alpha_to_num(chars[0]) * 26 * 26 + alpha_to_num(chars[1]) * 26 + alpha_to_num(chars[2])
}

fn alpha_to_num(c: char) -> usize {
    c as usize - 'A' as usize
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars()
        .map(|x| match x {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unexpected char {x}"),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

fn part_one((instructions, node_map): (Vec<Instruction>, NodeMap)) -> usize {
    let mut i = 0;
    let mut k = 0;
    let mut steps = 0;

    while k + 1 < SIZE {
        k = match instructions[i] {
            Instruction::Left => node_map[k].0,
            Instruction::Right => node_map[k].1,
        };

        i = (i + 1) % instructions.len();
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse() {
        let (instructions, _) = parse(EXAMPLE_INPUT);
        assert_eq!(instructions, vec![Instruction::Right, Instruction::Left]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(parse(EXAMPLE_INPUT)));
    }
}
