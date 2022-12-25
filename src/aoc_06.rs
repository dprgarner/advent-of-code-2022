use std::collections::HashSet;

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

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let line = input.into_iter().next().expect("No input!");
    find_window(&line, 4).ok_or("Could not find char group")
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let line = input.into_iter().next().expect("No input!");
    find_window(&line, 14).ok_or("Could not find char group")
}
