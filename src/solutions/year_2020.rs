use crate::util::INVALID_DAY;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_15;
mod day_16;
mod day_17;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::solve(input),
        2 => day_02::solve(input),
        3 => day_03::solve(input),
        4 => day_04::solve(input),
        5 => day_05::solve(input),
        6 => day_06::solve(input),
        7 => day_07::solve(input),
        8 => day_08::solve(input),
        9 => day_09::solve(input),
        10 => day_10::solve(input),
        11 => day_11::solve(input),
        12 => day_12::solve(input),
        13 => day_13::solve(input),
        15 => day_15::solve(input),
        16 => day_16::solve(input),
        17 => day_17::solve(input),
        _ => println!("{INVALID_DAY}"),
    }
}
