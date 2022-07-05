# aoc

Solutions from the various years of [AdventOfCode](https://adventofcode.com) in Rust as a tiny CLI application.

## Years

| Year | Progress |    Days |
| ---- |---------:|--------:|
| 2015 |     76 % | 19 / 25 |
| 2016 |     36 % |  9 / 25 |
| 2017 |     20 % |  5 / 25 |
| 2018 |     20 % |  5 / 25 |
| 2019 |      4 % |  1 / 25 |
| 2020 |     64 % | 16 / 25 |
| 2021 |     72 % | 18 / 25 |

## Structure

Each year has it's own module and in that each day is a submodule, so each day is a separate file.
The day modules should in generally not know each other, they are meant to be isolated.

For convenience each day has a `solve(input: &str)` function as a starting point.
This `solve`-function is supposed to calculate the solution to the given input and output them.

## Testing

Usually it's a good idea to write tests according to the examples provided by the puzzle itself...

## Paradigms

The idea is to get to the solution mainly with basic Rust and `std`, so in general additional crates should be added only if really necessary.

Currently, the only packages used are:

- `structopt` (to not worry about the CLI stuff)
- `regex` (since many puzzles require parsing input)
- `md5` [Day 4 in 2015](https://github.com/leun4m/aoc/blob/main/src/year_2015/day_04.rs) required md5 hashing
- `itertool` (Provides neat functional stuff for iterators like `unique()`, `sorted()` which `std` doesn't provide)
- `simple_logger` (for logging)
