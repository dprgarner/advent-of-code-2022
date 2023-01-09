use core::panic;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
    hash::Hash,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct MonkeyName(char, char, char, char);

impl From<&str> for MonkeyName {
    fn from(s: &str) -> Self {
        let (a, b, c, d) = s
            .chars()
            .collect_tuple()
            .expect("Monkey name should be exactly four characters");
        MonkeyName(a, b, c, d)
    }
}

impl Debug for MonkeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MonkeyName(\"{}{}{}{}\")",
            self.0, self.1, self.2, self.3
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Eq, Debug)]
struct Monkey {
    name: MonkeyName,
    instruction: Option<(Operation, MonkeyName, MonkeyName)>,
    call: RefCell<Option<i64>>,
}

impl Monkey {
    fn parse_operation(instruction: &str) -> Option<(Operation, MonkeyName, MonkeyName)> {
        if !instruction.contains(" ") {
            return None;
        }
        let (dep1, dep2) = instruction.split_once(" ").expect("Invalid input");
        let dep1: MonkeyName = dep1.into();
        let dep2: MonkeyName = dep2[2..].into();

        match &instruction.split_at(5).1[0..1] {
            "+" => Some((Operation::Add, dep1, dep2)),
            "-" => Some((Operation::Subtract, dep1, dep2)),
            "*" => Some((Operation::Multiply, dep1, dep2)),
            "/" => Some((Operation::Divide, dep1, dep2)),
            _ => panic!("Unrecognised instruction"),
        }
    }

    fn parse_call(instruction: &str) -> Option<i64> {
        if instruction.contains(" ") {
            return None;
        }
        return Some(instruction.parse().expect("Could not parse integer"));
    }

    fn is_resolved(&self) -> bool {
        *self.call.borrow() != None
    }

    fn value(&self) -> i64 {
        self.call.borrow().expect("No value")
    }

    fn resolve(&self, value: i64) {
        *self.call.borrow_mut() = Some(value);
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let (name, instruction) = s.split_once(": ").expect("Invalid input");
        let call = Self::parse_call(instruction);
        let instruction = Self::parse_operation(instruction);
        return Monkey {
            name: name.into(),
            instruction,
            call: RefCell::new(call),
        };
    }
}

/// The soln to part b only works on the assumption that the monkey dependencies form a tree.
fn check_is_tree(monkeys: &HashMap<MonkeyName, Monkey>) {
    let mut dep_use = HashSet::new();

    for monkey in monkeys.values() {
        if let Some((_, m1, m2)) = &monkey.instruction {
            if dep_use.contains(m1) {
                panic!("Monkey used more than once: {:?}", m1);
            }
            dep_use.insert(m1);
            if dep_use.contains(m2) {
                panic!("Monkey used more than once: {:?}", m2);
            }
            dep_use.insert(m2);
        }
    }
}

fn parse_monkeys(input: impl Iterator<Item = String>) -> HashMap<MonkeyName, Monkey> {
    let mut monkeys = HashMap::new();
    for line in input {
        let monkey = Monkey::from(line.as_str());
        monkeys.insert(monkey.name, monkey);
    }
    monkeys
}

fn solve_next_monkey(monkeys: &HashMap<MonkeyName, Monkey>) -> bool {
    for monkey in monkeys.values().filter(|m| !m.is_resolved()) {
        if let Some((op, monkey_name_1, monkey_name_2)) = &monkey.instruction {
            let monkey_1 = &monkeys[monkey_name_1];
            let monkey_2 = &monkeys[monkey_name_2];
            if monkey_1.is_resolved() && monkey_2.is_resolved() {
                monkey.resolve(match op {
                    &Operation::Add => monkey_1.value() + monkey_2.value(),
                    &Operation::Subtract => monkey_1.value() - monkey_2.value(),
                    &Operation::Multiply => monkey_1.value() * monkey_2.value(),
                    &Operation::Divide => monkey_1.value() / monkey_2.value(),
                });
                return true;
            }
        }
    }
    false
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let monkeys = parse_monkeys(input);
    while !monkeys[&"root".into()].is_resolved() {
        if !solve_next_monkey(&monkeys) {
            panic!("No more monkeys to solve");
        };
    }
    Ok(monkeys[&"root".into()].value())
}

