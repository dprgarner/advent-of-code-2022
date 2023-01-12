use super::{
    grid::{Grid, Space::*},
    position::{Direction, Position},
};

#[derive(Debug)]
pub struct ToroidalMap {
    pub grid: Grid,
}

impl ToroidalMap {
    pub fn step(&self, position: Position, orientation: &Direction) -> Position {
        let mut next = position + orientation;
        if self.grid.0[next.0][next.1] == Void {
            // Wrap back to next open or wall square
            next = position - orientation;
            while self.grid.0[next.0][next.1] != Void {
                next = next - orientation;
            }
            next = next + orientation;
        }

        if self.grid.0[next.0][next.1] == Open {
            next
        } else {
            position
        }
    }
}
