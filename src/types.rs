#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Value,
    Operator(usize),
    Function,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Error {
    UnbalancedParens,
    Malformed,
    StorageFull,
}

pub trait Stack {
    type Item;
    fn push(&mut self, value: Self::Item) -> Result<(), Error>;
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
}
