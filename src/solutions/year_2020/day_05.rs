use crate::parser;

pub fn solve(input: &str) {
    let seats: Vec<(u8, u8)> = parser::lines_custom(input, parse_line);
    let max = seats.iter().map(|&x| seat_id(x)).max().unwrap();
    let mut all_seats = Vec::new();

    for r in 1..127 {
        for c in 0..7 {
            all_seats.push((r, c));
        }
    }

    println!("MAX SEAT ID: {}", max);

    let my_seat = all_seats.iter().find(|x| !seats.contains(x)).unwrap();

    println!("Your SEAT ID: {}", seat_id(*my_seat));
}

const FRONT: char = 'F';
const LEFT: char = 'L';

fn parse_line(line: &str) -> (u8, u8) {
    if line.chars().count() != 10 {
        eprintln!("Unexpected length");
        return (0, 0);
    }

    let mut chars = line.chars();

    let mut row_min = 0;
    let mut row_max = 127;
    for _ in 0..7 {
        adjust_range(chars.next().unwrap(), FRONT, &mut row_min, &mut row_max);
    }

    let mut col_min = 0;
    let mut col_max = 7;
    for _ in 0..3 {
        adjust_range(chars.next().unwrap(), LEFT, &mut col_min, &mut col_max);
    }

    (row_min, col_min)
}

fn seat_id((row, col): (u8, u8)) -> u32 {
    (row as u32) * 8 + (col as u32)
}

fn adjust_range(current_char: char, lower: char, min: &mut u8, max: &mut u8) {
    if current_char == lower {
        *max = *min + ((*max - *min) / 2);
    } else {
        *min = *min + ((*max - *min) / 2) + 1;
    }
}
