use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::error::Error;

use self::blizzard::BlizzardHistory;
use self::position::{Direction, Position, EAST, NORTH, SOUTH, WEST};

mod blizzard;
mod position;

#[derive(PartialEq, Eq, Debug)]
struct Map {
    start: Position,
    end: Position,
    blizzard_history: BlizzardHistory,
    width: i32,
    height: i32,
}

struct Candidate(i32, usize, Position);

impl Eq for Candidate {}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn create(blizzard_history: BlizzardHistory) -> Self {
        let start = Position(0, 1);
        let end = Position(
            blizzard_history.map_height - 1,
            blizzard_history.map_width - 2,
        );
        Map {
            start,
            end,
            width: blizzard_history.map_width,
            height: blizzard_history.map_height,
            blizzard_history: blizzard_history,
        }
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        (position == &self.start || position == &self.end)
            || (position.1 > 0
                && position.1 < self.width - 1
                && position.0 > 0
                && position.0 < self.height - 1)
    }

    /// This doesn't seem to work. :(
    ///
    /// The number of states to check grows exponentially, and there's no way it
    /// can find a route in the large case, which is at least 35 + 100 squares.
    fn solve_bfs(&mut self) -> usize {
        let mut visited: HashSet<(usize, Position)> = HashSet::new();
        let mut available_moves = VecDeque::from([(0, self.start)]);

        let mut turns_so_far = 0;

        while let Some((turn, position)) = available_moves.pop_front() {
            if position == self.end {
                return turn;
            }
            if turn > turns_so_far {
                turns_so_far = turn;
                println!("Iterating on turn: {turn}");
            }
            let next_positions = [NORTH, EAST, SOUTH, WEST, Direction(0, 0)]
                .into_iter()
                .map(|d| (position + d))
                .filter(|p| !visited.contains(&(turn + 1, *p)))
                .filter(|p| self.is_in_bounds(p))
                .filter(|p| !self.blizzard_history.has_blizzard_at(turn + 1, p));
            for next_position in next_positions {
                available_moves.push_back((turn + 1, next_position));
            }
            visited.insert((turn, position));
        }
        panic!("Could not find solution")
    }

    // Alternative approach: DFS.
    // Prioritise checking those routes which are closer to the end.
    fn solve_dfs(&mut self) -> usize {
        let mut visited: HashSet<(usize, Position)> = HashSet::new();

        let mut best_priority = -self.start.distance(&self.end);
        let mut available_moves = BinaryHeap::from([Candidate(best_priority, 0, self.start)]);

        // For debugging
        let mut attempts = 0;

        while let Some(Candidate(priority, turn, position)) = available_moves.pop() {
            if position == self.end {
                println!("Attempts: {attempts}");
                return turn;
            }
            attempts += 1;
            if priority < best_priority {
                best_priority = priority;
                println!("Starting to check priority: {priority} - attempt {attempts}");
            }
            let next_positions = [NORTH, EAST, SOUTH, WEST, Direction(0, 0)]
                .into_iter()
                .map(|d| (position + d))
                .filter(|p| !visited.contains(&(turn + 1, *p)))
                .filter(|p| self.is_in_bounds(p))
                .filter(|p| !self.blizzard_history.has_blizzard_at(turn + 1, p));
            for next_position in next_positions {
                let next_turn = turn + 1;
                let next_priority = -(next_turn as i32) - next_position.distance(&self.end);
                available_moves.push(Candidate(next_priority, next_turn, next_position))
            }
            visited.insert((turn, position));
        }
        panic!("Could not find solution")
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let mut map = Map::create(BlizzardHistory::parse(input));
    let turns_to_solve = map.solve_dfs();
    Ok(turns_to_solve)
}

#[allow(unused_variables)]
pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    todo!("Solution for part b not yet implemented");
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
    fn it_creates_a_map() {
        let input = MAP.map(String::from).into_iter();
        let blizzard_history = BlizzardHistory::parse(input);
        let result = Map::create(blizzard_history);
        assert_eq!(result.start, Position(0, 1));
        assert_eq!(result.end, Position(7, 6));
        assert_eq!(result.width, 7);
        assert_eq!(result.height, 8);
    }

    #[test]
    #[ignore]
    fn it_runs_a() {
        let input = ["aaaaa", "bbbbb"].map(String::from).into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    #[ignore]
    fn it_runs_b() {
        let input = ["aaaaa", "bbbbb"].map(String::from).into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 2);
    }
}
