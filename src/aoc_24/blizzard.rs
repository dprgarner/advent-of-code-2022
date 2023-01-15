use std::{cell::RefCell, collections::HashSet};

use itertools::Itertools;

use super::position::{Direction, Position, EAST, NORTH, SOUTH, WEST};

#[derive(PartialEq, Eq, Debug)]
pub struct BlizzardHistory {
    pub map_width: i32,
    pub map_height: i32,
    latest_blizzards: RefCell<Vec<(Direction, Position)>>,
    blizzard_lookup: RefCell<Vec<HashSet<Position>>>,
}

impl BlizzardHistory {
    pub fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut latest_blizzards = Vec::new();
        let mut blizzard_lookup = HashSet::new();

        let map_strs = input.collect_vec();
        for (i, row) in map_strs.iter().enumerate() {
            for (j, col) in row.chars().enumerate() {
                let i = i32::try_from(i).unwrap();
                let j = i32::try_from(j).unwrap();
                if let Some(blizzard) = match col {
                    '^' => Some((NORTH, Position(i, j))),
                    '>' => Some((EAST, Position(i, j))),
                    'v' => Some((SOUTH, Position(i, j))),
                    '<' => Some((WEST, Position(i, j))),
                    _ => None,
                } {
                    blizzard_lookup.insert(blizzard.1);
                    latest_blizzards.push(blizzard);
                }
            }
        }
        Self {
            map_width: i32::try_from(map_strs[0].len()).unwrap(),
            map_height: i32::try_from(map_strs.len()).unwrap(),
            latest_blizzards: RefCell::from(latest_blizzards),
            blizzard_lookup: RefCell::from(vec![blizzard_lookup]),
        }
    }

    fn next(&self) {
        let mut next_blizzards = Vec::new();
        let mut blizzard_lookup = HashSet::new();

        for blizzard in self.latest_blizzards.borrow().iter() {
            let (dir, mut position) = blizzard;
            position = position + dir;
            if position.0 == 0 {
                position = Position(self.map_height - 2, position.1)
            };
            if position.1 == self.map_width - 1 {
                position = Position(position.0, 1)
            };
            if position.0 == self.map_height - 1 {
                position = Position(1, position.1)
            };
            if position.1 == 0 {
                position = Position(position.0, self.map_width - 2)
            };
            blizzard_lookup.insert(position);
            next_blizzards.push((*dir, position));
        }
        *self.latest_blizzards.borrow_mut() = next_blizzards;
        self.blizzard_lookup.borrow_mut().push(blizzard_lookup);
    }

    pub fn has_blizzard_at(&self, turn: usize, position: &Position) -> bool {
        if self.blizzard_lookup.borrow().get(turn).is_none() {
            self.next()
        }
        self.blizzard_lookup.borrow()[turn].contains(position)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[rustfmt::skip]
    const MAP: [&str; 7] = [
        "#.#####",
        "#.....#",
        "#..^>.#",
        "#.....#",
        "#...v.#",
        "#.<...#",
        "#####.#",
    ];

    #[test]
    fn it_parses_the_blizzard_text() {
        let input = MAP.map(String::from).into_iter();
        let result = BlizzardHistory::parse(input);
        assert_eq!(
            result,
            BlizzardHistory {
                map_width: 7,
                map_height: 7,
                latest_blizzards: RefCell::from(vec![
                    (NORTH, Position(2, 3)),
                    (EAST, Position(2, 4)),
                    (SOUTH, Position(4, 4)),
                    (WEST, Position(5, 2)),
                ]),
                blizzard_lookup: RefCell::from(vec![HashSet::from([
                    Position(2, 3),
                    Position(2, 4),
                    Position(4, 4),
                    Position(5, 2)
                ])]),
            },
        );
    }

    #[test]
    fn it_moves_blizzards() {
        let input = MAP.map(String::from).into_iter();
        let blizzard_history = BlizzardHistory::parse(input);
        blizzard_history.next();
        assert_eq!(
            blizzard_history,
            BlizzardHistory {
                map_width: 7,
                map_height: 7,
                latest_blizzards: RefCell::from(vec![
                    (NORTH, Position(1, 3)),
                    (EAST, Position(2, 5)),
                    (SOUTH, Position(5, 4)),
                    (WEST, Position(5, 1)),
                ]),
                blizzard_lookup: RefCell::from(vec![
                    HashSet::from([
                        Position(2, 3),
                        Position(2, 4),
                        Position(4, 4),
                        Position(5, 2)
                    ]),
                    HashSet::from([
                        Position(1, 3),
                        Position(2, 5),
                        Position(5, 4),
                        Position(5, 1),
                    ])
                ]),
            },
        )
    }

    #[test]
    fn it_wraps_blizzards_around() {
        let input = MAP.map(String::from).into_iter();
        let blizzard_history = BlizzardHistory::parse(input);
        blizzard_history.next();
        blizzard_history.next();
        // #.#####
        // #...v.#
        // #>....#
        // #.....#
        // #.....#
        // #..^.<#
        // #####.#
        assert_eq!(
            blizzard_history,
            BlizzardHistory {
                map_width: 7,
                map_height: 7,
                latest_blizzards: RefCell::from(vec![
                    (NORTH, Position(5, 3)),
                    (EAST, Position(2, 1)),
                    (SOUTH, Position(1, 4)),
                    (WEST, Position(5, 5)),
                ]),
                blizzard_lookup: RefCell::from(vec![
                    HashSet::from([
                        Position(2, 3),
                        Position(2, 4),
                        Position(4, 4),
                        Position(5, 2)
                    ]),
                    HashSet::from([
                        Position(1, 3),
                        Position(2, 5),
                        Position(5, 4),
                        Position(5, 1),
                    ]),
                    HashSet::from([
                        Position(5, 3),
                        Position(2, 1),
                        Position(1, 4),
                        Position(5, 5),
                    ])
                ]),
            },
        )
    }

    #[test]
    fn it_iterates_and_returns() {
        let input = MAP.map(String::from).into_iter();
        let blizzard_history = BlizzardHistory::parse(input);
        // After 0 turns:
        // #.#####
        // #.....#
        // #..^>.#
        // #.....#
        // #...v.#
        // #.<...#
        // #####.#
        assert_eq!(blizzard_history.has_blizzard_at(0, &Position(2, 2)), false);
        assert_eq!(blizzard_history.has_blizzard_at(0, &Position(2, 3)), true);
        assert_eq!(blizzard_history.has_blizzard_at(0, &Position(2, 4)), true);
        assert_eq!(blizzard_history.has_blizzard_at(0, &Position(2, 5)), false);

        blizzard_history.next();
        // After 2 turns:
        // #.#####
        // #...v.#
        // #>....#
        // #.....#
        // #.....#
        // #..^.<#
        // #####.#
        assert_eq!(blizzard_history.has_blizzard_at(2, &Position(1, 4)), true);
        assert_eq!(blizzard_history.has_blizzard_at(2, &Position(1, 5)), false);
        assert_eq!(blizzard_history.has_blizzard_at(2, &Position(5, 3)), true);
        assert_eq!(blizzard_history.has_blizzard_at(2, &Position(5, 4)), false);
        assert_eq!(blizzard_history.has_blizzard_at(2, &Position(5, 5)), true);
    }
}
