use itertools::Itertools;

use super::position::Position;

#[derive(PartialEq, Eq, Debug)]
pub enum Space {
    Open,
    Wall,
    Void,
}

use Space::*;

pub fn parse_grid(input: impl Iterator<Item = String>) -> Vec<Vec<Space>> {
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
    grid
}

pub fn get_start(grid: &Vec<Vec<Space>>) -> Position {
    let start_col = grid[1].iter().find_position(|x| x == &&Open).unwrap().0;
    Position(1, start_col)
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<Space>>, position: &Position) {
    for (i, row) in grid.iter().enumerate() {
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
    fn it_parses_a_grid() {
        let grid = parse_grid(Vec::from(MAP).iter().map(|x| String::from(*x)).into_iter());
        assert_eq!(grid.len(), 14);
        assert_eq!(
            grid[0],
            vec![
                Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void, Void,
                Void, Void, Void, Void,
            ]
        );
        assert_eq!(
            grid[1],
            vec![
                Void, // Extra voids are added at the beginning
                Void, Void, Void, Void, Void, Void, Void, Void, Open, Open, Open, Wall,
                // Adding extra voids at the end
                Void, Void, Void, Void, Void,
            ]
        );
    }
}
