use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Direction(i32, i32);

struct Move(Direction, usize);

impl Direction {
    /// Returns the number of steps needed to traverse this direction vector.
    fn distance(&self) -> i32 {
        self.0.abs().max(self.1.abs())
    }

    /// Returns a single step in the direction of `&self`.
    fn to_single_step(&self) -> Direction {
        let h_step;
        let v_step;

        if self.0 > 0 {
            h_step = 1;
        } else if self.0 < 0 {
            h_step = -1;
        } else {
            h_step = 0;
        }

        if self.1 > 0 {
            v_step = 1;
        } else if self.1 < 0 {
            v_step = -1;
        } else {
            v_step = 0
        }

        Direction(h_step, v_step)
    }
}

fn parse_move(line: &str) -> Move {
    let (dir, count) = line
        .split_once(" ")
        .expect("Expected a space-separated string");
    let dir: Direction = match dir {
        "U" => Direction(0, 1),
        "D" => Direction(0, -1),
        "L" => Direction(-1, 0),
        "R" => Direction(1, 0),
        _ => panic!("Expected U, D, L, or R"),
    };
    let count = count.parse().expect("Expected a number");
    Move(dir, count)
}

struct Rope {
    tail_tip_visited: HashSet<Position>,
    head: Position,
    tail: Vec<Position>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position(i32, i32);

impl Position {
    fn add(&self, t2: &Direction) -> Position {
        Position(self.0 + t2.0, self.1 + t2.1)
    }

    fn distance(&self, t2: &Position) -> Direction {
        Direction(self.0 - t2.0, self.1 - t2.1)
    }

    /// Returns the new position when `self` follows `other`.
    fn follow(&self, other: &Self) -> Self {
        let displacement = other.distance(&self);
        if displacement.distance() > 1 {
            self.add(&displacement.to_single_step())
        } else {
            self.clone()
        }
    }
}

impl Rope {
    fn new(tail_length: usize) -> Self {
        let mut tail = Vec::new();
        for _ in 0..tail_length {
            tail.push(Position(0, 0));
        }
        Rope {
            tail_tip_visited: HashSet::from([Position(0, 0)]),
            head: Position(0, 0),
            tail,
        }
    }

    fn apply(&mut self, m: &Move) {
        let direction = &m.0;
        let count = m.1;
        for _ in 0..count {
            self.head = self.head.add(&direction);

            let mut to_follow = &self.head;
            for i in 0..self.tail.len() {
                let new_tail_pos = self.tail[i].follow(to_follow);
                self.tail[i] = new_tail_pos;
                to_follow = &self.tail[i];
            }
            self.tail_tip_visited
                .insert(self.tail.last().unwrap().clone());
        }
    }

    fn draw(&self) {
        let mut strs = Vec::new();
        for y in -25..25 {
            let mut s = String::new();
            for x in -25..25 {
                let coord = Position(x, y);
                if let Some(idx) = self.tail.iter().position(|x| x == &coord) {
                    let digit = (idx + 1).to_string().chars().next().unwrap();
                    s.push(digit);
                } else if coord == Position(0, 0) {
                    s.push('s');
                } else if coord == self.head {
                    s.push('H')
                } else if self.tail_tip_visited.contains(&coord) {
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            strs.insert(0, s);
        }
        let output = strs.join("\n");
        println!("{}", &output);
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let mut rope = Rope::new(1);
    for m in input.map(|line| parse_move(&line)) {
        rope.apply(&m);
    }
    let count = rope.tail_tip_visited.len();
    rope.draw();
    Ok(count)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let mut rope = Rope::new(9);
    for m in input.map(|line| parse_move(&line)) {
        rope.apply(&m);
    }
    let count = rope.tail_tip_visited.len();
    rope.draw();
    Ok(count)
}
