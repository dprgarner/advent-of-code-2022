#[macro_use]
extern crate lazy_static;

use std::{env, fs};

mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;
mod aoc_07;
mod aoc_08;
mod aoc_09;
mod aoc_10;
mod aoc_11;
mod aoc_12;
mod aoc_13;
mod aoc_14;
mod aoc_15;
mod aoc_16;
mod aoc_17;
mod aoc_18;
mod aoc_19;
mod aoc_20;
mod aoc_21;
mod aoc_22;
mod aoc_23;
mod aoc_24;
mod aoc_25;

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
        ("08", 'a') => println!("a soln: {}", aoc_08::solve_a(input).unwrap()),
        ("08", 'b') => println!("b soln: {}", aoc_08::solve_b(input).unwrap()),
        ("09", 'a') => println!("a soln: {}", aoc_09::solve_a(input).unwrap()),
        ("09", 'b') => println!("b soln: {}", aoc_09::solve_b(input).unwrap()),
        ("10", 'a') => println!("a soln: {}", aoc_10::solve_a(input).unwrap()),
        ("10", 'b') => println!("b soln: {}", aoc_10::solve_b(input).unwrap()),
        ("11", 'a') => println!("a soln: {}", aoc_11::solve_a(input).unwrap()),
        ("11", 'b') => println!("b soln: {}", aoc_11::solve_b(input).unwrap()),
        ("12", 'a') => println!("a soln: {}", aoc_12::solve_a(input).unwrap()),
        ("12", 'b') => println!("b soln: {}", aoc_12::solve_b(input).unwrap()),
        ("13", 'a') => println!("a soln: {}", aoc_13::solve_a(input).unwrap()),
        ("13", 'b') => println!("b soln: {}", aoc_13::solve_b(input).unwrap()),
        ("14", 'a') => println!("a soln: {}", aoc_14::solve_a(input).unwrap()),
        ("14", 'b') => println!("b soln: {}", aoc_14::solve_b(input).unwrap()),
        ("15", 'a') => println!("a soln: {}", aoc_15::solve_a(input).unwrap()),
        ("15", 'b') => println!("b soln: {}", aoc_15::solve_b(input).unwrap()),
        ("16", 'a') => println!("a soln: {}", aoc_16::solve_a(input).unwrap()),
        ("16", 'b') => println!("b soln: {}", aoc_16::solve_b(input).unwrap()),
        ("17", 'a') => println!("a soln: {}", aoc_17::solve_a(input).unwrap()),
        ("17", 'b') => println!("b soln: {}", aoc_17::solve_b(input).unwrap()),
        ("18", 'a') => println!("a soln: {}", aoc_18::solve_a(input).unwrap()),
        ("18", 'b') => println!("b soln: {}", aoc_18::solve_b(input).unwrap()),
        ("19", 'a') => println!("a soln: {}", aoc_19::solve_a(input).unwrap()),
        ("19", 'b') => println!("b soln: {}", aoc_19::solve_b(input).unwrap()),
        ("20", 'a') => println!("a soln: {}", aoc_20::solve_a(input).unwrap()),
        ("20", 'b') => println!("b soln: {}", aoc_20::solve_b(input).unwrap()),
        ("21", 'a') => println!("a soln: {}", aoc_21::solve_a(input).unwrap()),
        ("21", 'b') => println!("b soln: {}", aoc_21::solve_b(input).unwrap()),
        ("22", 'a') => println!("a soln: {}", aoc_22::solve_a(input).unwrap()),
        ("22", 'b') => println!("b soln: {}", aoc_22::solve_b(input).unwrap()),
        ("23", 'a') => println!("a soln: {}", aoc_23::solve_a(input).unwrap()),
        ("23", 'b') => println!("b soln: {}", aoc_23::solve_b(input).unwrap()),
        ("24", 'a') => println!("a soln: {}", aoc_24::solve_a(input).unwrap()),
        ("24", 'b') => println!("b soln: {}", aoc_24::solve_b(input).unwrap()),
        ("25", 'a') => println!("a soln: {}", aoc_25::solve_a(input).unwrap()),
        ("25", 'b') => println!("b soln: {}", aoc_25::solve_b(input).unwrap()),
        _ => panic!("Unrecognised problem and solution"),
    }
}
