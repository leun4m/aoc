use itertools::Itertools;

pub fn solve(input: &str) {
    let opcodes = parse(input);

    println!("Part 1: {}", run_with(&opcodes, 12, 2));

    let (noun, verb) = part_two(&opcodes, 19690720);
    println!("Part 2: {}", 100 * noun + verb);
}

type Opcode = usize;

const OPCODE_ADD: Opcode = 1;
const OPCODE_MUL: Opcode = 2;
const OPCODE_STOP: Opcode = 99;

fn parse(input: &str) -> Vec<Opcode> {
    input
        .split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_two(opcodes_orig: &[Opcode], result: Opcode) -> (Opcode, Opcode) {
    (0..99)
        .cartesian_product(0..99)
        .find(|(noun, verb)| run_with(&opcodes_orig, *noun, *verb) == result)
        .unwrap()
}

fn run_with(opcodes_orig: &[usize], noun: usize, verb: usize) -> usize {
    let mut opcodes = opcodes_orig.to_vec();
    opcodes[1] = noun;
    opcodes[2] = verb;

    run(&mut opcodes)
}

fn run(opcodes: &mut Vec<usize>) -> usize {
    let mut i = 0;
    while opcodes[i] != OPCODE_STOP {
        let a = opcodes[opcodes[i + 1]];
        let b = opcodes[opcodes[i + 2]];
        let i_res = opcodes[i + 3];

        if opcodes[i] == OPCODE_ADD {
            opcodes[i_res] = a + b;
        } else if opcodes[i] == OPCODE_MUL {
            opcodes[i_res] = a * b;
        }

        i += 4;
    }

    opcodes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn test_run() {
        assert_eq!(3500, run(&mut parse(EXAMPLE_INPUT)));
    }
}
