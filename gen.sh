#!/bin/bash -e

# Usage: ./gen 03

touch input/$1-small.txt
touch input/$1-large.txt
cat > src/aoc-$1.rs << EOF
pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    todo!("Not implemented a")
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    todo!("Not implemented a")
}

EOF

echo "Created $1. To run:"
echo "  cargo run 04 a large"
echo "  cargo run 06 b small"
