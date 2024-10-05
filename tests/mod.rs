use maschen::{self, ShuntingYard, TokenKind};
use std::str::FromStr;

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
            "," => TokenKind::FnSeparator,
            "sin" | "cos" | "tan" | "log" => TokenKind::Function(1),
            "max" | "min" => TokenKind::Function(2),
            x if isize::from_str(x).is_ok() => TokenKind::Value,
            x => panic!("{} is not an int", x),
        }
    }
}
fn test_run(inp: &str) -> Result<String, maschen::Error> {
    let stream = inp.split_whitespace().map(StringToken);
    let mut yard = ShuntingYard::default();
    for v in stream {
        yard.process(v)?;
    }
    let mut outcome = yard.finish()?;
    let mut yard_output = String::from_str(outcome.remove(0).0).unwrap();
    for tok in outcome.iter() {
        yard_output.push(' ');
        yard_output.push_str(tok.0);
    }
    Ok(yard_output)
}

macro_rules! test_works {
    ($name: ident, $in: expr, $out: expr) => {
        #[test]
        fn $name() -> Result<(), maschen::Error> {
            assert_eq!(test_run($in)?.as_str(), $out);
            Ok(())
        }
    };
}

macro_rules! test_fails {
    ($name: ident, $in: expr, $patt: pat) => {
        #[test]
        fn $name() {
            assert!(matches!(test_run($in), Err($patt)))
        }
    };
}

test_works!(simple_value, "3", "3");
test_works!(single_infix_op, "3 + 3", "3 3 +");
test_works!(single_unary_op, "~ 3", "3 ~");
test_works!(func, "log ( 3 )", "3 log");
test_works!(precendence, "3 + 4 * 5", "3 4 5 * +");
test_works!(left_assoc, "3 + 4 - 5", "3 4 + 5 -");
test_works!(unary_and_infix, "3 * ~ 4", "3 4 ~ *");
test_works!(
    unary_and_infix_and_func,
    "3 * log ( ~ 4 + 5 )",
    "3 4 ~ 5 + log *"
);
test_works!(nested_braces, "3 * ( 4 + ( 2 - 1 ) )", "3 4 2 1 - + *");
test_works!(fn2, "max ( 2 , 3 )", "2 3 max");
test_works!(fn_with_nested, "max ( ( ( 2 , 3 ) ) )", "2 3 max");
test_works!(
    nested_fns,
    "max ( 3 , max ( log ( 4 ) , 2 ) )",
    "3 4 log 2 max max"
);

test_fails!(empty, "", maschen::Error::Malformed);
test_fails!(bad_infix1, "3 3 +", maschen::Error::Malformed);
test_fails!(bad_infix2, "+ 3 3", maschen::Error::Malformed);
test_fails!(arg_count1, "min", maschen::Error::FunctionLen);
test_fails!(arg_count2, "min ( 2 )", maschen::Error::FunctionLen);
test_fails!(arg_count3, "log ( 2 , 3 )", maschen::Error::FunctionLen);
test_fails!(empty_parens, "( )", maschen::Error::Malformed);
test_fails!(
    unbalanced_parens1,
    "log ( 2",
    maschen::Error::UnbalancedParens
);
test_fails!(
    unbalanced_parens2,
    "log ( 2",
    maschen::Error::UnbalancedParens
);
test_fails!(
    unbalanced_parens3,
    "log ( 2 ) )",
    maschen::Error::UnbalancedParens
);
