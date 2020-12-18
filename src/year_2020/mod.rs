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

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::main(&input),
        2 => day_02::main(&input),
        3 => day_03::main(&input),
        4 => day_04::main(&input),
        5 => day_05::main(&input),
        6 => day_06::main(&input),
        7 => day_07::main(&input),
        8 => day_08::main(&input),
        9 => day_09::main(&input),
        10 => day_10::main(&input),
        11 => day_11::main(&input),
        12 => day_12::main(&input),
        13 => day_13::main(&input),
        15 => day_15::main(&input),
        _ => println!("There exists no implementation for this day."),
    };
}
