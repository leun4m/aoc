use std::collections::HashSet;

pub fn main(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    println!("Part One: {}", run(&lines));
    println!("Part Two: {}", find_bug_line(&lines).unwrap());
}

fn find_bug_line(lines: &Vec<&str>) -> Option<i32> {
    for i in 0..lines.len() {
        let a = run_modified(lines, i);
        if a.is_some() {
            return a;
        }
    }
    None
}

fn run_modified(lines: &Vec<&str>, change_idx: usize) -> Option<i32> {
    let mut current = 0;
    let mut global = 0;
    let mut visited_lines = HashSet::new();

    while !visited_lines.contains(&current) && current != lines.len() {
        visited_lines.insert(current);

        let line = if current == change_idx {
            if lines[current].starts_with("nop") {
                lines[current].replace("nop", "jmp")
            } else {
                lines[current].replace("jmp", "nop")
            }
        } else {
            lines[current].to_string()
        };

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

fn run(lines: &Vec<&str>) -> i32 {
    let mut current = 0;
    let mut global = 0;
    let mut visited_lines = HashSet::new();

    while !visited_lines.contains(&current) {
        visited_lines.insert(current);
        let result = do_operation(lines[current], current, global);
        current = result.0;
        global = result.1;
    }

    global
}

fn do_operation(operation: &str, current: usize, mut global: i32) -> (usize, i32) {
    if operation.starts_with("nop") {
        (current + 1, global)
    } else if operation.starts_with("acc") {
        global += operation.replace("acc ", "").parse::<i32>().unwrap();
        (current + 1, global)
    } else if operation.starts_with("jmp") {
        (
            (current as i32 + operation.replace("jmp ", "").parse::<i32>().unwrap()) as usize,
            global,
        )
    } else {
        panic!("unexpected")
    }
}
