use std::collections::VecDeque;

struct Map {
    heights: Vec<Vec<u32>>,
    start: (usize, usize),
    end: (usize, usize),
    neighbours: Vec<Vec<Vec<(usize, usize)>>>,
}

impl Map {
    /// The maze is solved backwards; a square's neighbours are the potential
    /// preceding squares in the route.
    fn get_neighbours(heights: &Vec<Vec<u32>>) -> Vec<Vec<Vec<(usize, usize)>>> {
        let mut neighbours = Vec::new();

        let row_count = heights.len();
        let col_count = heights[0].len();
        for i in 0..row_count {
            neighbours.push(Vec::new());
            for j in 0..col_count {
                neighbours[i].push(Vec::new());
                let mut to_check = Vec::new();
                if i > 0 {
                    to_check.push((i - 1, j))
                };
                if j > 0 {
                    to_check.push((i, j - 1));
                }
                if j < col_count - 1 {
                    to_check.push((i, j + 1));
                }
                if i < row_count - 1 {
                    to_check.push((i + 1, j))
                };
                for (p, q) in to_check {
                    if heights[p][q] >= heights[i][j] - 1 {
                        neighbours[i][j].push((p, q));
                    }
                }
            }
        }

        neighbours
    }

    fn parse(input: impl Iterator<Item = String>) -> Option<Map> {
        let mut start = None;
        let mut end = None;

        let mut heights = Vec::new();
        for (i, row) in input.enumerate() {
            heights.push(Vec::new());
            for (j, col) in row.chars().enumerate() {
                let mut digit = col.to_digit(36)? - 'a'.to_digit(36).unwrap() + 1;
                if col == 'S' {
                    start = Some((i, j));
                    digit = 1;
                } else if col == 'E' {
                    end = Some((i, j));
                    digit = 26;
                }
                heights[i].push(digit);
            }
        }
        let neighbours = Map::get_neighbours(&heights);

        Some(Map {
            heights,
            start: start?,
            end: end?,
            neighbours,
        })
    }

    fn solve<F>(&self, stop_condition: F) -> Option<usize>
    where
        F: Fn(&Self, usize, usize) -> bool,
    {
        let mut distances = Vec::new();
        for i in 0..self.heights.len() {
            distances.push(Vec::new());
            for _ in 0..self.heights[0].len() {
                distances[i].push(None);
            }
        }
        distances[self.end.0][self.end.1] = Some(0);

        let mut locations_to_try = VecDeque::new();
        locations_to_try.push_back(self.end.clone());

        while let Some((i, j)) = locations_to_try.pop_front() {
            let distance = distances[i][j].unwrap();

            if stop_condition(self, i, j) {
                return Some(distance);
            }

            for (p, q) in &self.neighbours[i][j] {
                if distances[*p][*q] == None {
                    distances[*p][*q] = Some(distance + 1);
                    locations_to_try.push_back((*p, *q));
                }
            }
        }
        None
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let map = Map::parse(input).ok_or("Could not parse map")?;
    let min_distance = map
        .solve(|map, i, j| (i, j) == map.start)
        .ok_or("Could not solve map")?;
    Ok(min_distance)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let map = Map::parse(input).ok_or("Could not parse map")?;
    let min_distance = map
        .solve(|map, i, j| map.heights[i][j] == 1)
        .ok_or("Could not solve map")?;
    Ok(min_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    static TEST_MAP: [&str; 5] = [
        "Sabqponm",
        "abcryxxl",
        "accszExk",
        "acctuvwj",
        "abdefghi",
    ];

    #[test]
    fn it_parses_heights() {
        let map = Map::parse(TEST_MAP.map(String::from).into_iter()).unwrap();
        assert_eq!(map.start, (0, 0));
        assert_eq!(map.end, (2, 5));
        #[rustfmt::skip]
        assert_eq!(
            map.heights,
            [
                [ 1,  1,  2, 17, 16, 15, 14, 13],
                [ 1,  2,  3, 18, 25, 24, 24, 12],
                [ 1,  3,  3, 19, 26, 26, 24, 11],
                [ 1,  3,  3, 20, 21, 22, 23, 10],
                [ 1,  2,  4,  5,  6,  7,  8,  9]
            ]
        );
    }

    #[test]
    fn it_calculates_neighbours() {
        let map = Map::parse(TEST_MAP.map(String::from).into_iter()).unwrap();
        assert_eq!(map.neighbours[0][0], [(0, 1), (1, 0)]);
        assert_eq!(map.neighbours[1][2], [(0, 2), (1, 1), (1, 3), (2, 2)]);
        assert_eq!(map.neighbours[3][3], [(2, 3), (3, 4)]);
        assert_eq!(map.neighbours[4][7], [(3, 7), (4, 6)]);
    }

    #[test]
    fn solves_a() {
        let a_soln = solve_a(TEST_MAP.map(String::from).into_iter()).unwrap();
        assert_eq!(a_soln, 31);
    }

    #[test]
    fn returns_no_soln_for_a() {
        #[rustfmt::skip]
        let no_soln_map = [
            "Sab",
            "aac",
            "acE",
        ];

        let a_no_soln = solve_a(no_soln_map.map(String::from).into_iter());
        assert_eq!(a_no_soln, Err("Could not solve map"));
    }

    #[test]
    fn solves_b() {
        let b_soln = solve_b(TEST_MAP.map(String::from).into_iter()).unwrap();
        assert_eq!(b_soln, 29);
    }
}
