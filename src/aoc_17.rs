use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    vec,
};

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

impl<T> Loop<T> {
    fn len(&self) -> usize {
        self.vec.len()
    }
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

    /// The largest y-coordinate of a shape or floor that a new shape could
    /// potentially collide with.
    relevant_height: i32,
}

impl Chamber {
    fn new(directions: Loop<Direction>) -> Chamber {
        let filled_space = HashSet::from([(4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7)]);
        Chamber {
            highest: 0,
            filled_space,
            directions,
            shape_idx: 0,
            relevant_height: 4,
        }
    }

    fn print(&self) {
        for y in 0..(self.relevant_height + 1) {
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

    fn clear_irrelevant(&mut self) {
        // Maze-solving. Remove all the squares that it's now impossible to
        // collide with.
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
        self.relevant_height = new_relevant_height;
    }

    fn save_shape(&mut self, shape: &Shape) {
        let mut new_min_coord = 4;
        for coords in &shape.coords {
            self.filled_space.insert(*coords);
            new_min_coord = new_min_coord.min(coords.0);
        }
        let height_gained = 4 - new_min_coord;
        self.highest += height_gained;

        let mut new_filled_space = HashSet::new();
        for (y, x) in &self.filled_space {
            new_filled_space.insert((y + height_gained, *x));
        }
        self.filled_space = new_filled_space;
        self.relevant_height = height_gained;

        self.clear_irrelevant();
    }

    fn drop_shape(&mut self) {
        let mut shape = Shape::new(self.shape_idx);
        self.shape_idx = (self.shape_idx + 1) % 5;

        loop {
            let direction = self.directions.next().unwrap();
            shape.move_direction(&direction);

            if self.collides(&shape) {
                shape.move_direction(&direction.reverse());
            }

            shape.move_direction(&Direction(1, 0));
            if self.collides(&shape) {
                shape.move_direction(&Direction(-1, 0));
                self.save_shape(&shape);
                return;
            }
        }
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

// fn find_cycle(chamber: &mut Chamber) -> (usize, usize, i32) {
//     // Search for a cycle: a place where the same shape is next and the top-most
//     // shapes are the same. The easiest way to find this is is to look for when
//     // the top-most elements are a row.
//     let mut filled_row = None;
//     let mut idx = 0;

//     // Argh. This approach only works for the large soln.
//     loop {
//         chamber.drop_shape();
//         if (1..8).all(|x| chamber.filled_space.contains(&(chamber.highest, x))) {
//             if let Some((last_idx, last_shape_idx, last_highest)) = filled_row {
//                 // It's only a cycle if it starts and ends on the same shape.
//                 assert_eq!(last_shape_idx, chamber.shape_idx);
//                 let height_change = chamber.highest - last_highest;
//                 return (last_idx, idx, height_change);
//             } else {
//                 filled_row = Some((idx, chamber.shape_idx, chamber.highest));
//             }
//         }
//         idx += 1;
//     }
// }

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let directions = parse_directions(input)?;
    let mut chamber = Chamber::new(directions);

    // Search for a cycle where the state is the same.
    // let (start_idx, end_idx, height_change) = find_cycle(&mut chamber);
    // dbg!(start_idx, end_idx, height_change);
    // let cycle_length = end_idx - start_idx;

    // let complete_cycles: i64 = 1000000000000 / (cycle_length as i64);
    // dbg!(complete_cycles);
    // let height_change = complete_cycles
    // idx: 1546
    // highest: 24245310

    // Ok(cycle_length)
    Ok(1)
    // Ok(chamber.highest)
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
    #[ignore]
    fn it_runs_b() {
        let input = ["line 1", "line 2"].map(String::from).into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 2);
    }
}
