use std::error::Error;
use std::fs;

// const PATH: &str = "./small.txt";
const PATH: &str = "./large.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(PATH).unwrap();

    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf_total = 0;

    for line in input.lines() {
        if line == "" {
            elves.push(current_elf_total);
            current_elf_total = 0;
        } else {
            let cargo: i32 = line.parse()?;
            current_elf_total += cargo;
        }
    }
    elves.push(current_elf_total);
    elves.sort_unstable_by(|a, b| b.cmp(a));

    for elf in &elves {
        println!("{}", &elf);
    }

    println!("Best elf total: {}", elves.first().unwrap());
    let total: i32 = elves.get(0..3).unwrap().iter().sum();

    println!("Best three elves total: {}", total);
    Ok(())
}
