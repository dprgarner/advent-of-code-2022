use itertools::Itertools;
use std::cmp::Ordering;

mod signal;
use signal::Signal;

fn parse_signal_pairs(input: impl Iterator<Item = String>) -> Option<Vec<(Signal, Signal)>> {
    let mut signal_pairs = Vec::new();
    let mut idx = 0;
    let input: Vec<String> = input.collect_vec();
    while idx < input.len() {
        let left = Signal::parse(&input[idx])?;
        let right = Signal::parse(&input[idx + 1])?;
        signal_pairs.push((left, right));
        idx += 3;
    }
    Some(signal_pairs)
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Int(x), Self::Int(y)) = (self, other) {
            if x < y {
                return Ordering::Less;
            };
            if x > y {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        }
        if let (Self::Int(int), Self::List(_)) = (self, other) {
            let vec = vec![Self::Int(*int)];
            return Self::List(vec).cmp(other);
        }
        if let (Self::List(_), Self::Int(int)) = (self, other) {
            let vec = vec![Self::Int(*int)];
            return self.cmp(&Self::List(vec));
        }
        if let (Self::List(x), Self::List(y)) = (self, other) {
            for i in 0..x.len().min(y.len()) {
                let is_ordered_at_idx = x[i].cmp(&y[i]);
                if is_ordered_at_idx != Ordering::Equal {
                    return is_ordered_at_idx;
                }
            }
            if x.len() < y.len() {
                return Ordering::Less;
            }
            if x.len() > y.len() {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let signal_pairs = parse_signal_pairs(input).expect("Input should be parseable");
    let mut count = 0;
    for (idx, (left, right)) in signal_pairs.iter().enumerate() {
        if left < right {
            count += idx + 1;
        }
    }
    Ok(count)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<usize, &'static str> {
    let mut signals = input
        .filter(|x| x != "")
        .map(|x| Signal::parse(&x))
        .collect::<Option<Vec<Signal>>>()
        .expect("Input should be parseable");

    signals.push(Signal::parse("[[2]]").unwrap());
    signals.push(Signal::parse("[[6]]").unwrap());
    signals.sort();
    let marker1 = Signal::parse("[[2]]").unwrap();
    let marker2 = Signal::parse("[[6]]").unwrap();
    let idx1 = signals.iter().position(|s| s == &marker1).unwrap();
    let idx2 = signals.iter().position(|s| s == &marker2).unwrap();

    Ok((idx1 + 1) * (idx2 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_signal_pairs() {
        let input = ["[1]", "[[]]", "", "[1,2]", "[1,[2,3]]"]
            .map(String::from)
            .into_iter();

        let result = parse_signal_pairs(input).unwrap();
        assert_eq!(
            result,
            vec![
                (
                    Signal::List(vec![Signal::Int(1)]),
                    Signal::List(vec![Signal::List(vec![])]),
                ),
                (
                    Signal::List(vec![Signal::Int(1), Signal::Int(2)]),
                    Signal::List(vec![
                        Signal::Int(1),
                        Signal::List(vec![Signal::Int(2), Signal::Int(3)])
                    ]),
                ),
            ]
        );
    }

    #[test]
    fn it_orders_same_list_size() {
        let x = Signal::parse("[1,1]").unwrap();
        let y = Signal::parse("[1,2]").unwrap();
        assert_eq!(x < y, true);
        assert_eq!(y < x, false);
    }

    #[test]
    fn it_orders_different_list_sizes() {
        let x = Signal::parse("[1,2]").unwrap();
        let y = Signal::parse("[1,2,3]").unwrap();
        assert_eq!(x < y, true);
        assert_eq!(y < x, false);
    }

    #[test]
    fn it_orders_lists_against_lists() {
        let x = Signal::parse("[[1],[2]]").unwrap();
        let y = Signal::parse("[[1],[3]]").unwrap();
        assert_eq!(x < y, true);
        assert_eq!(y < x, false);
    }

    #[test]
    fn it_orders_lists_against_ints() {
        let x = Signal::parse("[[1],[2]]").unwrap();
        let y = Signal::parse("[1,[3]]").unwrap();
        assert_eq!(x < y, true);
        assert_eq!(y < x, false);
    }

    #[test]
    fn it_runs_a() {
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ]
        .map(String::from)
        .into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn it_runs_b() {
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ]
        .map(String::from)
        .into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 140);
    }
}
