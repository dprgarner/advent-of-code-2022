use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::{collections::HashSet, error::Error};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Position(pub i32, pub i32);

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Direction> for Position {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + Direction(-rhs.0, -rhs.1)
    }
}

struct ElfCollection {
    elves: HashSet<Position>,
    round: usize,
}

const NORTH: Direction = Direction(-1, 0);
const EAST: Direction = Direction(0, 1);
const SOUTH: Direction = Direction(1, 0);
const WEST: Direction = Direction(0, -1);
const NORTH_WEST: Direction = Direction(-1, -1);
const NORTH_EAST: Direction = Direction(-1, 1);
const SOUTH_EAST: Direction = Direction(1, 1);
const SOUTH_WEST: Direction = Direction(1, -1);
const DIRECTIONS_TO_CHECK: [(Direction, [Direction; 3]); 4] = [
    (NORTH, [NORTH_WEST, NORTH, NORTH_EAST]),
    (SOUTH, [SOUTH_WEST, SOUTH, SOUTH_EAST]),
    (WEST, [SOUTH_WEST, WEST, NORTH_WEST]),
    (EAST, [NORTH_EAST, EAST, SOUTH_EAST]),
];
const ALL_DIRECTIONS: [Direction; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

impl ElfCollection {
    fn parse(input: impl Iterator<Item = String>) -> ElfCollection {
        let mut elves = HashSet::new();
        for (i, row) in input.enumerate() {
            for (j, col) in row.chars().enumerate() {
                if col == '#' {
                    elves.insert(Position(
                        i32::try_from(i).unwrap(),
                        i32::try_from(j).unwrap(),
                    ));
                }
            }
        }
        ElfCollection { elves, round: 0 }
    }

    fn bounds(&self) -> ((i32, i32), (i32, i32)) {
        let min_i = self.elves.iter().map(|x| x.0).min().unwrap();
        let max_i = self.elves.iter().map(|x| x.0).max().unwrap();
        let min_j = self.elves.iter().map(|x| x.1).min().unwrap();
        let max_j = self.elves.iter().map(|x| x.1).max().unwrap();

        ((min_i, min_j), (max_i, max_j))
    }

    #[allow(unused)]
    fn print(&self) {
        let ((min_i, min_j), (max_i, max_j)) = self.bounds();
        for idx_i in 0..(max_i - min_i + 2) {
            for idx_j in 0..(max_j - min_j + 2) {
                let pos = Position(min_i + idx_i, min_j + idx_j);
                if self.elves.contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    fn contained_area(&self) -> i32 {
        let elves_count = self.elves.len();
        let ((min_i, min_j), (max_i, max_j)) = self.bounds();
        (1 + max_j - min_j) * (1 + max_i - min_i) - i32::try_from(elves_count).unwrap()
    }

    fn iterate_once(&mut self) -> bool {
        let mut elf_intentions = HashMap::new();
        let mut inverse_elf_intentions = HashMap::new();

        for elf in self.elves.iter().filter(|elf| {
            !ALL_DIRECTIONS
                .into_iter()
                .find(|d| self.elves.contains(&(**elf + *d)))
                .is_none()
        }) {
            let maybe_direction = (0..4)
                .map(|i| DIRECTIONS_TO_CHECK[(self.round + i) % 4])
                .find(|(_, neighbours_to_check)| {
                    neighbours_to_check
                        .iter()
                        .find(|d| self.elves.contains(&(*elf + **d)))
                        .is_none()
                })
                .map(|x| x.0);

            if let Some(direction) = maybe_direction {
                let elf_intention = *elf + direction;
                if inverse_elf_intentions.contains_key(&elf_intention) {
                    // There's more than one elf trying to go to the same position; remove the existing one.
                    let elf = inverse_elf_intentions.remove(&elf_intention).unwrap();
                    elf_intentions.remove(elf);
                } else {
                    elf_intentions.insert(elf, elf_intention);
                    inverse_elf_intentions.insert(elf_intention, elf);
                }
            }
        }
        let mut new_elves = HashSet::new();
        for elf in &self.elves {
            if let Some(new_elf_position) = elf_intentions.get(&elf) {
                new_elves.insert(*new_elf_position);
            } else {
                new_elves.insert(*elf);
            }
        }
        let has_changed = self.elves != new_elves;
        self.elves = new_elves;
        self.round += 1;

        has_changed
    }

    fn iterate_until_stable(&mut self) {
        while self.iterate_once() {}
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut elves = ElfCollection::parse(input);
    elves.print();
    for _ in 0..10 {
        elves.iterate_once();
    }
    Ok(elves.contained_area())
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let mut elves = ElfCollection::parse(input);
    elves.iterate_until_stable();
    Ok(elves.round)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const TINY_MAP: [&str;6] = [
        ".....",
        "..##.",
        "..#..",
        ".....",
        "..##.",
        ".....",
    ];

    #[rustfmt::skip]
    const SMALL_MAP: [&str; 7] = [
        "....#..",
        "..###.#",
        "#...#.#",
        ".#...##",
        "#.###..",
        "##.#.##",
        ".#..#..",
    ];

    #[test]
    fn it_parses_elves() {
        let elf_collection =
            ElfCollection::parse(TINY_MAP.iter().map(|x| String::from(*x)).into_iter());
        assert_eq!(
            elf_collection.elves,
            HashSet::from([
                Position(1, 2),
                Position(1, 3),
                Position(2, 2),
                Position(4, 2),
                Position(4, 3),
            ])
        );
    }

    #[test]
    fn it_iterates_elves() {
        let mut elf_collection =
            ElfCollection::parse(TINY_MAP.iter().map(|x| String::from(*x)).into_iter());
        elf_collection.iterate_once();
        assert_eq!(
            elf_collection.elves,
            HashSet::from([
                Position(0, 2),
                Position(0, 3),
                Position(2, 2),
                Position(4, 2),
                Position(3, 3),
            ])
        );
    }

    #[test]
    fn it_iterates_elves_twice() {
        let mut elf_collection =
            ElfCollection::parse(TINY_MAP.iter().map(|x| String::from(*x)).into_iter());
        elf_collection.iterate_once();
        elf_collection.iterate_once();
        assert_eq!(
            elf_collection.elves,
            HashSet::from([
                Position(1, 2),
                Position(1, 3),
                Position(2, 1),
                Position(5, 2),
                Position(3, 4),
            ])
        );
    }

    #[test]
    fn it_iterates_elves_thrice() {
        let mut elf_collection =
            ElfCollection::parse(TINY_MAP.iter().map(|x| String::from(*x)).into_iter());
        elf_collection.iterate_once();
        elf_collection.iterate_once();
        elf_collection.iterate_once();
        assert_eq!(
            elf_collection.elves,
            HashSet::from([
                Position(0, 2),
                Position(1, 4),
                Position(2, 0),
                Position(3, 4),
                Position(5, 2),
            ])
        );
    }

    #[test]
    fn it_iterates_until_stable() {
        let mut elf_collection =
            ElfCollection::parse(TINY_MAP.iter().map(|x| String::from(*x)).into_iter());
        elf_collection.iterate_until_stable();
        assert_eq!(
            elf_collection.elves,
            HashSet::from([
                Position(0, 2),
                Position(1, 4),
                Position(2, 0),
                Position(3, 4),
                Position(5, 2),
            ])
        );
    }

    #[test]
    fn it_runs_a() {
        let input = SMALL_MAP.map(String::from).into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 110);
    }

    #[test]
    fn it_runs_b() {
        let input = SMALL_MAP.map(String::from).into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 20);
    }
}
