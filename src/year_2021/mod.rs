use crate::util::INVALID_DAY;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_09;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::solve(input),
        2 => day_02::solve(input),
        3 => day_03::solve(input),
        4 => day_04::solve(input),
        5 => day_05::solve(input),
        6 => day_06::solve(input),
        7 => day_07::solve(input),
        9 => day_09::solve(input),
        _ => println!("{}", INVALID_DAY),
    };
}
