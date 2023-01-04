use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    vec,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Direction(i32, i32);

impl Direction {
    fn parse(x: char) -> Result<Direction, Box<dyn Error>> {
        match x {
            '<' => Ok(Direction(0, -1)),
            '>' => Ok(Direction(0, 1)),
            _ => Err("Invalid character".into()),
        }
    }

    fn reverse(self) -> Self {
        Direction(-self.0, -self.1)
    }
}

fn parse_directions(
    input: impl Iterator<Item = String>,
) -> Result<Loop<Direction>, Box<dyn Error>> {
    Ok(input
        .last()
        .unwrap()
        .chars()
        .map(Direction::parse)
        .collect::<Result<Vec<Direction>, _>>()?
        .into())
}

struct Loop<T> {
    idx: usize,
    vec: Vec<T>,
}

impl<T: Copy> Iterator for Loop<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.vec[self.idx];
        self.idx = (self.idx + 1) % self.vec.len();
        Some(next)
    }
}

impl<T> From<Vec<T>> for Loop<T> {
    fn from(vec: Vec<T>) -> Self {
        Loop { idx: 0, vec }
    }
}

#[derive(Debug)]
struct Shape {
    coords: Vec<(i32, i32)>,
}

// +---> x
// |
// |
// v y
// (y, x)
lazy_static! {
    static ref SHAPE_COORDS: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 3), (0, 4), (0, 5), (0, 6)],
        vec![(-2, 4), (-1, 3), (-1, 4), (-1, 5), (0, 4)],
        vec![(-2, 5), (-1, 5), (0, 3), (0, 4), (0, 5)],
        vec![(-3, 3), (-2, 3), (-1, 3), (0, 3)],
        vec![(-1, 3), (-1, 4), (0, 3), (0, 4)],
    ];
}

impl Shape {
    fn new(shape_idx: usize) -> Shape {
        let mut coords = Vec::new();
        for (y, x) in SHAPE_COORDS[shape_idx].iter() {
            coords.push((*y, *x));
        }
        Shape { coords }
    }

    fn move_direction(&mut self, direction: &Direction) {
        let l = self.coords.len();
        for idx in 0..l {
            self.coords[idx].0 = self.coords[idx].0 + direction.0;
            self.coords[idx].1 = self.coords[idx].1 + direction.1;
        }
    }
}

struct Chamber {
    highest: i32,
    filled_space: HashSet<(i32, i32)>,
    directions: Loop<Direction>,
    shape_idx: usize,
}

