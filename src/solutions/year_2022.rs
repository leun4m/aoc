use crate::util::INVALID_DAY;

mod day_01;
mod day_02;
mod day_03;
mod day_08;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::solve(input),
        2 => day_02::solve(input),
        3 => day_03::solve(input),
        8 => day_08::solve(input),
        _ => println!("{INVALID_DAY}"),
    };
}
