mod util;
mod year_2015;
mod year_2016;
mod year_2020;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Advent of Code",
    about = "A solver for the Advent of Code riddles.",
    author
)]
struct Opt {
    /// Set year (2 or 4 digits)
    #[structopt(short, long)]
    year: u16,

    /// Set day
    #[structopt(short, long)]
    day: u8,

    /// Input value
    #[structopt(short, long)]
    input: Option<String>,

    /// Input file - if present this will be taken as input
    #[structopt(parse(from_os_str))]
    input_file: Option<PathBuf>,
}
fn main() {
    let opt = Opt::from_args();

    let input = if opt.input_file.is_some() {
        read_file(opt.input_file.unwrap())
    } else if opt.input.is_some() {
        opt.input.unwrap()
    } else {
        eprintln!("{}", util::NO_INPUT);
        return;
    };

    match opt.year {
        15 | 2015 => year_2015::solve_day(opt.day, &input),
        16 | 2016 => year_2016::solve_day(opt.day, &input),
        20 | 2020 => year_2020::solve_day(opt.day, &input),
        _ => println!("{}", util::INVALID_YEAR),
    }
}

fn read_file(path: PathBuf) -> String {
    let file = File::open(path).expect("File could not be opened!");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Not readable!");
    contents
}
