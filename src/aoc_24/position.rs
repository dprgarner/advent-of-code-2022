use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Position(pub i32, pub i32);

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

impl Add<&Direction> for Position {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + &rhs
    }
}

impl Sub<Direction> for Position {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + Direction(-rhs.0, -rhs.1)
    }
}

impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }
}

pub const NORTH: Direction = Direction(-1, 0);
pub const EAST: Direction = Direction(0, 1);
pub const SOUTH: Direction = Direction(1, 0);
pub const WEST: Direction = Direction(0, -1);
