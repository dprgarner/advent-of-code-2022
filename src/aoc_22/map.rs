use super::{
    grid::Space,
    position::{Direction, Position},
};

pub trait MonkeyMap {
    fn create(grid: Vec<Vec<Space>>) -> Self;
    fn start(&self) -> Position;
    fn step(&self, position: Position, orientation: &Direction) -> Position;
    fn print(&self, position: &Position);
}
