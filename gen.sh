#!/bin/bash -e

# Usage: ./gen 03

touch input/$1-small.txt
touch input/$1-large.txt
cat > src/aoc_$1.rs << EOM
pub fn solve_a(_input: impl Iterator<Item = String>) -> Result<String, &'static str> {
    todo!("Solution for part a not yet implemented");
}

pub fn solve_b(_input: impl Iterator<Item = String>) -> Result<String, &'static str> {
    todo!("Solution for part b not yet implemented");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_a() -> Result<(), String> {
        let input = ["line 1", "line 2"].map(String::from).into_iter();
        let result = solve_a(input)?;
        assert_eq!(result, "a");
        Ok(())
    }

    #[test]
    #[ignore]
    fn it_runs_b() -> Result<(), String> {
        let input = [String::from("abc")];
        let result = solve_b(input.into_iter())?;
        assert_eq!(result, "b");
        Ok(())
    }
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
