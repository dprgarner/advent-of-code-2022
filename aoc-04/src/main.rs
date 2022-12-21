use std::error::Error;
use std::io;

// const SOLVE: char = 'A';
const SOLVE: char = 'B';

fn parse_pairs<T: Iterator<Item = String>>(
    input: T,
) -> impl Iterator<Item = ((i32, i32), (i32, i32))> {
    input.map(|line| -> ((i32, i32), (i32, i32)) {
        let (first, second) = line.split_once(',').unwrap();
        let (a, b) = first.split_once('-').unwrap();
        let a_start: i32 = a.parse().unwrap();
        let a_end: i32 = b.parse().unwrap();

        let (c, d) = second.split_once('-').unwrap();
        let b_start: i32 = c.parse().unwrap();
        let b_end: i32 = d.parse().unwrap();

        ((a_start, a_end), (b_start, b_end))
    })
}

fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;
    for ((a_start, a_end), (b_start, b_end)) in parse_pairs(input) {
        if (a_start <= b_start && b_end <= a_end) || (b_start <= a_start && a_end <= b_end) {
            count += 1;
        }
    }
    Ok(count)
}

fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;
    for ((a_start, a_end), (b_start, b_end)) in parse_pairs(input) {
        if !((a_end < b_start) || (b_end < a_start)) {
            count += 1;
        }
    }
    Ok(count)
}

fn main() {
    let input = io::stdin().lines().map(|line| line.expect("IO error"));

    match SOLVE {
        'A' => println!("a soln: {}", solve_a(input).unwrap()),
        'B' => println!("b soln: {}", solve_b(input).unwrap()),
        _ => panic!("Unrecognised soln"),
    }
}
