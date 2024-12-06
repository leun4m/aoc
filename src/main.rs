#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::similar_names
)]

#[macro_use]
extern crate lazy_static;

mod euclidic;
mod graph;
mod parser;
mod solutions;
mod util;

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(
    author,
    about,
    help_template = "
{name}
{author}
{about}

{usage-heading}
{usage}

{all-args}
"
)]
struct Opt {
    /// Set year (2 or 4 digits)
    #[arg(short, long)]
    year: u16,

    /// Set day
    #[arg(short, long)]
    day: u8,

    /// Input value, requires input-file if not present
    #[arg(short, long)]
    input: Option<String>,

    /// Input file - if present this will be taken as input
    #[arg()]
    input_file: Option<PathBuf>,
}
fn main() {
    env_logger::init();

    let opt = Opt::parse();

    let input = if opt.input_file.is_some() {
        read_file(opt.input_file.unwrap())
    } else if opt.input.is_some() {
        opt.input.unwrap()
    } else {
        eprintln!("{}", util::NO_INPUT);
        return;
    };

    if !input.is_ascii() {
        eprintln!("WARNING: Input is not ASCII!");
    }

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
