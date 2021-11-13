use crate::util::INVALID_DAY;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::main(input),
        2 => day_02::main(input),
        3 => day_03::main(input),
        4 => day_04::main(input),
        _ => println!("{}", INVALID_DAY),
    };
}
