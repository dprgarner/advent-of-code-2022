use std::{env, fs};

mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;
mod aoc_07;

struct Problem {
    number: String,
    part: char,
    size: String,
}

impl Problem {
    fn from_args() -> Option<Problem> {
        let number: String = env::args().nth(1)?;
        let part = env::args().nth(2)?.chars().next()?;
        let size = env::args().nth(3)?;

        Some(Problem { number, part, size })
    }
}

fn main() {
    let problem = Problem::from_args().expect("Usage:\n  cargo run -- 06 a small");

    let path = format!("./input/{}-{}.txt", problem.number, problem.size);

    let input_txt = fs::read_to_string(&path).expect("Could not find input file");
    let input = input_txt.lines().map(|x| x.to_string());

    let problem_pair: (&str, &char) = (&problem.number, &problem.part);
    match problem_pair {
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
        ("07", 'a') => println!("a soln: {}", aoc_07::solve_a(input).unwrap()),
        ("07", 'b') => println!("b soln: {}", aoc_07::solve_b(input).unwrap()),
        _ => panic!("Unrecognised problem and solution"),
    }
}