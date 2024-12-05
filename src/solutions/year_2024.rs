use crate::util::INVALID_DAY;

mod day_04;

pub fn solve_day(day: u8, input: &str) {
    match day {
        4 => day_04::solve(input),
        _ => println!("{INVALID_DAY}"),
    };
}
