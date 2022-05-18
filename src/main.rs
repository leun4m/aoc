#[macro_use]
extern crate lazy_static;

mod graph;
mod solutions;
mod util;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "AoC Solver",
    about = "A solver for the Advent of Code riddles.",
    author,
    no_version
)]
struct Opt {
    /// Set year (2 or 4 digits)
    #[structopt(short, long)]
    year: u16,

    /// Set day
    #[structopt(short, long)]
    day: u8,

    /// Input value, requires input-file if not present
    #[structopt(short, long)]
    input: Option<String>,

    /// Input file - if present this will be taken as input
    #[structopt(parse(from_os_str))]
    input_file: Option<PathBuf>,
}
fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let input = if opt.input_file.is_some() {
        read_file(opt.input_file.unwrap())
    } else if opt.input.is_some() {
        opt.input.unwrap()
    } else {
        eprintln!("{}", util::NO_INPUT);
        return;
    };

    let start = Instant::now();

    solutions::solve(opt.year, opt.day, &input);

    let duration = start.elapsed();

    println!("Time elapsed: {}ms", duration.as_millis());
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
