use core::fmt;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub fn solve(input: &str) {
    let final_state_3d = Grid::<Coordinate3D>::parse(input).run_cycles(6);
    println!("Part 1: {}", final_state_3d.actives_count());
    let final_state_4d = Grid::<Coordinate4D>::parse(input).run_cycles(6);
    println!("Part 2: {}", final_state_4d.actives_count());
}

trait GridTrait<T: Coordinate> {
    fn find_new_min_max(&self) -> (T, T);

    fn count_neighbours(&self, coordinate: &T) -> usize;

    fn run_cycles(&self, cycles: u32) -> Self;
}

#[derive(Clone)]
struct Grid<T: Coordinate + Clone> {
    actives: HashSet<T>,
}

impl<T: Coordinate + Clone> Grid<T> {
    fn new() -> Self {
        Grid {
            actives: HashSet::new(),
        }
    }

    fn new_with(actives: HashSet<T>) -> Self {
        Grid { actives }
    }

    fn is_active(&self, coordinate: &T) -> bool {
        self.actives.contains(coordinate)
    }

    fn set_active(&mut self, coordinate: T) {
        self.actives.insert(coordinate);
    }

    fn actives_count(&self) -> usize {
        self.actives.len()
    }

    fn parse(input: &str) -> Grid<T> {
        let mut active_blocks = HashSet::new();

        for (line, y) in input.lines().zip(0..) {
            for (state, x) in line.chars().zip(0..) {
                if state == '#' {
                    active_blocks.insert(T::new_2d(x, y));
                }
            }
        }

        Grid::new_with(active_blocks)
    }
}

impl GridTrait<Coordinate3D> for Grid<Coordinate3D> {
    fn find_new_min_max(&self) -> (Coordinate3D, Coordinate3D) {
        (
            Coordinate3D::new(
                self.actives.iter().map(|c| c.x).min().unwrap() - 1,
                self.actives.iter().map(|c| c.y).min().unwrap() - 1,
                self.actives.iter().map(|c| c.z).min().unwrap() - 1,
            ),
            Coordinate3D::new(
                self.actives.iter().map(|c| c.x).max().unwrap() + 1,
                self.actives.iter().map(|c| c.y).max().unwrap() + 1,
                self.actives.iter().map(|c| c.z).max().unwrap() + 1,
            ),
        )
    }

    fn count_neighbours(&self, coordinate: &Coordinate3D) -> usize {
        let mut count = 0;
        for x in (coordinate.x - 1)..(coordinate.x + 2) {
            for y in (coordinate.y - 1)..(coordinate.y + 2) {
                for z in (coordinate.z - 1)..(coordinate.z + 2) {
                    if self.actives.contains(&Coordinate3D::new(x, y, z)) {
                        count += 1;
                    }
                }
            }
        }

        if self.actives.contains(coordinate) {
            count - 1
        } else {
            count
        }
    }

    fn run_cycles(&self, cycles: u32) -> Self {
        let mut current_state = self.clone();

        for _ in 0..cycles {
            let mut new_state = Grid::new();
            let (min, max) = current_state.find_new_min_max();
            for x in min.x..(max.x + 1) {
                for y in min.y..(max.y + 1) {
                    for z in min.z..(max.z + 1) {
                        let coordinate = Coordinate3D::new(x, y, z);
                        let neighbours = current_state.count_neighbours(&coordinate);

                        if (current_state.is_active(&coordinate) && neighbours == 2)
                            || neighbours == 3
                        {
                            new_state.set_active(coordinate);
                        }
                    }
                }
            }
            current_state = new_state.clone();
        }

        current_state
    }
}

impl GridTrait<Coordinate4D> for Grid<Coordinate4D> {
    fn find_new_min_max(&self) -> (Coordinate4D, Coordinate4D) {
        (
            Coordinate4D::new(
                self.actives.iter().map(|c| c.x).min().unwrap() - 1,
                self.actives.iter().map(|c| c.y).min().unwrap() - 1,
                self.actives.iter().map(|c| c.z).min().unwrap() - 1,
                self.actives.iter().map(|c| c.w).min().unwrap() - 1,
            ),
            Coordinate4D::new(
                self.actives.iter().map(|c| c.x).max().unwrap() + 1,
                self.actives.iter().map(|c| c.y).max().unwrap() + 1,
                self.actives.iter().map(|c| c.z).max().unwrap() + 1,
                self.actives.iter().map(|c| c.w).max().unwrap() + 1,
            ),
        )
    }

    fn count_neighbours(&self, coordinate: &Coordinate4D) -> usize {
        let mut count = 0;
        for x in (coordinate.x - 1)..(coordinate.x + 2) {
            for y in (coordinate.y - 1)..(coordinate.y + 2) {
                for z in (coordinate.z - 1)..(coordinate.z + 2) {
                    for w in (coordinate.w - 1)..(coordinate.w + 2) {
                        if self.actives.contains(&Coordinate4D::new(x, y, z, w)) {
                            count += 1;
                        }
                    }
                }
            }
        }

        if self.actives.contains(coordinate) {
            count - 1
        } else {
            count
        }
    }

    fn run_cycles(&self, cycles: u32) -> Self {
        let mut current_state = self.clone();

        for _ in 0..cycles {
            let mut new_state = Grid::new();
            let (min, max) = current_state.find_new_min_max();
            for x in min.x..(max.x + 1) {
                for y in min.y..(max.y + 1) {
                    for z in min.z..(max.z + 1) {
                        for w in min.w..(max.w + 1) {
                            let coordinate = Coordinate4D::new(x, y, z, w);
                            let neighbours = current_state.count_neighbours(&coordinate);

                            if (current_state.is_active(&coordinate) && neighbours == 2)
                                || neighbours == 3
                            {
                                new_state.set_active(coordinate);
                            }
                        }
                    }
                }
            }
            current_state = new_state.clone();
        }

        current_state
    }
}

type GridBase = i64;

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
