use itertools::Itertools;

mod monkeys;

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<u64, &'static str> {
    let monkeys = monkeys::parse(input).ok_or("Could not parse monkeys")?;

    for _ in 0..20 {
        for monkey in &monkeys {
            monkey.take_turn(true, &None);
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| *m.inspection_count.borrow())
        .collect_vec();
    inspection_counts.sort_unstable();

    let mut monkey_business = inspection_counts.pop().unwrap();
    monkey_business = monkey_business * inspection_counts.pop().unwrap();
    Ok(monkey_business)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<u64, &'static str> {
    let monkeys = monkeys::parse(input).ok_or("Could not parse monkeys")?;

    let mut large_modulo = 1;
    for monkey in &monkeys {
        large_modulo *= monkey.test_divisor;
    }
    let large_modulo = Some(large_modulo);
    for _ in 0..10000 {
        for monkey in &monkeys {
            monkey.take_turn(false, &large_modulo);
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| *m.inspection_count.borrow())
        .collect_vec();
    inspection_counts.sort_unstable();

    let mut monkey_business = inspection_counts.pop().unwrap();
    monkey_business = monkey_business * inspection_counts.pop().unwrap();
    Ok(monkey_business)
}
