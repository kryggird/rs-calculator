enum Token {
    Integer(i64),
    Add,
    Sub,
    Mul,
    Div,
}

fn apply_binary<F>(stack: &mut Vec<i64>, op: F) -> Option<i64>
    where F: FnOnce(i64, i64) -> i64
{
    let rhs = stack.pop()?;
    let lhs = stack.pop()?;
    Some(op(lhs, rhs))
}

fn reverse_polish_calc(tokens: &[Token]) -> Option<i64> {
    let mut stack = Vec::new();
    for t in tokens.iter() {
        let val = match t {
            Token::Integer(val) => *val,
            Token::Add => apply_binary(&mut stack, |x, y| x + y)?,
            Token::Sub => apply_binary(&mut stack, |x, y| x - y)?,
            Token::Mul => apply_binary(&mut stack, |x, y| x * y)?,
            Token::Div => apply_binary(&mut stack, |x, y| x / y)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_testcases() -> Vec<(&'static str, i64)> {
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

    fn parse(testcase: &str) -> Vec<Token> {
        testcase
            .split_whitespace()
            .map(|s| {
                if s == "+" {
                    Token::Add
                } else if s == "-" {
                    Token::Sub
                } else if s == "*" {
                    Token::Mul
                } else if s == "/" {
                    Token::Div
                } else if let Ok(val) = s.parse::<i64>() {
                    Token::Integer(val)
                } else {
                    panic!("Invalid test string");
                }
            })
            .collect()
    }

    #[test]
    fn test_run_rpn() {
        for (s, r) in make_testcases().into_iter() {
            let tks = parse(s);
            assert_eq!(reverse_polish_calc(&tks), Some(r), "Failed for '{}', {}", s, r);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