impl Chamber {
    fn new(directions: Loop<Direction>) -> Chamber {
        let filled_space = HashSet::from([(4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7)]);
        Chamber {
            highest: 0,
            filled_space,
            directions,
            shape_idx: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut relevant_height = 0;
        for (i, _) in &self.filled_space {
            relevant_height = relevant_height.max(*i);
        }
        for y in 0..(relevant_height + 1) {
            let mut line = String::new();
            for x in 0..9 {
                if x == 0 || x == 8 {
                    line.push('|');
                } else if self.filled_space.contains(&(y, x)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
        println!("");
    }

    fn collides(&self, shape: &Shape) -> bool {
        for coords in &shape.coords {
            if self.filled_space.contains(coords) {
                return true;
            }
            if coords.1 == 0 || coords.1 == 8 {
                return true;
            }
        }
        false
    }

    fn move_shape_until_collision(&mut self, mut shape: Shape) -> Shape {
        loop {
            let direction = self.directions.next().unwrap();
            shape.move_direction(&direction);

            if self.collides(&shape) {
                shape.move_direction(&direction.reverse());
            }

            shape.move_direction(&Direction(1, 0));
            if self.collides(&shape) {
                shape.move_direction(&Direction(-1, 0));
                return shape;
            }
        }
    }

    /// Saves the shape into `self.filled_space`, and returns the amount by
    /// which this shape has increased the height of the tower.
    fn save_shape(&mut self, shape: Shape) -> i32 {
        let mut new_min_coord = 4;
        for coords in &shape.coords {
            self.filled_space.insert(*coords);
            new_min_coord = new_min_coord.min(coords.0);
        }
        let height_gained = 4 - new_min_coord;
        return height_gained;
    }

    /// Adjusts the grid coordinates so that the next shape always appears with
    /// its lowest y-coordinate at 0.
    fn shift_coords_up(&mut self, height_gained: i32) {
        self.highest += height_gained;
        let mut new_filled_space = HashSet::new();
        for (y, x) in &self.filled_space {
            new_filled_space.insert((y + height_gained, *x));
        }
        self.filled_space = new_filled_space;
    }

    /// Uses a breadth-first search to remove all the filled squares which it's
    /// now impossible for a shape to collide with.
    fn clear_inaccessible(&mut self) {
        let mut open_spaces = VecDeque::from([(0, 1)]);
        let mut new_filled_space = HashSet::new();
        let mut new_relevant_height = 0;

        let mut visited = HashSet::new();
        while let Some((y, x)) = open_spaces.pop_front() {
            for space in [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)] {
                if visited.contains(&space) {
                    continue;
                }
                visited.insert(space);
                if self.filled_space.contains(&space) {
                    new_filled_space.insert(space);
                    new_relevant_height = new_relevant_height.max(space.0);
                } else if space.1 > 0 && space.1 < 8 && space.0 > 0 {
                    open_spaces.push_back(space);
                }
            }
        }
        self.filled_space = new_filled_space;
    }

    /// Moves the shape until it's collided with the floor or an existing shape,
    /// and then shifts and simplifies the coords to the simplest
    /// representation.
    fn drop_shape(&mut self) {
        let mut shape = Shape::new(self.shape_idx);
        self.shape_idx = (self.shape_idx + 1) % 5;
        shape = self.move_shape_until_collision(shape);
        let height_gained = self.save_shape(shape);
        self.shift_coords_up(height_gained);
        self.clear_inaccessible();
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let directions = parse_directions(input)?;
    let mut chamber = Chamber::new(directions);

    for _ in 0..2022 {
        chamber.drop_shape();
    }
    Ok(chamber.highest)
}

/// Iterates until it finds a cycle: a repetition of the same shape index and
/// direction index where all the accessible filled squares are the same.
fn find_cycle(chamber: &mut Chamber) -> (i64, i64, i64, i64) {
    let mut chamber_history: Vec<(usize, usize, HashSet<(i32, i32)>, i32)> = Vec::new();

    loop {
        chamber.drop_shape();
        let last_occurrence = chamber_history.iter().find_position(|x| {
            x.0 == chamber.shape_idx && x.1 == chamber.directions.idx && x.2 == chamber.filled_space
        });
        if let Some((idx, last_occurrence)) = last_occurrence {
            return (
                (idx).try_into().unwrap(),
                (chamber_history.len()).try_into().unwrap(),
                last_occurrence.3.try_into().unwrap(),
                chamber.highest.try_into().unwrap(),
            );
        }
        chamber_history.push((
            chamber.shape_idx,
            chamber.directions.idx,
            chamber.filled_space.clone(),
            chamber.highest,
        ));
    }
}

// static TARGET: i64 = 2022;
static TARGET: i64 = 1000000000000;

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let directions = parse_directions(input)?;
    let mut chamber = Chamber::new(directions);

    let (cycle_start_idx, cycle_end_idx, cycle_start_height, cycle_end_height) =
        find_cycle(&mut chamber);

    let cycles_to_skip = (TARGET - cycle_end_idx) / (cycle_end_idx - cycle_start_idx) - 1;
    let height_delta_of_skipped_cycles = (cycle_end_height - cycle_start_height) * cycles_to_skip;
    let remaining_shape_drops =
        TARGET - cycles_to_skip * (cycle_end_idx - cycle_start_idx) - cycle_end_idx - 1;

    for _ in 0..remaining_shape_drops {
        chamber.drop_shape();
    }
    Ok(height_delta_of_skipped_cycles + i64::from(chamber.highest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_a() {
        let input = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"]
            .map(String::from)
            .into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 3068);
    }

    #[test]
    fn it_runs_b() {
        let input = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"]
            .map(String::from)
            .into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 1514285714288);
    }
}
