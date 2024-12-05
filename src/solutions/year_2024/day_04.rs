use crate::parser;

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
}

struct Matrix {
    grid: Vec<Vec<char>>,
    size: usize,
}

fn parse(input: &str) -> Matrix {
    let grid = parser::lines_custom(input, |line| {
        line.chars().collect::<Vec<char>>()
    });
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
}
