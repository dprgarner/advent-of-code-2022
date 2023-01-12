use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

#[derive(PartialEq, Eq, Debug)]
pub struct Direction(pub i32, pub i32);

impl Add<&Direction> for Position {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        Position(
            usize::try_from(i32::try_from(self.0).unwrap() + rhs.0).unwrap(),
            usize::try_from(i32::try_from(self.1).unwrap() + rhs.1).unwrap(),
        )
    }
}

impl Sub<&Direction> for Position {
    type Output = Self;

    fn sub(self, rhs: &Direction) -> Self::Output {
        self + &Direction(-rhs.0, -rhs.1)
    }
}
