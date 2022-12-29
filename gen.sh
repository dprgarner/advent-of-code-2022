#!/bin/bash -e

# Usage: ./gen 03

touch input/$1-small.txt
touch input/$1-large.txt
cat > src/aoc_$1.rs << EOM
pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    todo!("Not implemented a")
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    todo!("Not implemented a")
}

EOM

export MATCH_1="        (\"$1\", 'a') => println!(\"a soln: {}\", aoc_$1::solve_a(input).unwrap()),";
export MATCH_2="        (\"$1\", 'b') => println!(\"b soln: {}\", aoc_$1::solve_b(input).unwrap()),";
cat src/main.rs \
    | tr '\n' '🌶' \
    | sed "s/🌶🌶struct Problem/🌶mod aoc_$1;🌶🌶struct Problem/g" \
    | sed "s/\([^🌶]*Unrecognised problem and solution\)/${MATCH_1}🌶\1/g" \
    | sed "s/\([^🌶]*Unrecognised problem and solution\)/${MATCH_2}🌶\1/g" \
    | tr '🌶' '\n' \
    > src/main2.rs
mv src/main2.rs src/main.rs

echo "Created $1. To run:"
echo "  cargo run $1 a small"
