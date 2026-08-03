use std::collections::{VecDeque, btree_map::Values};

#[derive(Clone, Copy, Debug)]
enum RpToken {
    Integer(i64),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HsToken {
    Integer(i64),
    Add,
    Sub,
    Mul,
    Div,
    OpenBracket,
    CloseBracket,
}

impl HsToken {
    fn priority(self: Self) -> usize {
        match self {
            Self::Integer(_) => 0,
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
            Self::OpenBracket | Self::CloseBracket => 3,
        }
    }

    fn to_rptoken(self: Self) -> Option<RpToken> {
        match self {
            Self::Add => Some(RpToken::Add),
            Self::Sub => Some(RpToken::Sub),
            Self::Mul => Some(RpToken::Mul),
            Self::Div => Some(RpToken::Div),
            Self::Integer(value) => Some(RpToken::Integer(value)),
            _ => None,
        }
    }
}

fn apply_binary<F>(stack: &mut Vec<i64>, op: F) -> Option<i64>
    where F: FnOnce(i64, i64) -> i64
{
    let rhs = stack.pop()?;
    let lhs = stack.pop()?;
    Some(op(lhs, rhs))
}

fn reverse_polish_calc(tokens: &[RpToken]) -> Option<i64> {
    let mut stack = Vec::new();
    for t in tokens.iter() {
        let val = match t {
            RpToken::Integer(val) => *val,
            RpToken::Add => apply_binary(&mut stack, |x, y| x + y)?,
            RpToken::Sub => apply_binary(&mut stack, |x, y| x - y)?,
            RpToken::Mul => apply_binary(&mut stack, |x, y| x * y)?,
            RpToken::Div => apply_binary(&mut stack, |x, y| x / y)?,
        };

        stack.push(val);
    };

    let res = stack.pop()?;
    if stack.is_empty() {
        Some(res)
    } else {
        None
    }
}

fn shunting_yard(tokens: &[HsToken]) -> Vec<RpToken> {
    use HsToken::*;

    let mut stack = Vec::new();
    let mut res = Vec::new();

    for t in tokens.iter().copied() {
        match t {
            Integer(val) => res.push(RpToken::Integer(val)),
            OpenBracket => stack.push(t),
            CloseBracket => {
                while let Some(o) = stack.last().copied() && o != OpenBracket {
                    let o = stack.pop().unwrap();
                    res.push(o.to_rptoken().unwrap());
                }
                let _ = stack.pop();
            },
            Add | Sub | Mul | Div => {
                while let Some(o) = stack.last().copied() 
                        && o.priority() >= t.priority()
                        && o != OpenBracket {
                    res.push(stack
                                .pop()
                                .unwrap()
                                .to_rptoken()
                                .unwrap()
                                );
                }
                stack.push(t);
            }
        }
    }

    // TODO: test that there is no parantheses left?
    for o in stack.iter().rev().copied().flat_map(|t| t.to_rptoken()) {
        res.push(o);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rp_testcases() -> Vec<(&'static str, i64)> {
        let res = vec![
            ("3 4 +", 7),
            ("10 3 -", 7),
            ("6 7 *", 42),
            ("20 5 /", 4),
            ("8 2 / 7 +", 11),
            ("12 3 / 5 +", 9),
            ("9 3 / 2 *", 6),
            ("15 5 / 8 +", 11),
            ("3 4 + 5 *", 35),
            ("3 4 5 + *", 27),
            ("7 2 3 * +", 13),
            ("5 1 2 + 4 * + 3 -", 14),
            ("1 2 + 3 * 4 5 + -", 0),
            ("8 2 / 3 4 * + 5 -", 11),
            ("14 6 + 2 /", 10),
            ("8 3 * 5 +", 29),
            ("7 2 5 + *", 49),
            ("12 3 / 2 7 + *", 36),
            ("6 2 / 3 4 * +", 15),
            ("10 3 2 + * 5 /", 10),
            ("1 2 + 3 + 4 + 5 +", 15),
            ("9 8 + 7 6 + * 5 -", 216),
            ("1 2 3 4 + * +", 15),
            ("16 4 / 5 2 + *", 28),
            ("18 6 / 9 3 / +", 6),
            ("20 5 / 4 2 / +", 6),
            ("24 6 / 8 2 / *", 16),
            ("5 5 + 3 * 6 -", 24),
            ("30 6 / 7 2 - *", 25),
            ("100 10 / 2 / 3 +", 8),
        ];
        res
    }

    fn parse_rp(testcase: &str) -> Vec<RpToken> {
        testcase
            .split_whitespace()
            .map(|s| {
                if s == "+" {
                    RpToken::Add
                } else if s == "-" {
                    RpToken::Sub
                } else if s == "*" {
                    RpToken::Mul
                } else if s == "/" {
                    RpToken::Div
                } else if let Ok(val) = s.parse::<i64>() {
                    RpToken::Integer(val)
                } else {
                    panic!("Invalid test string");
                }
            })
            .collect()
    }

    #[test]
    fn test_run_rpn() {
        for (s, r) in make_rp_testcases().into_iter() {
            let tks = parse_rp(s);
            assert_eq!(reverse_polish_calc(&tks), Some(r), "Failed for '{}', {}", s, r);
        }
    }
    
    fn make_hs_testcases() -> Vec<(&'static str, i64)> {
        let res = vec![
            ("1 + 2", 3),
            ("7 - 4", 3),
            ("3 * 5", 15),
            ("20 / 4", 5),
            ("1 + 2 * 3", 7),
            ("8 - 3 * 2", 2),
            ("4 * 3 + 2", 14),
            ("18 / 3 + 4", 10),
            ("12 - 8 / 2", 8),
            ("2 + 3 * 4 - 5", 9),
            ("20 / 5 * 3", 12),
            ("24 / 4 / 2", 3),
            ("10 - 3 - 2", 5),
            ("10 + 6 / 3 * 2", 14),
            ("5 * 4 - 18 / 3", 14),
            ("( 1 + 2 ) * 3", 9),
            ("8 / ( 2 + 2 )", 2),
            ("( 7 - 3 ) * ( 2 + 4 )", 24),
            ("20 / ( 3 + 2 )", 4),
            ("( 8 + 4 ) / 3", 4),
            ("2 * ( 3 + 4 ) - 5", 9),
            ("18 / ( 2 * 3 ) + 7", 10),
            ("( 10 - 4 ) / 2 + 8", 11),
            ("3 + ( 12 / 4 ) * 5", 18),
            ("( 2 + 3 ) * ( 8 - 4 )", 20),
            ("100 / 10 + 6 * 3", 28),
            ("50 - 4 * 8 + 2", 20),
            ("72 / 8 * 3 - 5", 22),
            ("9 * 9 - 80 / 10", 73),
            ("64 / 4 / 4 + 7", 11),
            ("( 15 + 5 ) / ( 6 - 2 )", 5),
            ("( 9 - 3 ) * ( 12 / 4 )", 18),
            ("48 / ( 2 + 4 ) * 3", 24),
            ("7 + 8 * ( 6 - 3 )", 31),
            ("( 20 - 8 ) / 3 + 9", 13),
            ("6 * ( 5 + 3 ) / 4", 12),
            ("( 14 + 10 ) / 6 * 5", 20),
            ("90 / ( 3 * 5 ) + 11", 17),
            ("4 * ( 18 / 6 + 2 )", 20),
            ("( 30 / 5 - 2 ) * 7", 28),
            ("( 1 - 5 - 6 )", -10),
            ("( 3 - 8 * 2 )", -13),
            ("( 4 - 10 ) / 2", -3),
            ("12 / ( 2 - 5 )", -4),
            ("( 3 - 7 ) * ( 2 + 1 )", -12),
            ("20 + 5 * 4 / 2 - 3", 27),
            ("( 16 / 4 + 2 ) * ( 9 - 6 )", 18),
            ("100 / ( 5 * ( 2 + 2 ) )", 5),
            ("( 7 + 5 ) * ( 9 - 3 ) / 4", 18),
            ("2 + 3 * ( 4 + 5 * ( 6 - 4 ) )", 44),
        ];
        res
    }

    fn parse_hs(testcase: &str) -> Vec<HsToken> {
        testcase
            .split_whitespace()
            .map(|s| {
                if s == "(" {
                    HsToken::OpenBracket
                } else if s == ")" {
                    HsToken::CloseBracket
                } else if s == "+" {
                    HsToken::Add
                } else if s == "-" {
                    HsToken::Sub
                } else if s == "*" {
                    HsToken::Mul
                } else if s == "/" {
                    HsToken::Div
                } else if let Ok(val) = s.parse::<i64>() {
                    HsToken::Integer(val)
                } else {
                    panic!("Invalid test string {:#?}", testcase);
                }
            })
            .collect()
    }

    #[test]
    fn test_run_hs() {
        for (s, r) in make_hs_testcases().into_iter() {
            let hst = parse_hs(s);
            let rpt = shunting_yard(&hst[..]);
            assert_eq!(reverse_polish_calc(&rpt), Some(r), 
                        "Failed for '{}', {}. Intermediate {:#?}.", s, r, &rpt);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
