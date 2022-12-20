use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

// const PATH: &str = "./small.txt";
const PATH: &str = "./large.txt";

fn get_score_char(char: &char) -> u32 {
    // println!("{}", char);
    if char.is_ascii_uppercase() {
        char.to_digit(36).unwrap() + 17
    } else {
        char.to_digit(36).unwrap() - 9
    }
}

fn get_count_a(input: &str) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let length = line.chars().count();
        let (str1, str2) = line.split_at(length / 2);

        let mut hash_set1 = HashSet::new();
        let mut hash_set2 = HashSet::new();

        for c in str1.chars() {
            hash_set1.insert(c);
        }
        for c in str2.chars() {
            hash_set2.insert(c);
        }
        let intersect = hash_set1.intersection(&hash_set2).next().unwrap();
        // println!("{} has score: {}", intersect, get_score(intersect));
        count += get_score_char(intersect);
    }
    count
}

fn get_count_b(input: &str) -> u32 {
    let mut count = 0;

    for mut lines in &input.lines().chunks(3) {
        let mut hash_set1 = HashSet::new();
        let mut hash_set2 = HashSet::new();
        let mut hash_set3 = HashSet::new();

        for c in lines.next().unwrap().chars() {
            hash_set1.insert(c);
        }
        for c in lines.next().unwrap().chars() {
            hash_set2.insert(c);
        }
        for c in lines.next().unwrap().chars() {
            hash_set3.insert(c);
        }

        let int1: HashSet<char> = hash_set1.intersection(&hash_set2).copied().collect();
        let mut int2 = int1.intersection(&hash_set3);
        count += get_score_char(&int2.next().unwrap());
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(PATH).unwrap();

    println!("Value of count 1: {}", get_count_a(&input));
    println!("Value of count 2: {}", get_count_b(&input));
    Ok(())
}
