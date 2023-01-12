mod grid;
mod instruction;
mod map;
mod position;

use std::error::Error;

use grid::Grid;
use instruction::Instruction::{self, *};
use itertools::Itertools;
use map::ToroidalMap;
use position::{Direction, Position};

#[derive(Debug)]
struct Navigator {
    map: ToroidalMap,
    instructions: Vec<Instruction>,
    position: Position,
    orientation: Direction,
}

impl Navigator {
    fn create(map: ToroidalMap, instructions: Vec<Instruction>) -> Navigator {
        Navigator {
            position: map.grid.start(),
            map,
            instructions,
            orientation: Direction(0, 1),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.map.grid.print(&self.position);
    }

    fn navigate(&mut self) {
        for instruction in &self.instructions {
            // self.print();
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
        // self.print();
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
    let grid = Grid::parse(grid_strings.into_iter());
    let map = ToroidalMap { grid };

    let mut navigator = Navigator::create(map, instructions);
    navigator.navigate();

    Ok(navigator.password())
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut grid_strings = input.collect_vec();
    let instructions = Instruction::parse(&grid_strings.pop().unwrap());
    grid_strings.pop();
    let grid = Grid::parse(grid_strings.into_iter());
    let map = ToroidalMap { grid };

    let mut navigator = Navigator::create(map, instructions);
    navigator.navigate();

    Ok(navigator.password())
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::Space::*;

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
    fn it_parses_instructions() {
        assert_eq!(
            Instruction::parse("R10"),
            vec![Instruction::Right, Instruction::Forward(10)]
        );
        assert_eq!(Instruction::parse("101"), vec![Instruction::Forward(101)]);
        assert_eq!(
            Instruction::parse("10R5L5R10L4R5L5"),
            vec![
                Instruction::Forward(10),
                Instruction::Right,
                Instruction::Forward(5),
                Instruction::Left,
                Instruction::Forward(5),
                Instruction::Right,
                Instruction::Forward(10),
                Instruction::Left,
                Instruction::Forward(4),
                Instruction::Right,
                Instruction::Forward(5),
                Instruction::Left,
                Instruction::Forward(5),
            ]
        );
    }

    #[test]
    fn it_parses_a_grid() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        assert_eq!(grid.0.len(), 14);
        assert_eq!(
            grid.0[0],
            vec![
                Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void,
                Void, Void, Void, Void,
            ]
        );
        assert_eq!(
            grid.0[1],
            vec![
                Void, // Extra voids are added at the beginning
                Void, Void, Void, Void, Void, Void, Void, Void, Open, Open, Open, Wall,
                // Adding extra voids at the end
                Void, Void, Void, Void, Void,
            ]
        );
    }

    #[test]
    fn it_navigates_to_a_wall() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap { grid };
        let instructions = Instruction::parse("R3");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(2, 9));
    }

    #[test]
    fn it_wraps_around_north() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap { grid };
        let instructions = Instruction::parse("L1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(12, 9));
    }

    #[test]
    fn it_wraps_around_south() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap { grid };
        let instructions = Instruction::parse("L1RR1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(1, 9));
    }

    #[test]
    fn it_wraps_around_west() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap { grid };
        let instructions = Instruction::parse("R1R1");
        let mut navigator = Navigator::create(map, instructions);

        assert_eq!(navigator.position, Position(1, 9));
        navigator.navigate();
        assert_eq!(navigator.position, Position(2, 12));
    }

    #[test]
    fn it_does_not_wrap_around_into_wall() {
        let grid = Grid::parse(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        let map = ToroidalMap { grid };
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
