use itertools::Itertools;
use std::collections::HashSet;

struct Structure {
    filled_space: HashSet<(i32, i32)>,
    floor: i32,
}

impl Structure {
    fn parse(input: impl Iterator<Item = String>) -> Option<Structure> {
        let mut rocks = Vec::new();
        let mut max_y = 0;
        for input_line in input {
            let mut rock_line = Vec::new();
            for x in input_line.split(" -> ") {
                let (x, y) = x.split_once(",")?;
                let x: i32 = x.parse().ok()?;
                let y: i32 = y.parse().ok()?;
                rock_line.push((x, y));
                max_y = max_y.max(y);
            }
            rocks.push(rock_line);
        }

        let mut filled_space = HashSet::new();
        for rock_line in rocks {
            for (&(x1, y1), &(x2, y2)) in rock_line.iter().tuple_windows() {
                if x1 == x2 {
                    for y in y1.min(y2)..y1.max(y2) + 1 {
                        filled_space.insert((x1, y));
                    }
                } else if y1 == y2 {
                    for x in x1.min(x2)..x1.max(x2) + 1 {
                        filled_space.insert((x, y1));
                    }
                } else {
                    return None;
                }
            }
        }

        Some(Structure {
            filled_space,
            floor: max_y + 2,
        })
    }

    fn _print(&self) {
        for y in 0..20 {
            let mut line = String::new();
            for x in 450..550 {
                if (x, y) == (500, 0) {
                    line.push('+');
                } else if self.filled_space.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
    }

    fn drop_sand_grain(&self) -> Option<(i32, i32)> {
        let mut x: i32 = 500;
        let mut y: i32 = 0;
        while y < self.floor {
            if !self.filled_space.contains(&(x, y + 1)) {
                y += 1;
            } else if !self.filled_space.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !self.filled_space.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                if (x, y) == (500, 0) {
                    return None;
                }
                return Some((x, y));
            }
        }
        return None;
    }

    fn pour_sand(&mut self) -> usize {
        let mut iterations = 0;
        while let Some((x, y)) = self.drop_sand_grain() {
            self.filled_space.insert((x, y));
            iterations += 1;
        }
        iterations
    }

    fn add_floor(&mut self) {
        let min_x = 500 - self.floor - 15;
        let max_x = 500 + self.floor + 15;
        for x in min_x..max_x {
            self.filled_space.insert((x, self.floor));
        }
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let mut structure = Structure::parse(input).ok_or("Input should be parseable")?;
    let iterations = structure.pour_sand();
    Ok(iterations)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let mut structure = Structure::parse(input).ok_or("Input should be parseable")?;
    structure.add_floor();
    let iterations = 1 + structure.pour_sand();
    Ok(iterations)
}
