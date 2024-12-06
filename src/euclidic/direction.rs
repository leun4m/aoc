use super::coord::Coord2D;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthWest,
        Direction::SouthEast,
    ];

    const NORTH: Coord2D = Coord2D(0, -1);
    const WEST: Coord2D = Coord2D(-1, 0);

    pub fn coordinates(self) -> Coord2D {
        match self {
            Direction::North => Direction::NORTH,
            Direction::South => -Direction::NORTH,
            Direction::West => Direction::WEST,
            Direction::East => -Direction::WEST,
            Direction::NorthWest => Direction::NORTH + Direction::WEST,
            Direction::NorthEast => Direction::NORTH + (-Direction::WEST),
            Direction::SouthWest => -Direction::NORTH + Direction::WEST,
            Direction::SouthEast => -Direction::NORTH + (-Direction::WEST),
        }
    }

    pub fn opposing(self) -> Direction {
        *Direction::ALL
            .iter()
            .find(|dir| -dir.coordinates() == self.coordinates())
            .unwrap()
    }

    pub fn rotate_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,

            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}
