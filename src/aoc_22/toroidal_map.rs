use super::{
    grid::{
        get_start, print_grid,
        Space::{self, *},
    },
    map::MonkeyMap,
    position::{Direction, Position},
};

#[derive(Debug)]
pub struct ToroidalMap {
    pub grid: Vec<Vec<Space>>,
}

impl MonkeyMap for ToroidalMap {
    fn create(grid: Vec<Vec<Space>>) -> Self {
        ToroidalMap { grid }
    }

    fn start(&self) -> Position {
        get_start(&self.grid)
    }

    fn step(&self, position: Position, orientation: &Direction) -> Position {
        let mut next = position + orientation;
        if self.grid[next.0][next.1] == Void {
            // Wrap back to next open or wall square
            next = position - orientation;
            while self.grid[next.0][next.1] != Void {
                next = next - orientation;
            }
            next = next + orientation;
        }

        if self.grid[next.0][next.1] == Open {
            next
        } else {
            position
        }
    }

    #[allow(dead_code)]
    fn print(&self, position: &Position) {
        print_grid(&self.grid, &position);
    }
}
