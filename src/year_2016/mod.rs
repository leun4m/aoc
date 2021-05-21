mod day_01;

pub fn solve_day(day: u8, input: &str) {
    match day {
        1 => day_01::main(&input),
        _ => println!("There exists no implementation for this day."),
    };
}

