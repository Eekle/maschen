use std::{cmp::max, marker::PhantomData, path::Iter};

use maschen::{self, ShuntingYard, TokenKind};

#[derive(Default, Debug)]
struct VecStorage<T> {
    output: Vec<T>,
    operators: Vec<T>,
}

impl<T> VecStorage<T> {
    fn new() -> Self {
        Self {
            operators: vec![],
            output: vec![],
        }
    }
}

#[derive(Debug)]
enum CannotFail {}

impl<T> maschen::YardStorage for VecStorage<T> {
    type Item = T;
    type StorageError = CannotFail;

    fn push_to_output_stack(&mut self, value: T) -> Result<(), Self::StorageError> {
        self.output.push(value);
        Ok(())
    }

    fn push_to_operator_stack(&mut self, value: T) -> Result<(), Self::StorageError> {
        self.operators.push(value);
        Ok(())
    }

    fn peek_operator_stack(&self) -> Option<&T> {
        self.operators.last()
    }

    fn pop_operator_stack(&mut self) -> Option<T> {
        self.operators.pop()
    }
}
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
    let mut storage: VecStorage<_> = VecStorage::new();
    let instring = "( / 3 )";
    let stream = instring.split(' ').map(StringToken);
    let mut yard = ShuntingYard::new(&mut storage);
    stream.for_each(|v| yard.process(v).unwrap());
    yard.finish().unwrap();
    dbg!(storage);
}
