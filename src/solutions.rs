use crate::util;

mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;

pub fn solve(year: u16, day: u8, input: &str) {
    match year {
        15 | 2015 => year_2015::solve_day(day, input),
        16 | 2016 => year_2016::solve_day(day, input),
        17 | 2017 => year_2017::solve_day(day, input),
        18 | 2018 => year_2018::solve_day(day, input),
        19 | 2019 => year_2019::solve_day(day, input),
        20 | 2020 => year_2020::solve_day(day, input),
        21 | 2021 => year_2021::solve_day(day, input),
        22 | 2022 => year_2022::solve_day(day, input),
        23 | 2023 => year_2023::solve_day(day, input),
        _ => println!("{}", util::INVALID_YEAR),
    }
}
