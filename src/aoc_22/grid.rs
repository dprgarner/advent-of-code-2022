use itertools::Itertools;

use super::position::Position;

#[derive(PartialEq, Eq, Debug)]
pub enum Space {
    Open,
    Wall,
    Void,
}

use Space::*;

#[derive(Debug)]
pub struct Grid(pub Vec<Vec<Space>>);

impl Grid {
    pub fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut grid = input
            .map(|row| {
                row.chars()
                    .map(|col| match col {
                        '.' => Open,
                        '#' => Wall,
                        ' ' => Void,
                        _ => panic!("Unrecognised character in grid"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        grid.insert(0, vec![]);
        grid.push(vec![]);
        let max_row_length = grid.iter().fold(0, |acc, row| acc.max(row.len())) + 2;
        for i in 0..grid.len() {
            grid[i].insert(0, Void);
            while &grid[i].len() < &max_row_length {
                grid[i].push(Void);
            }
        }
        Self(grid)
    }

    pub fn start(&self) -> Position {
        let start_col = &self.0[1].iter().find_position(|x| x == &&Open).unwrap().0;
        Position(1, *start_col)
    }

    #[allow(dead_code)]
    pub fn print(&self, position: &Position) {
        for (i, row) in self.0.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if &Position(i, j) == position {
                    print!("@");
                } else {
                    print!(
                        "{}",
                        match col {
                            Open => '.',
                            Wall => '#',
                            Void => ' ',
                        }
                    );
                }
            }
            print!("\n");
        }
    }
}
