use super::{
    grid::{get_start, print_grid, Space},
    map::MonkeyMap,
    position::{Direction, Position},
};

#[derive(Debug)]
pub struct CubicMap {
    grid: Vec<Vec<Space>>,
}

impl MonkeyMap for CubicMap {
    fn create(grid: Vec<Vec<Space>>) -> Self {
        todo!("not implemented");
        // CubicMap { grid }
    }

    fn start(&self) -> Position {
        get_start(&self.grid)
    }

    fn step(&self, position: Position, orientation: &Direction) -> Position {
        todo!("not implemented")
    }

    #[allow(dead_code)]
    fn print(&self, position: &Position) {
        print_grid(&self.grid, &position);
    }
}
