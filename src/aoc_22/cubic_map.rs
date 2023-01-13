use std::collections::{HashMap, HashSet};

use super::{
    grid::{get_start, print_grid, Space, Space::*},
    map::MonkeyMap,
    position::{Direction, Position},
};

#[derive(Debug)]
pub struct CubicMap {
    grid: Vec<Vec<Space>>,
    net: HashMap<(usize, usize), Face>,
    face_width: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Face {
    face_indices: (usize, usize),
    neighbours: HashMap<Direction, ((usize, usize), Direction)>,
}

const NORTH: Direction = Direction(-1, 0);
const EAST: Direction = Direction(0, 1);
const SOUTH: Direction = Direction(1, 0);
const WEST: Direction = Direction(0, -1);

fn get_face_coords(grid: &Vec<Vec<Space>>) -> (Vec<(usize, usize, Position)>, usize) {
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
    for idx_i in 0..(grid.len() - 2) / face_width {
        let i = 1 + idx_i * face_width;
        for idx_j in 0..(grid[i].len() - 2) / face_width {
            let j = 1 + idx_j * face_width;
            if grid[i][j] != Void {
                face_coords.push((idx_i, idx_j, Position(i, j)))
            }
        }
    }

    (face_coords, face_width)
}

/// TODO.
///
/// It would add loads of complexity to this solution implement net-tracing
/// logic in general for every possible cube wireframe. There's only two cube
/// nets being considered in this problem, so I'm just hard-coding these.
fn get_net(face_indices: &HashSet<(usize, usize)>) -> [Face; 6] {
    // ..X.
    // XXX.
    // ..XX
    if face_indices == &HashSet::from([(0, 2), (1, 0), (1, 1), (1, 2), (2, 2), (2, 3)]) {
        return [
            Face {
                face_indices: (0, 2),
                neighbours: HashMap::from([
                    (NORTH, ((1, 0), SOUTH)),
                    (EAST, ((2, 3), WEST)),
                    (SOUTH, ((1, 2), SOUTH)),
                    (WEST, ((1, 1), SOUTH)),
                ]),
            },
            Face {
                face_indices: (1, 0),
                neighbours: HashMap::from([
                    (NORTH, ((0, 2), SOUTH)),
                    (EAST, ((1, 1), EAST)),
                    (SOUTH, ((2, 2), NORTH)),
                    (WEST, ((2, 3), NORTH)),
                ]),
            },
            Face {
                face_indices: (1, 1),
                neighbours: HashMap::from([
                    (NORTH, ((0, 2), EAST)),
                    (EAST, ((1, 2), EAST)),
                    (SOUTH, ((2, 2), EAST)),
                    (WEST, ((1, 0), WEST)),
                ]),
            },
            Face {
                face_indices: (1, 2),
                neighbours: HashMap::from([
                    (NORTH, ((0, 2), NORTH)),
                    (EAST, ((2, 3), SOUTH)),
                    (SOUTH, ((2, 2), SOUTH)),
                    (WEST, ((1, 1), WEST)),
                ]),
            },
            Face {
                face_indices: (2, 2),
                neighbours: HashMap::from([
                    (NORTH, ((1, 2), NORTH)),
                    (EAST, ((2, 3), EAST)),
                    (SOUTH, ((1, 0), NORTH)),
                    (WEST, ((1, 1), NORTH)),
                ]),
            },
            Face {
                face_indices: (2, 3),
                neighbours: HashMap::from([
                    (NORTH, ((1, 2), WEST)),
                    (EAST, ((0, 2), WEST)),
                    (SOUTH, ((1, 0), EAST)),
                    (WEST, ((2, 2), WEST)),
                ]),
            },
        ];
    }

    // .XX
    // .X
    // XX
    // X.
    if face_indices == &HashSet::from([(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)]) {
        return [
            Face {
                face_indices: (0, 1),
                neighbours: HashMap::from([
                    (NORTH, ((3, 0), EAST)),
                    (EAST, ((0, 2), EAST)),
                    (SOUTH, ((1, 1), SOUTH)),
                    (WEST, ((2, 0), EAST)),
                ]),
            },
            Face {
                face_indices: (0, 2),
                neighbours: HashMap::from([
                    (NORTH, ((3, 0), NORTH)),
                    (EAST, ((2, 1), WEST)),
                    (SOUTH, ((1, 1), WEST)),
                    (WEST, ((0, 1), WEST)),
                ]),
            },
            Face {
                face_indices: (1, 1),
                neighbours: HashMap::from([
                    (NORTH, ((0, 1), NORTH)),
                    (EAST, ((0, 2), NORTH)),
                    (SOUTH, ((2, 1), SOUTH)),
                    (WEST, ((2, 0), SOUTH)),
                ]),
            },
            Face {
                face_indices: (2, 0),
                neighbours: HashMap::from([
                    (NORTH, ((1, 1), EAST)),
                    (EAST, ((2, 1), EAST)),
                    (SOUTH, ((3, 0), SOUTH)),
                    (WEST, ((0, 1), EAST)),
                ]),
            },
            Face {
                face_indices: (2, 1),
                neighbours: HashMap::from([
                    (NORTH, ((1, 1), NORTH)),
                    (EAST, ((0, 2), WEST)),
                    (SOUTH, ((3, 0), WEST)),
                    (WEST, ((2, 0), WEST)),
                ]),
            },
            Face {
                face_indices: (3, 0),
                neighbours: HashMap::from([
                    (NORTH, ((2, 0), NORTH)),
                    (EAST, ((2, 1), NORTH)),
                    (SOUTH, ((0, 2), SOUTH)),
                    (WEST, ((0, 1), SOUTH)),
                ]),
            },
        ];
    }

    todo!("unrecognised net");
}

impl MonkeyMap for CubicMap {
    fn create(grid: Vec<Vec<Space>>) -> Self {
        let (face_coords, face_width) = get_face_coords(&grid);
        let face_indices: HashSet<(usize, usize)> =
            HashSet::from_iter(face_coords.iter().map(|(i, j, _)| (*i, *j)));
        let net = get_net(&face_indices);

        CubicMap {
            grid,
            net: HashMap::from_iter(net.into_iter().map(|f| (f.face_indices, f))),
            face_width,
        }
    }

