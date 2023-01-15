use itertools::Itertools;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    MinusOne,
    MinusTwo,
}

use SnafuDigit::*;

impl From<&char> for SnafuDigit {
    fn from(s: &char) -> Self {
        match s {
            '2' => Two,
            '1' => One,
            '0' => Zero,
            '-' => MinusOne,
            '=' => MinusTwo,
            _ => panic!("Unrecognised character"),
        }
    }
}

impl From<&SnafuDigit> for char {
    fn from(s: &SnafuDigit) -> Self {
        match s {
            Two => '2',
            One => '1',
            Zero => '0',
            MinusOne => '-',
            MinusTwo => '=',
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SnafuNumber(Vec<SnafuDigit>);

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_iter(self.0.iter().map(|s| char::from(s)));
        write!(f, "{}", str)
    }
}

impl From<&str> for SnafuNumber {
    fn from(s: &str) -> Self {
        Self(s.chars().map(|s| (&s).into()).collect_vec())
    }
}

impl From<&SnafuNumber> for String {
    fn from(s: &SnafuNumber) -> Self {
        String::from_iter(s.0.iter().map(|d| char::from(d)))
    }
}

impl From<&SnafuNumber> for i64 {
    fn from(s: &SnafuNumber) -> Self {
        let mut as_decimal = 0;
        let mut place_value = 1;
        let mut idx = s.0.len();

        while idx > 0 {
            idx -= 1;
            let digit = &s.0[idx];
            as_decimal += (match digit {
                Two => 2,
                One => 1,
                Zero => 0,
                MinusOne => -1,
                MinusTwo => -2,
            }) * place_value;
            place_value *= 5;
        }

        as_decimal
    }
}

impl From<&i64> for SnafuNumber {
    fn from(decimal: &i64) -> Self {
        let mut snafu_digits = Vec::new();
        let mut decimal = *decimal;

        while decimal > 0 {
            let (carry_over, next_digit) = match decimal % 5 {
                0 => (0, Zero),
                1 => (0, One),
                2 => (0, Two),
                3 => (1, MinusTwo),
                4 => (1, MinusOne),
                _ => panic!("Unexpected input"),
            };
            snafu_digits.push(next_digit);
            decimal = carry_over + decimal / 5;
        }

        snafu_digits.reverse();
        Self(snafu_digits)
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let snafus: Vec<SnafuNumber> = input.map(|l| l.as_str().into()).collect_vec();
    let total = snafus.iter().fold(0, |acc, x| acc + i64::from(x));
    let snafu_total = SnafuNumber::from(&total);

    Ok(snafu_total.to_string())
}

pub fn solve_b(_input: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    panic!("Merry Christmas!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_coverts_snafu_numbers_to_decimal() {
        let candidates: Vec<(&str, i64)> = vec![
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];
        for (snafu_str, expected_value) in candidates {
            let snafu: SnafuNumber = snafu_str.into();
            let snafu_value: i64 = (&snafu).into();
            assert_eq!(snafu_value, expected_value);
        }
    }

    #[test]
    fn it_coverts_decimal_numbers_to_snafu() {
        let candidates: Vec<(&str, i64)> = vec![
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];
        for (snafu_str, actual_value) in candidates {
            let snafu: SnafuNumber = (&actual_value).into();
            assert_eq!(snafu_str, format!("{}", snafu));
        }
    }

    #[test]
    fn it_runs_a() {
        #[rustfmt::skip]
        let input = [
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        ]
        .map(String::from)
        .into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, "2=-1=0");
    }
}
