#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod types;

use types::TokenKind as TK;
pub use types::{Error, Stack, Token, TokenKind};

pub struct ShuntingYard<Out, Op, Fun> {
    /// The kind of the last processed token
    last_token_kind: Option<TK>,
    /// The running count of how many items will be
    /// on the runtime stack if this computation
    /// is processed. At the end, this must be 1
    /// for the expression to be valid.
    final_value_count: usize,
    outstack: Out,
    opstack: Op,
    fnstack: Fun,
}

impl<T, Out, Op, Fun> ShuntingYard<Out, Op, Fun>
where
    Out: Stack<Item = T>,
    Op: Stack<Item = T>,
    Fun: Stack<Item = usize>,
    T: types::Token,
{
    pub fn new_with_storage(outstack: Out, opstack: Op, fnstack: Fun) -> Self {
        Self {
            last_token_kind: None,
            final_value_count: 0,
            outstack,
            opstack,
            fnstack,
        }
    }

    /// .Check if two tokens can follow each other
    ///
    /// # Errors
    ///
    /// If the sequence of tokens is illegal
    fn check_adjacency(before: Option<TK>, after: TK) -> Result<(), Error> {
        let is_okay = match before {
            Some(TK::Function(_)) => matches!(after, TK::LeftParen),
            None
            | Some(TK::InfixOperator(_) | TK::UnaryOperator | TK::LeftParen | TK::FnSeparator) => {
                matches!(
                    after,
                    TK::Function(_) | TK::LeftParen | TK::UnaryOperator | TK::Value
                )
            }
            Some(TK::RightParen | TK::Value) => {
                matches!(
                    after,
                    TK::InfixOperator(_) | TK::RightParen | TK::FnSeparator
                )
            }
        };
        if is_okay {
            Ok(())
        } else {
            Err(Error::Malformed)
        }
    }

    /// Push a value to the output stack, while tracking the
    /// state of the computation thus far.
    ///
    ///
    /// # Errors
    ///
    /// Returns an error if the push would generate a nonsensical
    /// computation, such as an operator that doesn't have enough
    /// arguments or a function that doesn't have the right number
    fn push_to_output_stack(&mut self, v: T) -> Result<(), Error> {
        let kind = v.kind();
        match kind {
            TK::Value => self.final_value_count += 1,
            TK::InfixOperator(_) => {
                if self.final_value_count < 2 {
                    return Err(Error::Malformed);
                }
                self.final_value_count -= 1
            }
            TK::UnaryOperator => {
                if self.final_value_count < 1 {
                    return Err(Error::Malformed);
                }
            }
            TK::Function(n) => {
                if n == 0 || self.final_value_count < n {
                    return Err(Error::FunctionLen);
                }
                match self.fnstack.pop() {
                    None => return Err(Error::Malformed),
                    Some(1) => {}
                    Some(_) => return Err(Error::FunctionLen),
                }
                self.final_value_count -= n - 1;
            }
            _ => return Err(Error::Internal),
        }
        self.outstack.push(v)?;
        Ok(())
    }

    fn pop_operators_while(&mut self, predicate: impl Fn(TK) -> bool) -> Result<(), Error> {
        while let Some(popped) = self.opstack.pop() {
            if predicate(popped.kind()) {
                self.push_to_output_stack(popped)?;
            } else {
                return self.opstack.push(popped);
            }
        }
        Ok(())
    }

    pub fn process(&mut self, token: T) -> Result<(), Error> {
        let kind = token.kind();
        Self::check_adjacency(self.last_token_kind, kind)?;
        self.last_token_kind = Some(kind);

        match kind {
            TK::Value => self.push_to_output_stack(token)?,
            TK::Function(n) => {
                self.opstack.push(token)?;
                self.fnstack.push(n)?;
            }
            TK::InfixOperator(o1_p) => {
                self.pop_operators_while(|t| {
                    matches!(t, TK::UnaryOperator) || matches!(t, TK::InfixOperator(v) if v <= o1_p)
                })?;
                self.opstack.push(token)?;
            }
            TK::FnSeparator => {
                match self.fnstack.pop() {
                    None => return Err(Error::Malformed),
                    Some(0) => return Err(Error::FunctionLen),
                    Some(n) => self.fnstack.push(n - 1)?,
                };
                self.pop_operators_while(|t| !matches!(t, TK::LeftParen))?;
            }
            TK::UnaryOperator => self.opstack.push(token)?,
            TK::LeftParen => self.opstack.push(token)?,
            TK::RightParen => {
                self.pop_operators_while(|t| !matches!(t, TK::LeftParen))?;
                if !self
                    .opstack
                    .pop()
                    .is_some_and(|v| matches!(v.kind(), TK::LeftParen))
                {
                    return Err(Error::UnbalancedParens);
                }
                self.pop_operators_while(|t| matches!(t, TK::Function(_)))?;
            }
        };
        Ok(())
    }

    /// Mark the input stream as complete. Returns the output stack.
    pub fn finish(mut self) -> Result<Out, Error> {
        while let Some(v) = self.opstack.pop() {
            match v.kind() {
                TK::LeftParen => return Err(Error::UnbalancedParens),
                _ => self.push_to_output_stack(v)?,
            };
        }
        if self.final_value_count != 1 {
            return Err(Error::Malformed);
        }
        Ok(self.outstack)
    }
}
