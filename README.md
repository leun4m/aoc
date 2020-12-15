# aoc

Solutions from the various years of [AdventOfCode](https://adventofcode.com) in Rust as a tiny CLI application.

## Years

### In Progress

- 2020
- 2015

## Structure

Each year has it's own module and in that each day is a submodule, so each day is a separate file.
The day modules should in generally not know each other, they are meant to be isolated. 

For convenience each day has a `main(input: &str)` function as a starting point.
This `main`-function is supposed to calculate the solution to the given input and output them.

## Testing

Usually it's a good idea to write tests according to the examples provided by the puzzle itself...

## Paradigms

The idea is to get to the solution mainly with basic Rust and `std`, so in general additional crates should be added only if really necessary.

Currently, the only packages used are:

- `structopt` (to not worry about the CLI stuff)
- `regex` (since many puzzles require parsing input)
- `crypto` [Day 4 in 2015](https://github.com/leun4m/aoc/blob/main/src/year_2015/day_04.rs) required md5 hashing
