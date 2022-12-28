use std::str::FromStr;

enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line == "noop" {
            return Ok(Self::Noop);
        }
        if line.starts_with("addx ") {
            let count_str = line.split(" ").nth(1).ok_or(ParseInstructionError)?;
            let count = count_str.parse().or(Err(ParseInstructionError))?;
            return Ok(Self::Add(count));
        }

        Err(ParseInstructionError)
    }
}

struct Oscillator {
    history: Vec<i32>,
}

impl Oscillator {
    fn new() -> Oscillator {
        let mut history = Vec::new();
        history.push(1);
        Oscillator { history }
    }

    fn act(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                let value = self.history.last().unwrap();
                self.history.push(*value);
            }
            Instruction::Add(x) => {
                let mut value = *self.history.last().unwrap();
                self.history.push(value);
                value += *x;
                self.history.push(value);
            }
        }
    }

    fn plot(&self) -> String {
        let width = 40;
        let height = 6;
        assert_eq!(self.history.len(), width * height + 1);

        let mut result = String::new();
        for i in 0..height {
            for j in 0..width {
                let idx = i * width + j;
                let register = &self.history[idx];
                let delta = (j as i32) - register;
                if delta.abs() <= 1 {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push_str("\n");
        }
        result
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, &'static str> {
    let instructions = input.map(|line| line.parse().expect("Could not parse line"));
    let mut oscillator = Oscillator::new();
    for instruction in instructions {
        oscillator.act(&instruction);
    }

    let mut signal_strength = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let signal_boost = (cycle as i32) * *oscillator.history.get(cycle - 1).unwrap();
        signal_strength += signal_boost;
    }

    Ok(signal_strength)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<&'static str, &'static str> {
    let instructions = input.map(|line| line.parse().expect("Could not parse line"));
    let mut oscillator = Oscillator::new();
    for instruction in instructions {
        oscillator.act(&instruction);
    }

    println!("{}", &oscillator.plot());

    Ok("👍")
}
