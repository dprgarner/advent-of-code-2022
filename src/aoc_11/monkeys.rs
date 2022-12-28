use std::collections::VecDeque;
use std::{cell::RefCell, rc::Weak};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub enum Operation {
    Square,
    Add(u64),
    Multiply(u64),
}

#[derive(Debug)]
pub struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    pub test_divisor: u64,
    true_monkey: RefCell<Weak<Monkey>>,
    false_monkey: RefCell<Weak<Monkey>>,
    pub inspection_count: RefCell<u64>,
}

mod builder {
    use std::collections::VecDeque;

    use super::Operation;

    pub fn parse_items(line: &str) -> Option<VecDeque<u64>> {
        let mut items: VecDeque<u64> = VecDeque::new();
        for x in line.strip_prefix("  Starting items: ")?.split(", ") {
            items.push_back(x.parse().ok()?)
        }
        Some(items)
    }

    pub fn parse_operation(line: &str) -> Option<Operation> {
        let stripped_line = line.strip_prefix("  Operation: new = old ")?;
        if stripped_line.starts_with("* old") {
            return Some(Operation::Square);
        }
        if stripped_line.starts_with("*") {
            let factor = stripped_line.split_once("* ")?.1.parse().ok()?;
            return Some(Operation::Multiply(factor));
        }
        if stripped_line.starts_with("+") {
            let sum = stripped_line.split_once("+ ")?.1.parse().ok()?;
            return Some(Operation::Add(sum));
        }
        None
    }

    pub fn parse_test(line: &str) -> Option<u64> {
        let stripped_line = line.strip_prefix("  Test: divisible by ")?;
        let divisor = stripped_line.parse().ok()?;
        Some(divisor)
    }

    pub fn parse_throw(line: &str) -> Option<usize> {
        let monkey_idx = line.split_once(" throw to monkey ")?.1.parse().ok()?;
        Some(monkey_idx)
    }
}

pub fn parse(mut input: impl Iterator<Item = String>) -> Option<Vec<Rc<Monkey>>> {
    let mut monkeys: Vec<Rc<Monkey>> = Vec::new();
    let mut monkey_true_destinations: HashMap<usize, usize> = HashMap::new();
    let mut monkey_false_destinations: HashMap<usize, usize> = HashMap::new();
    let mut monkey_idx = 0;
    loop {
        if input.next().is_none() {
            break;
        }
        let items = RefCell::new(builder::parse_items(&input.next()?)?);
        let operation = builder::parse_operation(&input.next()?)?;
        let test_divisor = builder::parse_test(&input.next()?)?;
        monkey_true_destinations.insert(monkey_idx, builder::parse_throw(&input.next()?)?);
        monkey_false_destinations.insert(monkey_idx, builder::parse_throw(&input.next()?)?);
        monkeys.push(Rc::new(Monkey {
            items,
            operation,
            test_divisor,
            true_monkey: RefCell::new(Weak::new()),
            false_monkey: RefCell::new(Weak::new()),
            inspection_count: RefCell::new(0),
        }));
        monkey_idx += 1;
        input.next();
    }
    for (i, monkey) in monkeys.iter().enumerate() {
        *monkey.true_monkey.borrow_mut() = Rc::downgrade(&monkeys[monkey_true_destinations[&i]]);
        *monkey.false_monkey.borrow_mut() = Rc::downgrade(&monkeys[monkey_false_destinations[&i]]);
    }
    Some(monkeys)
}

impl Monkey {
    pub fn take_turn(&self, divide_by_three: bool, mod_by_large_number: &Option<u64>) {
        while !self.items.borrow().is_empty() {
            let mut item = self.items.borrow_mut().pop_front().unwrap();
            item = match self.operation {
                Operation::Square => item * item,
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
            };
            if divide_by_three {
                item = item / 3;
            }
            // A number `x` passes the test if and only if `x + k * test_divisor` also passes the test, for any `k`.
            // This means we can mod out by the product of the prime factors.
            if let Some(modulo) = mod_by_large_number {
                item = item % modulo;
            }
            let test_result = item % self.test_divisor == 0;
            let monkey = if test_result {
                &self.true_monkey
            } else {
                &self.false_monkey
            };
            monkey
                .borrow()
                .upgrade()
                .unwrap()
                .items
                .borrow_mut()
                .push_back(item);
            *self.inspection_count.borrow_mut() += 1;
        }
    }
}
