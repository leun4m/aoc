use std::collections::HashSet;

pub fn main(input: &str) {
    let lines: Vec<Operation> = input.lines().map(|x| Operation::parse(x)).collect();

    println!("Part One: {}", run(&lines));
    println!("Part Two: {}", find_bug_line(&lines).unwrap());
}

fn find_bug_line(lines: &[Operation]) -> Option<i32> {
    for i in 0..lines.len() {
        let a = run_modified(lines, i);
        if a.is_some() {
            return a;
        }
    }
    None
}

fn run_modified(lines: &[Operation], change_idx: usize) -> Option<i32> {
    let mut current = 0;
    let mut global = 0;
    let mut visited_lines = HashSet::new();

    while !visited_lines.contains(&current) && current != lines.len() {
        visited_lines.insert(current);

        let line;
        if current == change_idx {
            line = match lines[current] {
                Operation::NOP(x) => Operation::JMP(x),
                Operation::JMP(x) => Operation::NOP(x),
                Operation::ACC(x) => Operation::ACC(x),
            }
        } else {
            line = lines[current];
        }

        let result = do_operation(&line, current, global);
        current = result.0;
        global = result.1;
    }

    if current == lines.len() {
        Some(global)
    } else {
        None
    }
}

fn run(lines: &[Operation]) -> i32 {
    let mut current = 0;
    let mut global = 0;
    let mut visited_lines = HashSet::new();

    while !visited_lines.contains(&current) {
        visited_lines.insert(current);
        let result = do_operation(&lines[current], current, global);
        current = result.0;
        global = result.1;
    }

    global
}

fn do_operation(operation: &Operation, current: usize, global: i32) -> (usize, i32) {
    match operation {
        Operation::NOP(_) => (current + 1, global),
        Operation::ACC(x) => (current + 1, global + x),
        Operation::JMP(x) => ((current as i32 + x) as usize, global),
    }
}

#[derive(Copy, Clone)]
enum Operation {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Operation {
    fn parse(operation: &str) -> Self {
        let splits: Vec<&str> = operation.split(' ').collect();
        let value = splits[1].parse::<i32>().unwrap();

        match splits[0] {
            "nop" => Operation::NOP(value),
            "acc" => Operation::ACC(value),
            "jmp" => Operation::JMP(value),
            _ => panic!("unexpected"),
        }
    }
}
