use std::ops::{Add, Mul, Neg};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord2D(pub isize, pub isize);

impl Neg for Coord2D {
    type Output = Coord2D;

    fn neg(self) -> Self::Output {
        Coord2D(-self.0, -self.1)
    }
}

impl Add<Coord2D> for Coord2D {
    type Output = Coord2D;

    fn add(self, rhs: Coord2D) -> Self::Output {
        Coord2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<Coord2D> for isize {
    type Output = Coord2D;

    fn mul(self, rhs: Coord2D) -> Self::Output {
        Coord2D(self * rhs.0, self * rhs.1)
    }
}
