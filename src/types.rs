#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Value,
    InfixOperator(usize),
    UnaryOperator,
    Function(usize),
    LeftParen,
    RightParen,
    FnSeparator,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnbalancedParens,
    Malformed,
    StorageFull,
    FunctionLen,
}

pub trait Stack {
    type Item;
    fn push(&mut self, value: Self::Item) -> Result<(), Error>;
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
}

pub trait Token {
    fn kind(&self) -> TokenKind;
}
