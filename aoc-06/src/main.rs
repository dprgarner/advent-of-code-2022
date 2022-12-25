use std::collections::HashSet;
use std::{env, io};

fn find_window(line: &str, length: usize) -> Option<usize> {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..(chars.len() - length) {
        let uniq_chars: HashSet<&char> = (&chars[i..i + length]).iter().collect();
        if uniq_chars.len() == length {
            return Some(i + length);
        }
    }
    None
}

fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let line = input.into_iter().next().expect("No input!");
    find_window(&line, 4).ok_or("Could not find char group")
}

fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let line = input.into_iter().next().expect("No input!");
    find_window(&line, 14).ok_or("Could not find char group")
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
