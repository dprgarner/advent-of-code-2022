use std::error::Error;

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in input {
        for (str1, str2) in line.split_once(" ") {
            // println!("{}", str1);
            // println!("{}", str2);
            // A: rock, B: paper, C: scissors
            // X: rock, Y: paper, Z: scissors
            score += match (str1, str2) {
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
        }
    }
    Ok(score)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let mut score = 0;

    for line in input {
        for (str1, str2) in line.split_once(" ") {
            // A: rock, B: paper, C: scissors
            // X: lose, Y: draw, Z: win
            score += match (str1, str2) {
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
    Ok(score)
}
