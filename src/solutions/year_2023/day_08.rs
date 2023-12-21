pub fn solve(input: &str) {
    let (i, n) = parse(input);
    println!("Part 1: {}", part_one(&i, &n));
    println!("Part 2: {}", part_two(&i, &n));
}

fn parse(input: &str) -> (Vec<Instruction>, NodeMap) {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let instructions = parse_instructions(lines[0]);
    let nodes = parse_nodes(lines[1]);

    (instructions, nodes)
}

const LETTERS: usize = 'Z' as usize - 'A' as usize + 1;
const SIZE: usize = LETTERS * LETTERS * LETTERS;

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
    alpha_to_num(chars[0]) * LETTERS * LETTERS
        + alpha_to_num(chars[1]) * LETTERS
        + alpha_to_num(chars[2])
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

fn part_one(instructions: &[Instruction], nodes: &NodeMap) -> usize {
    let mut i = 0;
    let mut k = 0;
    let mut steps = 0;

    while k + 1 < SIZE {
        k = match instructions[i] {
            Instruction::Left => nodes[k].0,
            Instruction::Right => nodes[k].1,
        };

        i = (i + 1) % instructions.len();
        steps += 1;
    }

    steps
}

fn part_two(instructions: &[Instruction], nodes: &NodeMap) -> usize {
    let indices: Vec<usize> = (0..SIZE)
        .filter(|&x| nodes[x] != (0, 0))
        .filter(|x| x % LETTERS == alpha_to_num('A'))
        .collect();
    let mut steps_taken = Vec::new();
    
    for index in indices {
        let mut steps = 0;
        let mut i = 0;
        let mut k = index;

        while k % LETTERS != alpha_to_num('Z') {
            k = match instructions[i] {
                Instruction::Left => nodes[k].0,
                Instruction::Right => nodes[k].1,
            };
    
            i = (i + 1) % instructions.len();
            steps += 1;
        }

        steps_taken.push(steps);
    }

    lcm(&steps_taken)
}

fn lcm(numbers: &[usize]) -> usize {
    let mut result = 1;
    let mut a = result;

    for i in 0..numbers.len() {
        let b = numbers[i];
        result = (a * b) / gcd(a, b);
        a = result;
    }

    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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

    const EXAMPLE_INPUT2: &str = "LR

DDA = (DDB, XXX)
DDB = (XXX, DDZ)
DDZ = (DDB, XXX)
EEA = (EEB, XXX)
EEB = (EEC, EEC)
EEC = (EEZ, EEZ)
EEZ = (EEB, EEB)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse() {
        let (instructions, _) = parse(EXAMPLE_INPUT);
        assert_eq!(instructions, vec![Instruction::Right, Instruction::Left]);
    }

    #[test]
    fn test_part_one() {
        let (instructions, nodes) = parse(EXAMPLE_INPUT);
        assert_eq!(2, part_one(&instructions, &nodes));
    }

    #[test]
    fn test_part_two() {
        let (instructions, nodes) = parse(EXAMPLE_INPUT2);
        assert_eq!(6, part_two(&instructions, &nodes));
    }
}
