use std::collections::HashSet;

pub fn solve(input: &str) {
    let (paper, fold_instructions) = parse(input);
    println!("Part 1: {}", part_one(&paper, &fold_instructions));
    println!("Part 2:\n{}", part_two(&paper, &fold_instructions));
}

type Point = (usize, usize);
type Paper = HashSet<Point>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn parse(input: &str) -> (Paper, Vec<FoldInstruction>) {
    let paper = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("fold"))
        .map(parse_pair)
        .collect();

    let instructions = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| line.starts_with("fold"))
        .map(parse_fold)
        .collect();

    (paper, instructions)
}

fn parse_pair(input: &str) -> Point {
    let nums: Vec<_> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

fn parse_fold(input: &str) -> FoldInstruction {
    let trimmed = input.replace("fold along ", "");
    let fold: Vec<_> = trimmed.split('=').collect();
    let value = fold[1].parse().unwrap();
    match fold.first() {
        Some(&"x") => FoldInstruction::X(value),
        Some(&"y") => FoldInstruction::Y(value),
        Some(z) => panic!("Unexpected axis: {}", z),
        None => panic!("Unexpected value"),
    }
}

fn part_one(paper: &Paper, instructions: &[FoldInstruction]) -> usize {
    let instruction = instructions[0];
    fold(paper, instruction).len()
}

fn part_two(paper: &Paper, instructions: &[FoldInstruction]) -> String {
    let mut paper_copy = paper.clone();

    for instruction in instructions.iter().copied() {
        paper_copy = fold(&paper_copy, instruction);
    }

    paper_to_string(&paper_copy)
}

fn fold(paper: &Paper, instruction: FoldInstruction) -> Paper {
    match instruction {
        FoldInstruction::X(fold) => paper
            .iter()
            .map(|&(x, y)| {
                if fold <= x {
                    let new_x = fold - (x - fold);
                    (new_x, y)
                } else {
                    (x, y)
                }
            })
            .collect(),
        FoldInstruction::Y(fold) => paper
            .iter()
            .map(|&(x, y)| {
                if fold <= y {
                    let new_y = fold - (y - fold);
                    (x, new_y)
                } else {
                    (x, y)
                }
            })
            .collect(),
    }
}

fn paper_to_string(paper: &Paper) -> String {
    let x_len = *paper.iter().map(|(x, _)| x).max().unwrap() + 1;
    let y_len = *paper.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut result = String::with_capacity((x_len + 1) * y_len);

    for y in 0..y_len {
        for x in 0..x_len {
            let c = if paper.contains(&(x, y)) { '#' } else { ' ' };
            result.push(c);
        }
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    #[test]
    fn parse_works() {
        assert_eq!(
            parse(INPUT).1,
            vec![FoldInstruction::Y(7), FoldInstruction::X(5)]
        );
    }

    #[test]
    fn part_one_works() {
        let (paper, instructions) = parse(INPUT);
        assert_eq!(part_one(&paper, &instructions), 17);
    }
}
