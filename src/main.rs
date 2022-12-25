use std::{env, io};

mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;

fn get_problem() -> Option<(String, char)> {
    let problem_number: String = env::args().nth(1)?;
    let problem_part = env::args().nth(2)?.chars().next()?;
    Some((problem_number, problem_part))
}

fn main() {
    let input = io::stdin().lines().map(|line| line.expect("IO error"));
    let problem = get_problem().expect("Usage:\n  cargo run -- a");
    let problem: (&str, char) = (&problem.0, problem.1);

    match problem {
        ("01", 'a') => println!("a soln: {}", aoc_01::solve_a(input).unwrap()),
        ("01", 'b') => println!("b soln: {}", aoc_01::solve_b(input).unwrap()),
        ("02", 'a') => println!("a soln: {}", aoc_02::solve_a(input).unwrap()),
        ("02", 'b') => println!("b soln: {}", aoc_02::solve_b(input).unwrap()),
        ("03", 'a') => println!("a soln: {}", aoc_03::solve_a(input).unwrap()),
        ("03", 'b') => println!("b soln: {}", aoc_03::solve_b(input).unwrap()),
        ("04", 'a') => println!("a soln: {}", aoc_04::solve_a(input).unwrap()),
        ("04", 'b') => println!("b soln: {}", aoc_04::solve_b(input).unwrap()),
        ("05", 'a') => println!("a soln: {}", aoc_05::solve_a(input).unwrap()),
        ("05", 'b') => println!("b soln: {}", aoc_05::solve_b(input).unwrap()),
        ("06", 'a') => println!("a soln: {}", aoc_06::solve_a(input).unwrap()),
        ("06", 'b') => println!("b soln: {}", aoc_06::solve_b(input).unwrap()),
        _ => panic!("Unrecognised problem and solution"),
    }
}