    fn start(&self) -> Position {
        get_start(&self.grid)
    }

    fn step(&self, position: Position, orientation: Direction) -> (Position, Direction) {
        let mut next_position = position + &orientation;
        let mut next_orientation = &orientation;

        if self.grid[next_position.0][next_position.1] == Void {
            let face_indices = (
                (position.0 - 1) / self.face_width,
                (position.1 - 1) / self.face_width,
            );
            let (destination_face_idx, destination_orientation) = self
                .net
                .get(&face_indices)
                .unwrap()
                .neighbours
                .get(&orientation)
                .unwrap();

            let origin_face_top_left = Position(
                1 + self.face_width * face_indices.0,
                1 + self.face_width * face_indices.1,
            );
            let distance_from_corner_on_left = match &orientation {
                &NORTH => position.1 - origin_face_top_left.1,
                &EAST => position.0 - origin_face_top_left.0,
                &SOUTH => self.face_width - 1 - (position.1 - origin_face_top_left.1),
                &WEST => self.face_width - 1 - (position.0 - origin_face_top_left.0),
                _ => panic!("Unexpected direction"),
            };

            let destination_face_top_left = (
                1 + self.face_width * destination_face_idx.0,
                1 + self.face_width * destination_face_idx.1,
            );

            next_position = match destination_orientation {
                &NORTH => Position(
                    destination_face_top_left.0 + self.face_width - 1,
                    destination_face_top_left.1 + distance_from_corner_on_left,
                ),
                &EAST => Position(
                    destination_face_top_left.0 + distance_from_corner_on_left,
                    destination_face_top_left.1,
                ),
                &SOUTH => Position(
                    destination_face_top_left.0,
                    destination_face_top_left.1 + self.face_width
                        - 1
                        - distance_from_corner_on_left,
                ),
                &WEST => Position(
                    destination_face_top_left.0 + self.face_width
                        - 1
                        - distance_from_corner_on_left,
                    destination_face_top_left.1 + self.face_width - 1,
                ),
                _ => panic!("Unexpected direction"),
            };
            next_orientation = destination_orientation;
        }

        if self.grid[next_position.0][next_position.1] == Open {
            (next_position, *next_orientation)
        } else {
            (position, orientation)
        }
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
    const TINY_MAP: [&str; 4] = [
        " ..",
        " . ",
        ".. ",
        ".  ",
    ];

    #[rustfmt::skip]
    const SMALL_MAP: [&str; 6] = [
        "    ..  ",
        "    ..  ",
        "......  ",
        "......  ",
        "    ....",
        "    ....",
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
                (0, 1, Position(1, 2)),
                (0, 2, Position(1, 3)),
                (1, 1, Position(2, 2)),
                (2, 0, Position(3, 1)),
                (2, 1, Position(3, 2)),
                (3, 0, Position(4, 1)),
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
                (0, 2, Position(1, 5)),
                (1, 0, Position(3, 1)),
                (1, 1, Position(3, 3)),
                (1, 2, Position(3, 5)),
                (2, 2, Position(5, 5)),
                (2, 3, Position(5, 7)),
            ]
        );
        assert_eq!(width, 2)
    }

