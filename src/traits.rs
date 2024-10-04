#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Value,
    Operator(usize),
    Function,
    LeftParen,
    RightParen,
}

pub trait YardStorage {
    type Item;
    type StorageError;
    fn push_to_output_stack(&mut self, value: Self::Item) -> Result<(), Self::StorageError>;
    fn push_to_operator_stack(&mut self, value: Self::Item) -> Result<(), Self::StorageError>;
    fn peek_operator_stack(&self) -> Option<&Self::Item>;
    fn pop_operator_stack(&mut self) -> Option<Self::Item>;
}
