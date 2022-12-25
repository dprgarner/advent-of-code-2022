use std::error::Error;

fn get_best_elves(input: impl Iterator<Item = String>) -> Option<Vec<i32>> {
    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf_total = 0;

    for line in input {
        if line == "" {
            elves.push(current_elf_total);
            current_elf_total = 0;
        } else {
            let cargo: i32 = line.parse().ok()?;
            current_elf_total += cargo;
        }
    }
    elves.push(current_elf_total);
    elves.sort_unstable_by(|a, b| b.cmp(a));

    // for elf in &elves {
    //     println!("{}", &elf);
    // }
    Some(elves.get(0..3)?.to_vec())
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let best_elves = get_best_elves(input);
    let best_elf = best_elves.unwrap().first().unwrap().clone();
    Ok(best_elf)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let best_elves = get_best_elves(input);
    let total: i32 = best_elves.unwrap().get(0..3).unwrap().iter().sum();
    Ok(total)
}
