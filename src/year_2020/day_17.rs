use core::fmt;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

pub fn main(input: &str) {
    let initial_state = parse_input(input);
    let final_state = run_cycles(&initial_state, 6);
    println!("Part 1: {}", final_state.len());
}

fn run_cycles(initial: &ActiveBlocks, cycles: u32) -> ActiveBlocks {
    let mut current_state = initial.clone();

    for _ in 0..cycles {
        let mut new_state = HashSet::new();
        let (min, max) = find_new_min_max(&current_state);
        for x in min.x..(max.x + 1) {
            for y in min.y..(max.y + 1) {
                for z in min.z..(max.z + 1) {
                    let coordinate = Coordinate::new(x, y, z);
                    let neighbours = count_active_neighbours(&coordinate, &current_state);

                    if (current_state.contains(&coordinate) && neighbours == 2) || neighbours == 3 {
                        new_state.insert(coordinate);
                    }
                }
            }
        }
        current_state = new_state.clone();
    }

    current_state
}

fn count_active_neighbours(block: &Coordinate, state: &ActiveBlocks) -> usize {
    let mut count = 0;
    for x in (block.x - 1)..(block.x + 2) {
        for y in (block.y - 1)..(block.y + 2) {
            for z in (block.z - 1)..(block.z + 2) {
                if state.contains(&Coordinate::new(x, y, z)) {
                    count += 1;
                }
            }
        }
    }

    if state.contains(&block) {
        count - 1
    } else {
        count
    }
}

type GridBase = i64;
type ActiveBlocks = HashSet<Coordinate>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: GridBase,
    y: GridBase,
    z: GridBase,
}

impl Coordinate {
    fn new(x: GridBase, y: GridBase, z: GridBase) -> Self {
        Coordinate { x, y, z }
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

fn parse_input(input: &str) -> ActiveBlocks {
    let mut active_blocks = HashSet::new();
    let initial_z = 0;

    for (line, y) in input.lines().zip(0..) {
        for (state, x) in line.chars().zip(0..) {
            if state == '#' {
                active_blocks.insert(Coordinate::new(x, y, initial_z));
            }
        }
    }

    active_blocks
}

fn find_new_min_max(actives: &ActiveBlocks) -> (Coordinate, Coordinate) {
    (
        Coordinate::new(
            actives.iter().map(|c| c.x).min().unwrap() - 1,
            actives.iter().map(|c| c.y).min().unwrap() - 1,
            actives.iter().map(|c| c.z).min().unwrap() - 1,
        ),
        Coordinate::new(
            actives.iter().map(|c| c.x).max().unwrap() + 1,
            actives.iter().map(|c| c.y).max().unwrap() + 1,
            actives.iter().map(|c| c.z).max().unwrap() + 1,
        ),
    )
}
