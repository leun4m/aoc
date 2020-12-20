use core::fmt;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub fn main(input: &str) {
    let initial_state_3d = parse_input(input);
    let final_state_3d = run_cycles_3d(&initial_state_3d, 6);
    println!("Part 1: {}", final_state_3d.len());
    let initial_state_4d = parse_input(input);
    let final_state_4d = run_cycles_4d(&initial_state_4d, 6);
    println!("Part 2: {}", final_state_4d.len());
}

fn run_cycles_3d(initial: &ActiveBlocks3D, cycles: u32) -> ActiveBlocks3D {
    let mut current_state = initial.clone();

    for _ in 0..cycles {
        let mut new_state = HashSet::new();
        let (min, max) = Coordinate3D::find_new_min_max(&current_state);
        for x in min.x..(max.x + 1) {
            for y in min.y..(max.y + 1) {
                for z in min.z..(max.z + 1) {
                    let coordinate = Coordinate3D::new(x, y, z);
                    let neighbours = coordinate.count_neighbours(&current_state);

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

fn run_cycles_4d(initial: &ActiveBlocks4D, cycles: u32) -> ActiveBlocks4D {
    let mut current_state = initial.clone();

    for _ in 0..cycles {
        let mut new_state = HashSet::new();
        let (min, max) = Coordinate4D::find_new_min_max(&current_state);
        for x in min.x..(max.x + 1) {
            for y in min.y..(max.y + 1) {
                for z in min.z..(max.z + 1) {
                    for w in min.w..(max.w + 1) {
                        let coordinate = Coordinate4D::new(x, y, z, w);
                        let neighbours = coordinate.count_active_neighbours(&current_state);

                        if (current_state.contains(&coordinate) && neighbours == 2)
                            || neighbours == 3
                        {
                            new_state.insert(coordinate);
                        }
                    }
                }
            }
        }
        current_state = new_state.clone();
    }

    current_state
}

fn parse_input<T: Coordinate>(input: &str) -> HashSet<T> {
    let mut active_blocks = HashSet::new();

    for (line, y) in input.lines().zip(0..) {
        for (state, x) in line.chars().zip(0..) {
            if state == '#' {
                active_blocks.insert(T::new_2d(x, y));
            }
        }
    }

    active_blocks
}

type GridBase = i64;
type ActiveBlocks3D = HashSet<Coordinate3D>;
type ActiveBlocks4D = HashSet<Coordinate4D>;

trait Coordinate: Hash + Eq {
    fn new_2d(x: GridBase, y: GridBase) -> Self;
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate3D {
    x: GridBase,
    y: GridBase,
    z: GridBase,
}

impl Coordinate3D {
    fn new(x: GridBase, y: GridBase, z: GridBase) -> Self {
        Coordinate3D { x, y, z }
    }

    fn find_new_min_max(actives: &ActiveBlocks3D) -> (Self, Self) {
        (
            Coordinate3D::new(
                actives.iter().map(|c| c.x).min().unwrap() - 1,
                actives.iter().map(|c| c.y).min().unwrap() - 1,
                actives.iter().map(|c| c.z).min().unwrap() - 1,
            ),
            Coordinate3D::new(
                actives.iter().map(|c| c.x).max().unwrap() + 1,
                actives.iter().map(|c| c.y).max().unwrap() + 1,
                actives.iter().map(|c| c.z).max().unwrap() + 1,
            ),
        )
    }

    fn count_neighbours(&self, state: &ActiveBlocks3D) -> usize {
        let mut count = 0;
        for x in (self.x - 1)..(self.x + 2) {
            for y in (self.y - 1)..(self.y + 2) {
                for z in (self.z - 1)..(self.z + 2) {
                    if state.contains(&Coordinate3D::new(x, y, z)) {
                        count += 1;
                    }
                }
            }
        }

        if state.contains(self) {
            count - 1
        } else {
            count
        }
    }
}

impl Coordinate for Coordinate3D {
    fn new_2d(x: i64, y: i64) -> Self {
        Self::new(x, y, 0)
    }
}

impl Debug for Coordinate3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate4D {
    x: GridBase,
    y: GridBase,
    z: GridBase,
    w: GridBase,
}

impl Coordinate4D {
    fn new(x: GridBase, y: GridBase, z: GridBase, w: GridBase) -> Self {
        Coordinate4D { x, y, z, w }
    }

    fn find_new_min_max(actives: &ActiveBlocks4D) -> (Self, Self) {
        (
            Self::new(
                actives.iter().map(|c| c.x).min().unwrap() - 1,
                actives.iter().map(|c| c.y).min().unwrap() - 1,
                actives.iter().map(|c| c.z).min().unwrap() - 1,
                actives.iter().map(|c| c.w).min().unwrap() - 1,
            ),
            Self::new(
                actives.iter().map(|c| c.x).max().unwrap() + 1,
                actives.iter().map(|c| c.y).max().unwrap() + 1,
                actives.iter().map(|c| c.z).max().unwrap() + 1,
                actives.iter().map(|c| c.w).max().unwrap() + 1,
            ),
        )
    }

    fn count_active_neighbours(&self, state: &ActiveBlocks4D) -> usize {
        let mut count = 0;
        for x in (self.x - 1)..(self.x + 2) {
            for y in (self.y - 1)..(self.y + 2) {
                for z in (self.z - 1)..(self.z + 2) {
                    for w in (self.w - 1)..(self.w + 2) {
                        if state.contains(&Coordinate4D::new(x, y, z, w)) {
                            count += 1;
                        }
                    }
                }
            }
        }

        if state.contains(self) {
            count - 1
        } else {
            count
        }
    }
}

impl Coordinate for Coordinate4D {
    fn new_2d(x: i64, y: i64) -> Self {
        Self::new(x, y, 0, 0)
    }
}

impl Debug for Coordinate4D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}
