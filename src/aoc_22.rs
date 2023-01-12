mod cubic_map;
mod grid;
mod instruction;
mod map;
mod position;
mod toroidal_map;

use std::error::Error;

use self::cubic_map::CubicMap;
use self::grid::parse_grid;
use self::instruction::Instruction::{self, *};
use self::map::MonkeyMap;
use self::position::{Direction, Position};
use self::toroidal_map::ToroidalMap;

use itertools::Itertools;

#[derive(Debug)]
struct Navigator<M: MonkeyMap> {
    map: M,
    orientation: Direction,
    position: Position,
    instructions: Vec<Instruction>,
}

impl<M: MonkeyMap> Navigator<M> {
    fn create(map: M, instructions: Vec<Instruction>) -> Navigator<M> {
        Navigator {
            position: map.start(),
            map,
            instructions,
            orientation: Direction(0, 1),
        }
    }

    fn navigate(&mut self) {
        for instruction in &self.instructions {
            // self.map.print(&self.position);
            if let Forward(steps) = instruction {
                for _ in 0..*steps {
                    self.position = self.map.step(self.position, &self.orientation);
                }
            } else {
                self.orientation = match (&self.orientation, instruction) {
                    (Direction(-1, 0), Left) => Direction(0, -1),
                    (Direction(0, 1), Left) => Direction(-1, 0),
                    (Direction(1, 0), Left) => Direction(0, 1),
                    (Direction(0, -1), Left) => Direction(1, 0),
                    (Direction(-1, 0), Right) => Direction(0, 1),
                    (Direction(0, 1), Right) => Direction(1, 0),
                    (Direction(1, 0), Right) => Direction(0, -1),
                    (Direction(0, -1), Right) => Direction(-1, 0),
                    _ => panic!("Not a turn"),
                }
            }
        }
        // self.map.print(&self.position);
    }

    fn password(&self) -> i32 {
        let Position(i, j) = &self.position;
        let facing = match &self.orientation {
            Direction(0, 1) => 0,
            Direction(1, 0) => 1,
            Direction(0, -1) => 2,
            Direction(-1, 0) => 3,
            _ => panic!("Unrecognised direction"),
        };
        1000 * i32::try_from(*i).unwrap() + 4 * i32::try_from(*j).unwrap() + facing
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut grid_strings = input.collect_vec();
    let instructions = Instruction::parse(&grid_strings.pop().unwrap());
    grid_strings.pop();
    let grid = parse_grid(grid_strings.into_iter());
    let map = ToroidalMap::create(grid);

    let mut navigator = Navigator::create(map, instructions);
    navigator.navigate();

    Ok(navigator.password())
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut grid_strings = input.collect_vec();
    let instructions = Instruction::parse(&grid_strings.pop().unwrap());
    grid_strings.pop();
    let grid = parse_grid(grid_strings.into_iter());
    let map = CubicMap::create(grid);

    let mut navigator = Navigator::create(map, instructions);
    navigator.navigate();

    Ok(navigator.password())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: [&str; 12] = [
        "        ...#",
        "        .#..",
        "        #...",
        "        ....",
        "...#.......#",
        "........#...",
        "..#....#....",
        "..........#.",
        "        ...#....",
        "        .....#..",
        "        .#......",
        "        ......#.",
    ];

    #[test]
    fn it_navigates_to_a_wall() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap::create(grid);
        let instructions = Instruction::parse("R3");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(2, 9));
    }

    #[test]
    fn it_wraps_around_north() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap::create(grid);
        let instructions = Instruction::parse("L1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(12, 9));
    }

    #[test]
    fn it_wraps_around_south() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap::create(grid);
        let instructions = Instruction::parse("L1RR1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(1, 9));
    }

    #[test]
    fn it_wraps_around_west() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap::create(grid);
        let instructions = Instruction::parse("R1R1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(2, 12));
    }

    #[test]
    fn it_does_not_wrap_around_into_wall() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap::create(grid);
        let instructions = Instruction::parse("RR1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(1, 9));
    }

    #[test]
    fn it_runs_a() {
        let mut input = Vec::from(MAP);
        input.extend_from_slice(&["", "10R5L5R10L4R5L5"]);
        let result = solve_a(input.iter().map(|x| String::from(*x)).into_iter()).unwrap();
        assert_eq!(result, 6032);
    }

    #[ignore]
    #[test]
    fn it_runs_b() {
        let input = ["aaaaa", "bbbbb"].map(String::from).into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 2);
    }
}
