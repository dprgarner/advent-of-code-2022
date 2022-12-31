#[derive(PartialEq, Debug)]
enum SignalToken {
    OpenBracket,
    CloseBracket,
    Int(u32),
}

impl SignalToken {
    fn parse_next<'a>(s: &'a str) -> Option<(SignalToken, &'a str)> {
        if s.len() == 0 {
            return None;
        }
        Some(match &s[0..1] {
            "[" => (SignalToken::OpenBracket, &s[1..]),
            "]" => (SignalToken::CloseBracket, &s[1..]),
            "," => SignalToken::parse_next(&s[1..])?,
            _ => {
                let idx = s
                    .find(|x| x == '[' || x == ']' || x == ',')
                    .unwrap_or(s.len());
                let int: u32 = (&s[0..idx]).parse().ok()?;
                (SignalToken::Int(int), &s[idx..])
            }
        })
    }

    fn parse(s: &str) -> Option<Vec<SignalToken>> {
        if !s.is_ascii() {
            return None;
        }
        let mut tokens = Vec::new();
        let mut s = s;
        while s.len() > 0 {
            let (token, next_s) = SignalToken::parse_next(s)?;
            tokens.push(token);
            s = next_s;
        }
        Some(tokens)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Signal {
    Int(u32),
    List(Vec<Signal>),
}

impl Signal {
    fn find_closing_bracket(tokens: &[SignalToken]) -> Option<usize> {
        let mut open_minus_close_brackets = 0;
        for (idx, token) in tokens.iter().enumerate() {
            open_minus_close_brackets += match token {
                SignalToken::OpenBracket => 1,
                SignalToken::CloseBracket => -1,
                _ => 0,
            };
            if open_minus_close_brackets == 0 {
                return Some(idx);
            }
        }
        None
    }

    fn parse_list(tokens: &[SignalToken]) -> Option<Signal> {
        if tokens.first()? != &SignalToken::OpenBracket
            || tokens.last()? != &SignalToken::CloseBracket
        {
            return None;
        }

        let mut signals = Vec::new();
        let mut idx = 1;
        while idx < tokens.len() - 1 {
            if let SignalToken::Int(int) = tokens[idx] {
                signals.push(Signal::Int(int));
                idx += 1
            } else if SignalToken::OpenBracket == tokens[idx] {
                let end_idx = idx + Signal::find_closing_bracket(&tokens[idx..])?;
                let sub_signals = Signal::parse_list(&tokens[idx..end_idx + 1])?;
                signals.push(sub_signals);
                idx = end_idx + 1;
            } else {
                return None;
            }
        }

        Some(Signal::List(signals))
    }

    pub fn parse(s: &str) -> Option<Signal> {
        let tokens = SignalToken::parse(s)?;
        Signal::parse_list(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_next_token() {
        assert_eq!(
            SignalToken::parse_next("1,2,3").unwrap(),
            (SignalToken::Int(1), ",2,3")
        );
        assert_eq!(
            SignalToken::parse_next("101,2,3").unwrap(),
            (SignalToken::Int(101), ",2,3")
        );
        assert_eq!(
            SignalToken::parse_next("[1,2,3]").unwrap(),
            (SignalToken::OpenBracket, "1,2,3]")
        );
        assert_eq!(
            SignalToken::parse_next("]").unwrap(),
            (SignalToken::CloseBracket, "")
        );
        assert_eq!(
            SignalToken::parse_next(",1").unwrap(),
            (SignalToken::Int(1), "")
        );
    }

    #[test]
    fn it_parses_a_string_to_tokens() {
        assert_eq!(SignalToken::parse("").unwrap(), Vec::new());
        assert_eq!(
            SignalToken::parse("[1,[2,3]]").unwrap(),
            vec![
                SignalToken::OpenBracket,
                SignalToken::Int(1),
                SignalToken::OpenBracket,
                SignalToken::Int(2),
                SignalToken::Int(3),
                SignalToken::CloseBracket,
                SignalToken::CloseBracket,
            ]
        );
    }

    #[test]
    fn it_parses_a_flat_list() {
        let input = "[1,1,3,1,1]";
        assert_eq!(
            Signal::parse(input).unwrap(),
            Signal::List(vec![
                Signal::Int(1),
                Signal::Int(1),
                Signal::Int(3),
                Signal::Int(1),
                Signal::Int(1),
            ])
        );
    }

    #[test]
    fn it_rejects_malformed_strings() {
        assert_eq!(Signal::parse("[y̆]"), None);
        assert_eq!(Signal::parse("[1"), None);
        assert_eq!(Signal::parse(""), None);
        assert_eq!(Signal::parse("1]"), None);
    }

    #[test]
    fn it_finds_closing_brackets() {
        assert_eq!(
            Signal::find_closing_bracket(&vec![
                SignalToken::OpenBracket,
                SignalToken::Int(1),
                SignalToken::CloseBracket,
                SignalToken::Int(2),
                SignalToken::Int(3),
                SignalToken::CloseBracket,
            ])
            .unwrap(),
            2
        );
    }

    #[test]
    fn it_rejects_unmatched_closing_brackets() {
        assert_eq!(
            Signal::find_closing_bracket(&vec![
                SignalToken::OpenBracket,
                SignalToken::Int(1),
                SignalToken::Int(2),
                SignalToken::Int(3),
            ]),
            None
        );
    }

    #[test]
    fn it_parses_an_empty_list() {
        assert_eq!(Signal::parse("[]").unwrap(), Signal::List(vec![]));
        assert_eq!(
            Signal::parse("[[]]").unwrap(),
            Signal::List(vec![Signal::List(vec![])])
        );
    }

    #[test]
    fn it_parses_a_nested_list() {
        let input = "[1,[2,[3,4]],5,6]";
        assert_eq!(
            Signal::parse(input).unwrap(),
            Signal::List(vec![
                Signal::Int(1),
                Signal::List(vec![
                    Signal::Int(2),
                    Signal::List(vec![Signal::Int(3), Signal::Int(4),]),
                ]),
                Signal::Int(5),
                Signal::Int(6)
            ])
        );
    }

    #[test]
    fn it_parses_multiple_nested_lists() {
        let input = "[[1,2],3,4,[5,6]]";
        assert_eq!(
            Signal::parse(input).unwrap(),
            Signal::List(vec![
                Signal::List(vec![Signal::Int(1), Signal::Int(2),]),
                Signal::Int(3),
                Signal::Int(4),
                Signal::List(vec![Signal::Int(5), Signal::Int(6),]),
            ])
        );
    }
}
