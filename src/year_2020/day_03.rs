use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

const SLOPES: &[(u64, u64)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn solve(input: &str) {
    let world = parse_map(input);

    let mut pos = Position::default();
    let mut product: u64 = 1;

    for slope in SLOPES {
        let mut trees = 0;
        while (pos.y + 1) < world.height {
            pos.move_in(slope.0, slope.1, world.width);
            if world.is_tree(&pos) {
                trees += 1;
            }
        }
        println!("Slope {:?} trees: {}", slope, trees);
        product *= trees;
        pos = Position::default();
    }

    println!("Product: {}", product);
}

fn parse_map(input: &str) -> World {
    let width = input.lines().next().unwrap().chars().count() as u64;
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        for char in line.chars() {
            let square = match char {
                '.' => Square::Open,
                '#' => Square::Tree,
                _ => panic!("Unexpected char: {}", char),
            };
            map.insert(Position::new(x, y), square);
            x += 1;
        }
        y += 1;
        x = 0;
    }

    World::new(map, width, y)
}

enum Square {
    Open,
    Tree,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn new(x: u64, y: u64) -> Self {
        Position { x, y }
    }

    fn move_in(&mut self, west: u64, south: u64, maximal_west: u64) {
        if self.x + west < maximal_west {
            self.x += west;
        } else {
            self.x = (self.x + west) - maximal_west;
        }
        self.y += south;
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct World {
    map: HashMap<Position, Square>,
    width: u64,
    height: u64,
}

impl World {
    fn new(map: HashMap<Position, Square>, width: u64, height: u64) -> Self {
        World { map, width, height }
    }

    fn is_tree(&self, position: &Position) -> bool {
        match self.map.get(position) {
            Some(Square::Tree) => true,
            None => panic!("Not a valid position {}", position),
            _ => false,
        }
    }
}
