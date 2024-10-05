use std::str::FromStr;

use maschen::{self, ShuntingYard, TokenKind};

#[derive(Debug)]
struct StringToken<'a>(&'a str);

impl<'a> maschen::Token for StringToken<'a> {
    fn kind(&self) -> TokenKind {
        match self.0 {
            "*" | "/" => TokenKind::InfixOperator(0),
            "+" | "-" => TokenKind::InfixOperator(1),
            "(" => TokenKind::LeftParen,
            ")" => TokenKind::RightParen,
            "~" => TokenKind::UnaryOperator,
            "sin" | "cos" | "tan" | "log" => TokenKind::Function,
            _ => TokenKind::Value,
        }
    }
}
fn test_in_out(inp: &str, outp: &str) -> Result<(), maschen::Error> {
    let mut outstack = vec![];
    let mut opstack = vec![];
    let stream = inp.split(' ').map(StringToken);
    let mut yard = ShuntingYard::new(&mut outstack, &mut opstack);
    for v in stream {
        yard.process(v)?;
    }
    yard.finish()?;
    let mut yard_output = String::from_str(outstack.remove(0).0).unwrap();
    for tok in outstack.iter() {
        yard_output.push(' ');
        yard_output.push_str(tok.0);
    }
    assert_eq!(outp, yard_output.as_str());
    Ok(())
}
#[test]
fn simple_value() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3", "3")
}

#[test]
fn single_infix_op() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 + 3", "3 3 +")
}

#[test]
fn single_unary_op() -> anyhow::Result<(), maschen::Error> {
    test_in_out("~ 3", "3 ~")
}

#[test]
fn func() -> anyhow::Result<(), maschen::Error> {
    test_in_out("log ( 3 )", "3 log")
}

#[test]
fn precendence() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 + 4 * 5", "3 4 5 * +")
}

#[test]
fn left_assoc() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 + 4 - 5", "3 4 + 5 -")
}

#[test]
fn unary_and_infix() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 * ~ 4", "3 4 ~ *")
}

#[test]
fn unary_and_infix_and_func() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 * log ( ~ 4 + 5 )", "3 4 ~ 5 + log *")
}

#[test]
fn nested_braces() -> anyhow::Result<(), maschen::Error> {
    test_in_out("3 * ( 4 + ( 2 - 1 ) )", "3 4 2 1 - + *")
}
