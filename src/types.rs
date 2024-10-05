#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Value,
    InfixOperator(usize),
    UnaryOperator,
    Function,
    LeftParen,
    RightParen,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parentheses don't match up")]
    UnbalancedParens,
    #[error("Token stream is malformed")]
    Malformed,
    #[error("Backing storage is full")]
    StorageFull,
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
