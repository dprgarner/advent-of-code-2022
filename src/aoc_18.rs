use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Coord(i32, i32, i32);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Coord {
    fn parse(line: &str) -> Result<Coord, Box<dyn Error>> {
        let (x, y, z) = line
            .split(",")
            .collect_tuple::<(_, _, _)>()
            .ok_or("Could not split coords")?;
        Ok(Coord(x.parse()?, y.parse()?, z.parse()?))
    }

    fn neighbours(&self) -> [Coord; 6] {
        [
            Coord(&self.0 + 1, &self.1 + 0, &self.2 + 0),
            Coord(&self.0 + -1, &self.1 + 0, &self.2 + 0),
            Coord(&self.0 + 0, &self.1 + 1, &self.2 + 0),
            Coord(&self.0 + 0, &self.1 + -1, &self.2 + 0),
            Coord(&self.0 + 0, &self.1 + 0, &self.2 + 1),
            Coord(&self.0 + 0, &self.1 + 0, &self.2 + -1),
        ]
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Grid {
    cubes: HashSet<Coord>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
}

impl Grid {
    fn parse(input: impl Iterator<Item = String>) -> Result<Grid, Box<dyn Error>> {
        let mut cubes = HashSet::new();
        let mut x_min = 1;
        let mut x_max = 1;
        let mut y_min = 1;
        let mut y_max = 1;
        let mut z_min = 1;
        let mut z_max = 1;
        for line in input {
            let tuple = Coord::parse(&line)?;
            x_min = x_min.min(tuple.0);
            x_max = x_max.max(tuple.0);
            y_min = y_min.min(tuple.1);
            y_max = y_max.max(tuple.1);
            z_min = z_min.min(tuple.2);
            z_max = z_max.max(tuple.2);
            cubes.insert(tuple);
        }
        Ok(Grid {
            cubes,
            x_bounds: (x_min - 1, x_max + 1),
            y_bounds: (y_min - 1, y_max + 1),
            z_bounds: (z_min - 1, z_max + 1),
        })
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        coord.0 >= self.x_bounds.0
            && coord.0 <= self.x_bounds.1
            && coord.1 >= self.y_bounds.0
            && coord.1 <= self.y_bounds.1
            && coord.2 >= self.z_bounds.0
            && coord.2 <= self.z_bounds.1
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let grid = Grid::parse(input)?;

    let mut count = 0;
    for cube in &grid.cubes {
        for neighbour in cube.neighbours() {
            if !grid.cubes.contains(&neighbour) {
                count += 1
            }
        }
    }
    Ok(count)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let grid = Grid::parse(input)?;

    // Another BFS!
    assert!(!grid.cubes.contains(&Coord(0, 0, 0)));
    let mut count = 0;
    let mut open_spaces = VecDeque::from([Coord(0, 0, 0)]);
    let mut visited = HashSet::new();

    while let Some(coord) = open_spaces.pop_front() {
        for space in coord.neighbours() {
            if grid.cubes.contains(&space) {
                count += 1;
            } else if grid.in_bounds(&space) && !visited.contains(&space) {
                open_spaces.push_back(space);
            }
            visited.insert(space);
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_cubes() {
        let input = ["1,1,1", "2,1,1"].map(String::from).into_iter();
        let result = Grid::parse(input).unwrap();
        assert_eq!(
            result,
            Grid {
                cubes: HashSet::from([Coord(1, 1, 1), Coord(2, 1, 1)]),
                x_bounds: (0, 3),
                y_bounds: (0, 2),
                z_bounds: (0, 2),
            }
        );
    }

    #[test]
    fn it_runs_a_tiny() {
        let input = ["1,1,1", "2,1,1"].map(String::from).into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn it_runs_a() {
        let input = [
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ]
        .map(String::from)
        .into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 64);
    }

    #[test]
    fn it_runs_b_small() {
        let input = ["1,1,1", "2,1,1"].map(String::from).into_iter();

        let result = solve_b(input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn it_runs_b() {
        let input = [
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ]
        .map(String::from)
        .into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 58);
    }
}
