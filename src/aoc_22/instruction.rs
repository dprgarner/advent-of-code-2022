#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Left,
    Right,
    Forward(usize),
}

impl Instruction {
    pub fn parse(input: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut idx = 0;
        while idx < input.len() {
            if &input[idx..idx + 1] == "L" || &input[idx..idx + 1] == "R" {
                instructions.push(match &input[idx..idx + 1] {
                    "L" => Instruction::Left,
                    "R" => Instruction::Right,
                    _ => panic!("Unexpected char"),
                });
                idx += 1;
            } else {
                let end = (&input[idx..])
                    .find(|c| c == 'L' || c == 'R')
                    .unwrap_or(input.len() - idx);
                instructions.push(Instruction::Forward(input[idx..idx + end].parse().unwrap()));
                idx += end;
            }
        }
        instructions
    }
}
