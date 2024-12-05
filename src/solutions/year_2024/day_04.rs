use crate::parser;

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

struct Matrix {
    grid: Vec<Vec<char>>,
    size: usize,
}

fn parse(input: &str) -> Matrix {
    let grid = parser::lines_custom(input, |line| line.chars().collect::<Vec<char>>());
    let size = grid.len();

    Matrix { grid, size }
}

fn part_one(matrix: &Matrix) -> usize {
    let mut count = 0;

    // count horizontal -
    for y in 0..matrix.size {
        let s = matrix.grid[y].iter().collect::<String>();
        count += count_twoway(&s);
    }

    // count vertical |
    for x in 0..matrix.size {
        let mut a = String::new();
        for y in 0..matrix.size {
            a.push(matrix.grid[y][x]);
        }
        count += count_twoway(&a);
    }

    // count diagonal /
    for x in 0..matrix.size {
        let mut a = String::new();
        for i in 0..matrix.size - x {
            a.push(matrix.grid[i][x + i]);
        }
        count += count_twoway(&a);
    }
    for y in 1..matrix.size {
        let mut a = String::new();
        for i in 0..matrix.size - y {
            a.push(matrix.grid[y + i][i]);
        }
        count += count_twoway(&a);
    }

    // count diagonal \
    for x in 0..matrix.size {
        let mut a = String::new();
        for i in 0..matrix.size - x {
            a.push(matrix.grid[matrix.size - 1 - i][x + i]);
        }
        count += count_twoway(&a);
    }
    for y in 1..matrix.size {
        let mut a = String::new();
        for i in 0..matrix.size - y {
            a.push(matrix.grid[matrix.size - 1 - y - i][i]);
        }
        count += count_twoway(&a);
    }

    count
}

fn part_two(matrix: &Matrix) -> usize {
    let mut axis = Vec::new();

    let mut findings = Vec::new();
    // count horizontal -
    for y in 0..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for x in 0..matrix.size {
            let c = matrix.grid[y][x];

            state = apply_state(state, c, &mut pos_a, x, y, &mut findings);
        }
    }
    axis.push(findings.clone());

    // count vertical |
    findings.clear();
    for x in 0..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for y in 0..matrix.size {
            let c = matrix.grid[y][x];

            state = apply_state(state, c, &mut pos_a, x, y, &mut findings);
        }
    }
    axis.push(findings.clone());

    // count diagonal /
    findings.clear();
    for x in 0..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for i in 0..matrix.size - x {
            let dx = x + i;
            let dy = i;
            let c = matrix.grid[dy][dx];
            state = apply_state(state, c, &mut pos_a, dx, dy, &mut findings);
        }
    }
    for y in 1..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for i in 0..matrix.size - y {
            let dx = i;
            let dy = y + i;
            let c = matrix.grid[dy][dx];
            state = apply_state(state, c, &mut pos_a, dx, dy, &mut findings);
        }
    }
    axis.push(findings.clone());

    // count diagonal \
    findings.clear();
    for x in 0..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for i in 0..matrix.size - x {
            let dx = x + i;
            let dy = matrix.size - 1 - i;
            let c = matrix.grid[dy][dx];
            state = apply_state(state, c, &mut pos_a, dx, dy, &mut findings);
        }
    }
    for y in 1..matrix.size {
        let mut state = LookingForState::Start;
        let mut pos_a = (0, 0);
        for i in 0..matrix.size - y {
            let dx = i;
            let dy = matrix.size - 1 - y - i;
            let c = matrix.grid[dy][dx];
            state = apply_state(state, c, &mut pos_a, dx, dy, &mut findings);
        }
    }
    axis.push(findings.clone());

    count_duplicates_on_axis(axis)
}

fn apply_state(
    state: LookingForState,
    c: char,
    position_of_a: &mut (usize, usize),
    x: usize,
    y: usize,
    position: &mut Vec<(usize, usize)>,
) -> LookingForState {
    match state {
        LookingForState::Start => {
            if c == 'M' {
                LookingForState::MA
            } else if c == 'S' {
                LookingForState::SA
            } else {
                LookingForState::Start
            }
        }
        LookingForState::MA => {
            if c == 'A' {
                *position_of_a = (x, y);
                LookingForState::MAS
            } else {
                LookingForState::Start
            }
        }
        LookingForState::SA => {
            if c == 'A' {
                *position_of_a = (x, y);
                LookingForState::SAM
            } else {
                LookingForState::Start
            }
        }
        LookingForState::MAS => {
            if c == 'S' {
                position.push(*position_of_a);
            }
            LookingForState::Start
        }
        LookingForState::SAM => {
            if c == 'M' {
                position.push(*position_of_a);
            }
            LookingForState::Start
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
enum LookingForState {
    Start,
    MA,
    SA,
    MAS,
    SAM,
}

fn count_duplicates_on_axis(axis: Vec<Vec<(usize, usize)>>) -> usize {
    let mut count = 0;
    for i in 0..axis.len() {
        for k in i + 1..axis.len() {
            count += find_duplicates(&axis[i], &axis[k]);
        }
    }
    count
}

fn find_duplicates(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    a.iter().filter(|p| b.contains(p)).count()
}

const SEARCH_WORD: &str = "XMAS";

fn count(text: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = text[start..].find(SEARCH_WORD) {
        count += 1;
        start += pos + SEARCH_WORD.len();
    }

    count
}

fn count_twoway(text: &str) -> usize {
    count(text) + count(&reverse_str(text))
}

fn reverse_str(text: &str) -> String {
    text.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(matrix.size, 10);
    }

    #[test]
    fn test_count() {
        assert_eq!(count(""), 0);
        assert_eq!(count("X"), 0);
        assert_eq!(count("ALPHA"), 0);
        assert_eq!(count("XMAS"), 1);
        assert_eq!(count("XMASXMAS"), 2);
        assert_eq!(count("-XMASXMAS---XMAS"), 3);
    }

    #[test]
    fn test_count_twoway() {
        assert_eq!(count_twoway(""), 0);
        assert_eq!(count_twoway("X"), 0);
        assert_eq!(count_twoway("XMAS"), 1);
        assert_eq!(count_twoway("SAMX"), 1);
        assert_eq!(count_twoway("SAMXMAS"), 2);
    }

    #[test]
    fn test_reverse_str() {
        assert_eq!("", reverse_str(""));
        assert_eq!("abc", reverse_str("cba"));
    }

    #[test]
    fn test_part_one() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(part_one(&matrix), 18);
    }

    #[test]
    fn test_part_two() {
        let matrix = parse(EXAMPLE_INPUT);
        assert_eq!(part_two(&matrix), 9);
    }

    #[test]
    fn test_find_duplicates() {
        assert_eq!(find_duplicates(&[], &[]), 0);
        assert_eq!(find_duplicates(&[(1, 1)], &[]), 0);
        assert_eq!(find_duplicates(&[], &[(1, 1)]), 0);
        assert_eq!(find_duplicates(&[(1, 1)], &[(1, 1)]), 1);
        assert_eq!(find_duplicates(&[(1, 1)], &[(2, 2)]), 0);
        assert_eq!(
            find_duplicates(&[(1, 1), (2, 2), (3, 3)], &[(2, 2), (3, 3)]),
            2
        );
    }
}
