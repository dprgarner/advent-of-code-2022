use std::error::Error;
use std::fs;

const PATH: &str = "./small.txt";
// const PATH: &str = "./large.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(PATH).unwrap();

    let mut strategy1_score = 0;
    let mut strategy2_score = 0;

    for line in input.lines() {
        for (str1, str2) in line.split_once(" ") {
            // println!("{}", str1);
            // println!("{}", str2);
            // A: rock, B: paper, C: scissors
            // X: rock, Y: paper, Z: scissors
            strategy1_score += match (str1, str2) {
                ("A", "X") => 3 + 1,
                ("A", "Y") => 6 + 2,
                ("A", "Z") => 0 + 3,
                ("B", "X") => 0 + 1,
                ("B", "Y") => 3 + 2,
                ("B", "Z") => 6 + 3,
                ("C", "X") => 6 + 1,
                ("C", "Y") => 0 + 2,
                ("C", "Z") => 3 + 3,
                _ => panic!("Unrecognised combination"),
            };

            // A: rock, B: paper, C: scissors
            // X: lose, Y: draw, Z: win
            strategy2_score += match (str1, str2) {
                ("A", "X") => 0 + 3,
                ("A", "Y") => 3 + 1,
                ("A", "Z") => 6 + 2,
                ("B", "X") => 0 + 1,
                ("B", "Y") => 3 + 2,
                ("B", "Z") => 6 + 3,
                ("C", "X") => 0 + 2,
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 6 + 1,
                _ => panic!("Unrecognised combination"),
            };
        }
    }
    println!("Strategy 1 score is {}", strategy1_score);
    println!("Strategy 2 score is {}", strategy2_score);
    Ok(())
}
