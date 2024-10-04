use maschen::{self, ShuntingYard, TokenKind};

#[derive(Debug)]
struct StringToken<'a>(&'a str);

impl<'a, 'b> From<&'b StringToken<'a>> for maschen::TokenKind {
    fn from(value: &'b StringToken<'a>) -> Self {
        match value.0 {
            "*" | "/" => TokenKind::Operator(5),
            "+" | "-" => TokenKind::Operator(4),
            "(" => TokenKind::LeftParen,
            ")" => TokenKind::RightParen,
            "sin" | "cos" | "tan" | "log" => TokenKind::Function,
            _ => TokenKind::Value,
        }
    }
}
#[test]
fn test_basic() {
    let mut outstack = vec![];
    let mut opstack = vec![];
    let instring = "3 * ( 2 + log ( 4 ) )";
    let stream = instring.split(' ').map(StringToken);
    let mut yard = ShuntingYard::new(&mut outstack, &mut opstack);
    stream.for_each(|v| yard.process(v).unwrap());
    yard.finish().unwrap();
    dbg!(outstack);
}
