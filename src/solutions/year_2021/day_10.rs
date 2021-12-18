use itertools::Itertools;

pub fn solve(input: &str) {
    let chars = parse(input);
    println!("Part 1: {}", part_one(&chars));
    println!("Part 2: {}", part_two(&chars));
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn part_one(chars: &[String]) -> u64 {
    chars
        .iter()
        .map(|line| find_error(line))
        .map(map_error)
        .sum()
}

fn part_two(chars: &[String]) -> u64 {
    let p: Vec<u64> = chars
        .iter()
        .filter(|line| find_error(line).is_none())
        .map(|line| find_missing(line))
        .map(|added| sum_points_from_missing(&added))
        .sorted()
        .collect();
    p[p.len() / 2]
}

fn find_error(line: &str) -> Option<char> {
    let mut opened = Vec::new();

    for c in line.chars() {
        if let Some(invalid) = match c {
            '(' | '[' | '{' | '<' => {
                opened.push(c);
                None
            }
            ')' | ']' | '}' | '>' => {
                if match_bracket(c) != opened.pop() {
                    Some(c)
                } else {
                    None
                }
            }
            _ => {
                panic!("Unexpected char: {}", c);
            }
        } {
            return Some(invalid);
        }
    }
    None
}

fn find_missing(line: &str) -> String {
    let mut opened = String::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                opened.push(c);
            }
            ')' | ']' | '}' | '>' => {
                opened.pop();
            }
            _ => {
                panic!("Unexpected char: {}", c);
            }
        }
    }

    opened
        .chars()
        .rev()
        .map(|c| match_bracket(c).unwrap())
        .collect()
}

fn sum_points_from_missing(missing_chars: &str) -> u64 {
    missing_chars
        .chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        })
        .fold(0, |acc, x| 5 * acc + x)
}

fn match_bracket(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn map_error(char: Option<char>) -> u64 {
    match char {
        None => 0,
        Some(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unexpected char"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn find_error_works() {
        assert_eq!(find_error(""), None);
        assert_eq!(find_error("[[["), None);
        assert_eq!(find_error("{([(<{}[<>[]}>{[]{[(<()>"), Some('}'));
        assert_eq!(find_error("[[<[([]))<([[{}[[()]]]"), Some(')'));
        assert_eq!(find_error("[{[{({}]{}}([{[{{{}}([]"), Some(']'));
        assert_eq!(find_error("[<(<(<(<{}))><([]([]()"), Some(')'));
        assert_eq!(find_error("<{([([[(<>()){}]>(<<{{"), Some('>'));
    }

    #[test]
    fn find_missing_works() {
        assert_eq!(find_missing("[({(<(())[]>[[{[]{<()<>>"), "}}]])})]");
    }

    #[test]
    fn sum_points_from_missing_works() {
        assert_eq!(sum_points_from_missing("])}>"), 294);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT)), 26397);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(INPUT)), 288957);
    }
}
