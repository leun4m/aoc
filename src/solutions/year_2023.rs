use crate::util::INVALID_DAY;

mod day_01;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::solve(input),
        _ => println!("{INVALID_DAY}"),
    };
}