#!/bin/bash -e

# Usage: ./gen 03

DIR=aoc-$1

cargo new $DIR
touch $DIR/small.txt
touch $DIR/large.txt
cat > "$DIR"/src/main.rs << EOF
use std::error::Error;
use std::fs;

const PATH: &str = "./small.txt";
// const PATH: &str = "./large.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(PATH).unwrap();

    for line in input.lines() {
        todo!("{line}");
    }

    Ok(())
}

EOF

echo "Created $DIR. To run:"
echo "  cd $DIR"
echo "  cargo run"
