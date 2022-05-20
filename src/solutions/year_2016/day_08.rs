use itertools::Itertools;
use regex::Regex;
use std::cmp::min;

pub fn solve(input: &str) {
    let operations = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect_vec();

    let mut screen = Screen::new();

    for op in operations {
        screen.apply(&op);
    }

    println!("Part 1: {}", screen.lit_pixels());
    println!("Part 2: {}", screen.print());
}

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

lazy_static! {
    static ref ROTATION_REGEX: Regex = Regex::new(r#"=(\d+) by (\d+)"#).unwrap();
    static ref RECT_REGEX: Regex = Regex::new(r#"(\d+)x(\d+)"#).unwrap();
}

struct Screen {
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|col| col.iter().filter(|x| **x).count())
            .sum()
    }

    fn apply(&mut self, operation: &Operation) {
        match operation {
            Operation::Rect(width, height) => self.add_rect(*width, *height),
            Operation::RotateRow(arg) => self.rotate_row(arg),
            Operation::RotateCol(arg) => self.rotate_col(arg),
        }
    }

    fn add_rect(&mut self, width: usize, height: usize) {
        for x in 0..min(height, SCREEN_HEIGHT) {
            for y in 0..min(width, SCREEN_WIDTH) {
                self.pixels[x][y] = true;
            }
        }
    }

    fn rotate_row(&mut self, arg: &RotationArg) {
        self.pixels[arg.id].rotate_right(arg.by % SCREEN_WIDTH);
    }

    fn rotate_col(&mut self, arg: &RotationArg) {
        let mut new_col = [false; SCREEN_HEIGHT];

        for i in 0..new_col.len() {
            new_col[i] = self.pixels[i][arg.id];
        }

        new_col.rotate_right(arg.by % SCREEN_HEIGHT);

        for i in 0..new_col.len() {
            self.pixels[i][arg.id] = new_col[i];
        }
    }

    fn print(&self) -> String {
        "\n".to_owned() + &self
            .pixels
            .iter()
            .map(|col| col.iter().map(|x| if *x { '#' } else { ' ' }).collect())
            .map(|x: String| x + "\n")
            .collect::<String>()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Operation {
    Rect(usize, usize),
    RotateRow(RotationArg),
    RotateCol(RotationArg),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct RotationArg {
    id: usize,
    by: usize,
}

fn parse_line(line: &str) -> Operation {
    if line.starts_with("rotate") {
        let captures = ROTATION_REGEX.captures(line).expect("Invalid line");
        let id = captures[1].parse().unwrap();
        let by = captures[2].parse().unwrap();
        let arg = RotationArg { id, by };

        if line.contains("row") {
            Operation::RotateRow(arg)
        } else {
            Operation::RotateCol(arg)
        }
    } else {
        let captures = RECT_REGEX.captures(line).expect("Invalid line");
        let a = captures[1].parse().unwrap();
        let b = captures[2].parse().unwrap();

        Operation::Rect(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(parse_line("rect 3x2"), Operation::Rect(3, 2));
        assert_eq!(
            parse_line("rotate column x=1 by 1"),
            Operation::RotateCol(RotationArg { id: 1, by: 1 })
        );
        assert_eq!(
            parse_line("rotate row y=0 by 4"),
            Operation::RotateRow(RotationArg { id: 0, by: 4 })
        );
    }

    #[test]
    fn rect_works() {
        let mut screen = Screen::new();
        screen.apply(&Operation::Rect(3, 2));

        assert_eq!(screen.lit_pixels(), 6);
    }

    #[test]
    fn rotate_col_works() {
        let mut screen = Screen::new();
        screen.apply(&Operation::Rect(3, 2));
        screen.apply(&Operation::RotateCol(RotationArg {
            id: 2,
            by: SCREEN_WIDTH,
        }));

        assert_eq!(screen.lit_pixels(), 6);
    }

    #[test]
    fn print_works() {
        let screen = Screen::new();
        let expected: &str = "..................................................
..................................................
..................................................
..................................................
..................................................
..................................................
";
        assert_eq!(&screen.print(), expected);
    }
}
