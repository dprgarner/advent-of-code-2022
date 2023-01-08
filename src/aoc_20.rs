use std::error::Error;

use itertools::Itertools;

fn shift(numbers: &mut Vec<(usize, i64)>, og_idx: usize) {
    let l = numbers.len() - 1;
    // println!("{:?}", &numbers.iter().map(|(_, x)| x).join(", "));
    let (idx, _) = numbers.iter().find_position(|x| x.0 == og_idx).unwrap();
    let number = numbers[idx].1;
    let positive = number > 0;

    let mut idx = idx;
    for _ in 0..(number.abs() % i64::try_from(l).unwrap()) {
        if positive {
            if idx < l {
                numbers.swap(idx, idx + 1);
                idx += 1;
            } else {
                numbers.swap(0, l);
                idx = 0;
            }
        } else {
            if idx > 0 {
                numbers.swap(idx - 1, idx);
                idx -= 1;
            } else {
                numbers.swap(0, l);
                idx = l;
            }
        }
    }
}

pub fn parse(input: impl Iterator<Item = String>) -> Vec<(usize, i64)> {
    input
        .enumerate()
        .map(|(idx, x)| (idx, x.parse::<i64>().unwrap()))
        .collect_vec()
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let mut numbers = parse(input);
    let l = numbers.len();

    for og_idx in 0..l {
        shift(&mut numbers, og_idx);
    }

    let (start_idx, _) = numbers.iter().find_position(|x| x.1 == 0).unwrap();
    let n1 = numbers[(start_idx + 1000) % l].1;
    let n2 = numbers[(start_idx + 2000) % l].1;
    let n3 = numbers[(start_idx + 3000) % l].1;

    println!("{} {} {}", n1, n2, n3);
    Ok(n1 + n2 + n3)
}

const DECRYPTION_KEY: i64 = 811589153;

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i64, Box<dyn Error>> {
    let mut numbers = parse(input)
        .into_iter()
        .map(|(idx, value)| (idx, value * DECRYPTION_KEY))
        .collect_vec();
    let l = numbers.len();

    for _ in 0..10 {
        for og_idx in 0..l {
            shift(&mut numbers, og_idx);
        }
    }

    let (start_idx, _) = numbers.iter().find_position(|x| x.1 == 0).unwrap();
    let n1 = numbers[(start_idx + 1000) % l].1;
    let n2 = numbers[(start_idx + 2000) % l].1;
    let n3 = numbers[(start_idx + 3000) % l].1;

    println!("{} {} {}", n1, n2, n3);
    Ok(n1 + n2 + n3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shifts_forward() {
        let mut numbers = parse(
            ["1", "2", "-3", "3", "-2", "0", "4"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 3);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![1, 2, -3, -2, 0, 4, 3]
        );
    }

    #[test]
    fn it_shifts_backward() {
        let mut numbers = parse(
            ["1", "2", "-3", "3", "-2", "0", "4"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 4);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![1, 2, -2, -3, 3, 0, 4]
        );
    }

    #[test]
    fn it_shifts_to_back_to_start() {
        let mut numbers = parse(
            ["1", "2", "-2", "3", "-7", "0", "4"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 2);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![-2, 1, 2, 3, -7, 0, 4]
        );
    }

    #[test]
    fn it_shifts_to_end() {
        let mut numbers = parse(
            ["1", "2", "-4", "3", "-2", "0", "4"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 3);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![1, 2, -4, -2, 0, 4, 3,]
        );
    }

    #[test]
    fn it_shifts_around_end() {
        let mut numbers = parse(
            ["1", "2", "-4", "4", "-2", "0", "6"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 3);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![4, 2, -4, -2, 0, 6, 1]
        );
    }

    #[test]
    fn it_shifts_around_end_more() {
        let mut numbers = parse(
            ["1", "2", "-4", "5", "-2", "0", "6"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 3);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![2, 5, -4, -2, 0, 6, 1]
        );
    }

    #[test]
    fn it_shifts_forward_back_to_start() {
        let mut numbers = parse(
            ["1", "2", "-4", "5", "-2", "0", "6"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 6);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![1, 2, -4, 5, -2, 0, 6]
        );
    }

    #[test]
    fn it_shifts_backwards_back_to_start() {
        let mut numbers = parse(
            ["1", "2", "-4", "-6", "-2", "0", "6"]
                .map(String::from)
                .into_iter(),
        );
        shift(&mut numbers, 6);
        assert_eq!(
            numbers.into_iter().map(|(_, x)| x).collect_vec(),
            vec![1, 2, -4, -6, -2, 0, 6]
        );
    }

    #[test]
    fn it_runs_a() {
        let input = ["1", "2", "-3", "3", "-2", "0", "4"]
            .map(String::from)
            .into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn it_runs_b() {
        let input = ["1", "2", "-3", "3", "-2", "0", "4"]
            .map(String::from)
            .into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 1623178306);
    }
}
