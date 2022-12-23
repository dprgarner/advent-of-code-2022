use core::panic;
use regex::Regex;
use std::error::Error;
use std::{env, io};

fn parse_crates(lines: &Vec<String>) -> Vec<Vec<char>> {
    let re = Regex::new(r"(?:\[(.)\]|(    ))").unwrap();

    let mut crates: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        crates.push(Vec::new());
    }

    for line in lines {
        for (idx, cap) in re.captures_iter(&line).enumerate() {
            if let Some(char_str) = cap.get(1) {
                let c = char_str.as_str().chars().next().expect("Must be a char");
                crates[idx].insert(0, c)
            }
        }
    }
    crates
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    origin: usize,
    destination: usize,
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let regex_match = re.captures(line)?;

        Some(Instruction {
            count: regex_match
                .get(1)?
                .as_str()
                .parse()
                .expect("Could not parse integer"),
            origin: regex_match
                .get(2)?
                .as_str()
                .parse()
                .expect("Could not parse integer"),
            destination: regex_match
                .get(3)?
                .as_str()
                .parse()
                .expect("Could not parse integer"),
        })
    }
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        let maybe_instruction = Instruction::parse(line);
        if let Some(instruction) = maybe_instruction {
            instructions.push(instruction);
        }
    }
    instructions
}

fn get_last_crates(crates: &Vec<Vec<char>>) -> String {
    let last_crates: String = crates
        .iter()
        .map(|crate_pile| {
            crate_pile
                .last()
                .or(Some(&' '))
                .expect("Not a char")
                .to_string()
        })
        .reduce(|acc, item| format!("{}{}", acc, item))
        .expect("Empty array");
    last_crates
}

fn solve_a(input: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let lines: Vec<String> = input.collect();
    let mut crates = parse_crates(&lines);
    let instructions = parse_instructions(&lines);

    for instruction in instructions {
        for _ in 0..instruction.count {
            let moved_crate = crates[instruction.origin - 1]
                .pop()
                .expect("Attempted to move from empty stack");
            crates[instruction.destination - 1].push(moved_crate);
        }
    }

    Ok(get_last_crates(&crates))
}

fn solve_b(input: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let lines: Vec<String> = input.collect();
    let mut crates = parse_crates(&lines);
    let instructions = parse_instructions(&lines);

    for instruction in instructions {
        let origin_stack = &crates[instruction.origin - 1];
        let mut idx: usize = origin_stack.len().into();
        idx -= instruction.count;
        let moved_crates: Vec<char> = Vec::from(&origin_stack[idx..]);
        crates[instruction.destination - 1].extend(moved_crates);
        crates[instruction.origin - 1].truncate(idx);
    }

    Ok(get_last_crates(&crates))
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