    #[test]
    fn it_consistently_implements_small_net() {
        // ..X.
        // XXX.
        // ..XX
        let face_indices = HashSet::from([(0, 2), (1, 0), (1, 1), (1, 2), (2, 2), (2, 3)]);
        let net = get_net(&face_indices);
        assert_eq!(
            net[0],
            Face {
                face_indices: (0, 2),
                neighbours: HashMap::from([
                    (NORTH, ((1, 0), SOUTH)),
                    (EAST, ((2, 3), WEST)),
                    (SOUTH, ((1, 2), SOUTH)),
                    (WEST, ((1, 1), SOUTH)),
                ]),
            }
        );
        // Check the reversibility of nets
        for face in net.iter() {
            for (dir, (neighbour_idx, dir_on_neighbour)) in &face.neighbours {
                let neighbour = net
                    .iter()
                    .find(|f| &f.face_indices == neighbour_idx)
                    .unwrap();
                let reverse_dir = Direction(-dir_on_neighbour.0, -dir_on_neighbour.1);
                let (go_back_idx, go_back_direction) =
                    neighbour.neighbours.get(&reverse_dir).unwrap();
                assert_eq!(go_back_idx, &face.face_indices);
                assert_eq!(go_back_direction, &Direction(-dir.0, -dir.1));
            }
        }
    }

    #[test]
    fn it_consistently_implements_tiny_net() {
        // .XX
        // .X
        // XX
        // X.
        let face_indices = HashSet::from([(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)]);
        let net = get_net(&face_indices);
        assert_eq!(
            net[0],
            Face {
                face_indices: (0, 1),
                neighbours: HashMap::from([
                    (NORTH, ((3, 0), EAST)),
                    (EAST, ((0, 2), EAST)),
                    (SOUTH, ((1, 1), SOUTH)),
                    (WEST, ((2, 0), EAST)),
                ]),
            },
        );
        // Check the reversibility of nets
        for face in net.iter() {
            for (dir, (neighbour_idx, dir_on_neighbour)) in &face.neighbours {
                let neighbour = net
                    .iter()
                    .find(|f| &f.face_indices == neighbour_idx)
                    .unwrap();
                let reverse_dir = Direction(-dir_on_neighbour.0, -dir_on_neighbour.1);
                let (go_back_idx, go_back_direction) =
                    neighbour.neighbours.get(&reverse_dir).unwrap();
                assert_eq!(go_back_idx, &face.face_indices);
                assert_eq!(go_back_direction, &Direction(-dir.0, -dir.1));
            }
        }
    }

    #[test]
    fn it_steps_north_to_east() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     ..
        //     !.
        // ...@..
        // ......
        //     ....
        //     ....
        assert_eq!(map.step(Position(3, 4), NORTH), (Position(2, 5), EAST));

        //     !.
        //     ..
        // ..@...
        // ......
        //     ....
        //     ....
        assert_eq!(map.step(Position(3, 3), NORTH), (Position(1, 5), EAST));
    }

    #[test]
    fn it_steps_north_to_south() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     !.
        //     ..
        // .@....
        // ......
        //     ....
        //     ....
        assert_eq!(map.step(Position(3, 2), NORTH), (Position(1, 5), SOUTH));

        //     .!
        //     ..
        // @.....
        // ......
        //     ....
        //     ....
        assert_eq!(map.step(Position(3, 1), NORTH), (Position(1, 6), SOUTH));
    }

    #[test]
    fn it_steps_north_to_west() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     ..
        //     ..
        // ......
        // .....!
        //     ..@.
        //     ....
        assert_eq!(map.step(Position(5, 7), NORTH), (Position(4, 6), WEST));

        //     ..
        //     ..
        // .....!
        // ......
        //     ...@
        //     ....
        assert_eq!(map.step(Position(5, 8), NORTH), (Position(3, 6), WEST));
    }

    #[test]
    fn it_steps_west_to_north() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     ..
        //     ..
        // ......
        // ...!..
        //     @...
        //     ....
        assert_eq!(map.step(Position(5, 5), WEST), (Position(4, 4), NORTH));

        //     ..
        //     ..
        // ......
        // ..!...
        //     ....
        //     @...
        assert_eq!(map.step(Position(6, 5), WEST), (Position(4, 3), NORTH));
    }

    #[test]
    fn it_steps_south_to_east() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     ..
        //     ..
        // ......
        // !.....
        //     ....
        //     ..@.
        assert_eq!(map.step(Position(6, 7), SOUTH), (Position(4, 1), EAST));

        //     ..
        //     ..
        // !.....
        // ......
        //     ....
        //     ...@
        assert_eq!(map.step(Position(6, 8), SOUTH), (Position(3, 1), EAST));
    }

    #[test]
    fn it_steps_east_to_south() {
        let grid = parse_grid(
            Vec::from(SMALL_MAP)
                .iter()
                .map(|x| String::from(*x))
                .into_iter(),
        );
        let map = CubicMap::create(grid);

        //     ..
        //     ..
        // .....@
        // ......
        //     ...!
        //     ....
        assert_eq!(map.step(Position(3, 6), EAST), (Position(5, 8), SOUTH));

        //     ..
        //     ..
        // ......
        // .....@
        //     ..!.
        //     ....
        assert_eq!(map.step(Position(4, 6), EAST), (Position(5, 7), SOUTH));
    }
}