fn invert_monkeys(
    monkeys: &HashMap<MonkeyName, Monkey>,
    start: MonkeyName,
    end: MonkeyName,
) -> i64 {
    let mut monkey_name = start;
    // We represent the start equality as a difference equal to zero, i.e. monkey1 - monkey2 = 0.
    let mut required_value = 0;

    while monkey_name != end {
        let (op, m1_name, m2_name) = monkeys[&monkey_name]
            .instruction
            .as_ref()
            .expect("Monkey must have dependencies");

        let m1 = &monkeys[m1_name];
        let m2 = &monkeys[m2_name];

        let (next_monkey_name, other_monkey_value, unknown_first) = if m1.call.borrow().is_none() {
            (
                m1.name,
                m2.call
                    .borrow()
                    .expect("One of the two monkeys must have a value"),
                true,
            )
        } else {
            (
                m2.name,
                m1.call
                    .borrow()
                    .expect("One of the two monkeys must have a value"),
                false,
            )
        };

        monkey_name = next_monkey_name;

        match (op, unknown_first) {
            (&Operation::Add, _) => {
                required_value -= other_monkey_value;
            }
            (&Operation::Subtract, true) => {
                required_value += other_monkey_value;
            }
            (&Operation::Subtract, false) => {
                required_value = other_monkey_value - required_value;
            }
            (&Operation::Multiply, _) => {
                required_value /= other_monkey_value;
            }
            (&Operation::Divide, true) => {
                required_value = required_value * other_monkey_value;
            }
            (&Operation::Divide, false) => {
                required_value = other_monkey_value / required_value;
            }
        }
    }

    required_value
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let mut monkeys = parse_monkeys(input);
    check_is_tree(&monkeys);

    let mut root_monkey = monkeys
        .remove(&"root".into())
        .expect("should be a root monkey");
    if let Some((_, m1, m2)) = root_monkey.instruction {
        root_monkey.instruction = Some((Operation::Subtract, m1, m2));
    }
    monkeys.insert("root".into(), root_monkey);

    let mut human = monkeys.remove(&"humn".into()).expect("should be a human");
    human.instruction = None;
    *human.call.borrow_mut() = None;
    monkeys.insert("humn".into(), human);

    // Solves all monkeys which don't have a dependency on `humn`
    while solve_next_monkey(&monkeys) {}

    let result = invert_monkeys(&monkeys, "root".into(), "humn".into());

    // Sanity check: solve it again, and make sure that the root value evaluates to zero.
    let human = monkeys.remove(&"humn".into()).expect("should be a human");
    *human.call.borrow_mut() = Some(result);
    monkeys.insert("humn".into(), human);

    while !monkeys[&"root".into()].is_resolved() {
        if !solve_next_monkey(&monkeys) {
            panic!("No more monkeys to solve");
        };
    }
    assert_eq!(0, monkeys[&"root".into()].value());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_monkey_name() {
        let result = MonkeyName::from("root");
        assert_eq!(result, MonkeyName('r', 'o', 'o', 't'));
    }

    #[test]
    fn it_parses_monkeys() {
        assert_eq!(
            Monkey::from("root: pppw + sjmn"),
            Monkey {
                name: "root".into(),
                instruction: Some((Operation::Add, "pppw".into(), "sjmn".into())),
                call: RefCell::new(None),
            }
        );
        assert_eq!(
            Monkey::from("ptdq: humn - dvpt"),
            Monkey {
                name: "ptdq".into(),
                instruction: Some((Operation::Subtract, "humn".into(), "dvpt".into())),
                call: RefCell::new(None),
            }
        );
        assert_eq!(
            Monkey::from("pppw: cczh / lfqf"),
            Monkey {
                name: "pppw".into(),
                instruction: Some((Operation::Divide, "cczh".into(), "lfqf".into())),
                call: RefCell::new(None),
            }
        );
        assert_eq!(
            Monkey::from("lgvd: ljgn * ptdq"),
            Monkey {
                name: "lgvd".into(),
                instruction: Some((Operation::Multiply, "ljgn".into(), "ptdq".into())),
                call: RefCell::new(None),
            }
        );
        assert_eq!(
            Monkey::from("dbpl: 5"),
            Monkey {
                name: "dbpl".into(),
                instruction: None,
                call: RefCell::new(Some(5)),
            }
        );
    }
}
