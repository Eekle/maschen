#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // A value
    Value,
    // A left associative infix operator. Contains its priority. Lower values are higher priority.
    InfixOperator(usize),
    // A unary operator.
    UnaryOperator,
    // A function. Contains the number of arguments it takes (must be 1 or more).
    Function(usize),
    // A left parentheses. Used for grouping and function arguments.
    LeftParen,
    // A right parentheses.
    RightParen,
    // A separator between function arguments
    FnSeparator,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Left and right parens didn't match
    UnbalancedParens,
    /// Input stream was not well formed
    Malformed,
    /// Could not push to a stack
    StorageFull,
    /// Function has the wrong number of arguments
    FunctionLen,
    /// The internal logic of the library has failed
    Internal,
}

pub trait Stack {
    type Item;
    fn push(&mut self, value: Self::Item) -> Result<(), Error>;
    fn pop(&mut self) -> Option<Self::Item>;
}

/// A type that can be processed by a shunting yard
pub trait Token {
    fn kind(&self) -> TokenKind;
}
