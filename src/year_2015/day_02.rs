use std::cmp::{max, min};
use std::str::FromStr;

pub fn solve(input: &str) {
    let mut paper = 0;
    let mut ribbon = 0;
    for line in input.lines() {
        let dimensions = parse_line(line);
        paper += calc_paper(dimensions) + extra_paper(dimensions);
        ribbon += calc_ribbon(dimensions);
    }
    println!("Paper:  {}", paper);
    println!("Ribbon: {}", ribbon);
}

fn parse_line(line: &str) -> (u32, u32, u32) {
    let numbers: Vec<&str> = line.split('x').collect();
    if numbers.len() != 3 {
        panic!("There are {} numbers in line: {}", numbers.len(), line)
    } else {
        let length = u32::from_str(numbers.get(0).unwrap()).expect("Could not parse!");
        let width = u32::from_str(numbers.get(1).unwrap()).expect("Could not parse!");
        let height = u32::from_str(numbers.get(2).unwrap()).expect("Could not parse!");

        (length, width, height)
    }
}

fn calc_paper((l, w, h): (u32, u32, u32)) -> u32 {
    (2 * l * w) + (2 * w * h) + (2 * h * l)
}

fn extra_paper((a, b, c): (u32, u32, u32)) -> u32 {
    min(a, b) * min(max(a, b), c)
}

fn calc_ribbon((a, b, c): (u32, u32, u32)) -> u32 {
    (2 * min(a, b) + 2 * min(max(a, b), c)) + (a * b * c)
}

#[cfg(test)]
mod tests {
    use crate::year_2015::day_02::{calc_paper, extra_paper, parse_line};

    #[test]
    fn example() {
        assert_eq!((2, 3, 4), parse_line("2x3x4"));
        assert_eq!((1, 1, 10), parse_line("1x1x10"));

        assert_eq!(52, calc_paper((2, 3, 4)));
        assert_eq!(42, calc_paper((1, 1, 10)));

        assert_eq!(6, extra_paper((2, 3, 4)));
        assert_eq!(1, extra_paper((1, 1, 8)));

        assert_eq!(2, extra_paper((1, 2, 3)));
        assert_eq!(2, extra_paper((1, 3, 2)));
        assert_eq!(2, extra_paper((2, 3, 1)));
        assert_eq!(2, extra_paper((2, 1, 3)));
        assert_eq!(2, extra_paper((3, 1, 2)));
        assert_eq!(2, extra_paper((3, 2, 1)));
    }
}
