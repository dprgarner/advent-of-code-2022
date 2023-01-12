use std::ops::Mul;

use super::{
    grid::{get_start, print_grid, Space, Space::*},
    map::MonkeyMap,
    position::{Direction, Position},
};

#[derive(Debug)]
pub struct CubicMap {
    grid: Vec<Vec<Space>>,
}

#[derive(PartialEq, Eq, Debug)]
struct Face {
    coords: Position,
    width: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum D8 {
    Rotation(usize),
    FlipRotation(usize),
}

use itertools::Itertools;
use D8::*;

impl Mul<D8> for D8 {
    type Output = D8;

    fn mul(self, rhs: D8) -> Self::Output {
        match (self, rhs) {
            (Rotation(x), Rotation(y)) => Rotation((x + y) % 4),
            (FlipRotation(x), Rotation(y)) => FlipRotation((x + y) % 4),
            (Rotation(x), FlipRotation(y)) => FlipRotation(((4 + y).saturating_sub(x % 4)) % 4),
            (FlipRotation(x), FlipRotation(y)) => Rotation(((4 + y).saturating_sub(x % 4)) % 4),
        }
    }
}

impl<T> Mul<D8> for Vec<Vec<T>> {
    type Output = Vec<Vec<T>>;

    fn mul(mut self, rhs: D8) -> Self::Output {
        if let Rotation(x) = rhs {
            if x == 0 {
                return self;
            }
            if x == 2 {
                self.reverse();
                for i in 0..self.len() {
                    self[i].reverse();
                }
                return self;
            }
        }
        if let FlipRotation(x) = rhs {
            if x == 0 {
                for i in 0..self.len() {
                    self[i].reverse();
                }
                return self;
            }
            if x == 2 {
                self.reverse();
                return self;
            }
        }
        panic!("TODO")
    }
}

// impl<T> Mul<Vec<Vec<T>>> for D8 {
//     type Output = Vec<Vec<T>>;

//     fn mul(self, rhs: Vec<Vec<T>>) -> Self::Output {
//         if let Rotation(x) = self {
//             if x == 0 {
//                 return rhs;
//             }
//             if x = 2 {
//                 rhs.reverse();
//             }
//         }
//     }
// }

impl D8 {
    fn id() -> Self {
        D8::Rotation(0)
    }

    fn inv(&self) -> Self {
        match self {
            Rotation(x) => Rotation((4 - x) % 4),
            FlipRotation(x) => FlipRotation(*x),
        }
    }
}

fn get_face_coords(grid: &Vec<Vec<Space>>) -> (Vec<Position>, usize) {
    let spaces_count = grid.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, col| {
            acc + match col {
                Space::Void => 0,
                _ => 1,
            }
        })
    });
    if spaces_count % 6 != 0 {
        panic!("Expected a multiple of six non-void spaces");
    }
    let spaces_per_face = spaces_count / 6;
    let face_width = f64::sqrt(spaces_per_face.into()) as usize;

    let mut face_coords = Vec::new();
    for i in 0..(grid.len() - 2) / face_width {
        let i = 1 + i * face_width;
        for j in 0..(grid[i].len() - 2) / face_width {
            let j = 1 + j * face_width;
            if grid[i][j] != Void {
                face_coords.push(Position(i, j))
            }
        }
    }

    (face_coords, face_width)
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

#[cfg(test)]
mod tests {
    use crate::aoc_22::grid::parse_grid;

    use super::*;

    #[rustfmt::skip]
    const TINY_MAP: [&str; 3] = [
        " .  ",
        "....",
        "   .",
    ];

    #[rustfmt::skip]
    const SMALL_MAP: [&str; 6] = [
        "  ..    ",
        "  ..    ",
        "........",
        "........",
        "      ..",
        "      ..",
    ];

    #[rustfmt::skip]
    const WONKY_MAP: [&str; 4] = [
        "  .",
        " ..",
        ".. ",
        ".  ",
    ];

    #[rustfmt::skip]
    const THREE_IN_ROW_MAP: [&str; 5] = [
        " .",
        " .",
        "..",
        ". ",
        ". ",
    ];

    #[test]
    fn it_parses_a_tiny_cube() {
        let grid = parse_grid(
            Vec::from(TINY_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let (positions, width) = get_face_coords(&grid);
        assert_eq!(
            positions,
            vec![
                Position(1, 2),
                Position(2, 1),
                Position(2, 2),
                Position(2, 3),
                Position(2, 4),
                Position(3, 4),
            ],
        );
        assert_eq!(width, 1)
    }

    #[test]
    fn it_parses_a_small_cube() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let (positions, width) = get_face_coords(&grid);
        assert_eq!(
            positions,
            vec![
                Position(1, 3),
                Position(3, 1),
                Position(3, 3),
                Position(3, 5),
                Position(3, 7),
                Position(5, 7),
            ]
        );
        assert_eq!(width, 2)
    }

    #[test]
    fn d8_group() {
        assert_eq!(Rotation(0), D8::id());
        assert_eq!(FlipRotation(1) * D8::id(), FlipRotation(1));
        assert_eq!(D8::id() * Rotation(3), Rotation(3));

        assert_eq!(Rotation(3), Rotation(1).inv());
        assert_eq!(Rotation(0), Rotation(0).inv());
        assert_eq!(FlipRotation(0), FlipRotation(0).inv());
        assert_eq!(FlipRotation(1), FlipRotation(1).inv());

        assert_eq!(FlipRotation(1) * FlipRotation(1), D8::id());
        assert_eq!(FlipRotation(2) * FlipRotation(2), D8::id());
        assert_eq!(Rotation(3) * Rotation(1), D8::id());
        assert_eq!(Rotation(2) * Rotation(2), D8::id());

        assert_eq!(Rotation(1) * FlipRotation(0), FlipRotation(3));
        assert_eq!(Rotation(2) * FlipRotation(0), FlipRotation(2));
        assert_eq!(FlipRotation(0) * Rotation(1), FlipRotation(1));
        assert_eq!(FlipRotation(0) * Rotation(2), FlipRotation(2));
    }
}
