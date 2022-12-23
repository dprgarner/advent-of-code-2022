#!/bin/bash -e

# Usage: ./gen 03

DIR=aoc-$1

cargo new $DIR
touch $DIR/small.txt
touch $DIR/large.txt
cat > "$DIR"/src/main.rs << EOF
use std::error::Error;
use std::{env, io};

fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    for line in input {
        todo!("{:?}", line);
    }
    Ok(1)
}

fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    for line in input {
        todo!("{:?}", line);
    }
    Ok(2)
}

fn get_problem() -> Option<char> {
    let arg = env::args().nth(1)?;
    let problem = arg.chars().next();
    problem
}

fn main() {
    let problem = get_problem().expect("Usage:\n  cargo run -- a");
    let input = io::stdin().lines().map(|line| line.expect("IO error"));

    match problem {
        'a' => println!("a soln: {}", solve_a(input).unwrap()),
        'b' => println!("b soln: {}", solve_b(input).unwrap()),
        _ => panic!("Unrecognised soln"),
    }
}

EOF

echo "Created $DIR. To run:"
echo "  cd $DIR"
echo "  cat small.txt | cargo run -- a"
echo "  cat large.txt | cargo run -- b"
