mod traits;

use std::ops::Not;

pub use traits::{TokenKind, YardStorage};

#[derive(Debug)]
pub enum ShuntError<T> {
    UnbalancedParens,
    Malformed,
    Storage(T),
}

impl<T> From<T> for ShuntError<T> {
    fn from(value: T) -> Self {
        ShuntError::Storage(value)
    }
}

pub struct ShuntingYard<'a, Storage> {
    last_token_kind: Option<TokenKind>,
    stack_size: usize,
    yard: &'a mut Storage,
}

impl<'a, Storage> ShuntingYard<'a, Storage>
where
    Storage: YardStorage,
    for<'i> &'i Storage::Item: Into<TokenKind>,
{
    pub fn new(yard: &'a mut Storage) -> Self {
        Self {
            yard,
            stack_size: 0,
            last_token_kind: None,
        }
    }

    fn push_to_output_stack(
        &mut self,
        v: Storage::Item,
    ) -> Result<(), ShuntError<Storage::StorageError>> {
        match (&v).into() {
            TokenKind::Value => self.stack_size += 1,
            TokenKind::Operator(_) => {
                if self.stack_size < 2 {
                    return Err(ShuntError::Malformed);
                }
                self.stack_size -= 1
            }
            TokenKind::Function => {
                if self.stack_size < 1 {
                    return Err(ShuntError::Malformed);
                }
            }
            TokenKind::LeftParen | TokenKind::RightParen => {
                panic!("Can't push parens to the output")
            }
        }
        self.yard.push_to_output_stack(v)?;
        Ok(())
    }

    pub fn process(
        &mut self,
        token: Storage::Item,
    ) -> Result<(), ShuntError<Storage::StorageError>> {
        let kind = (&token).into();

        match (self.last_token_kind, kind) {
            (Some(TokenKind::LeftParen), TokenKind::Operator(_)) => {
                return Err(ShuntError::Malformed)
            }
            (Some(TokenKind::Operator(_)), TokenKind::RightParen) => {
                return Err(ShuntError::Malformed);
            }
            (Some(TokenKind::Value), TokenKind::Value) => return Err(ShuntError::Malformed),
            (Some(TokenKind::Operator(_)), TokenKind::Operator(_)) => {
                return Err(ShuntError::Malformed)
            }
            (Some(TokenKind::Function), x) if !matches!(x, TokenKind::LeftParen) => {
                return Err(ShuntError::Malformed)
            }
            (_, _) => {}
        };

        self.last_token_kind = Some(kind);

        match kind {
            TokenKind::Value => self.push_to_output_stack(token)?,
            TokenKind::Function => self.yard.push_to_operator_stack(token)?,
            TokenKind::Operator(o1_p) => {
                while let Some(top_of_op_stack) = self.yard.peek_operator_stack() {
                    match top_of_op_stack.into() {
                        TokenKind::Operator(o2_p) if (o2_p >= o1_p) => {
                            let popped = self.yard.pop_operator_stack().unwrap();
                            self.push_to_output_stack(popped)?;
                        }
                        _ => break,
                    }
                }
                self.yard.push_to_operator_stack(token)?;
            }
            TokenKind::LeftParen => self.yard.push_to_operator_stack(token)?,
            TokenKind::RightParen => {
                while let Some(top_of_op_stack) = self.yard.peek_operator_stack() {
                    match top_of_op_stack.into() {
                        TokenKind::LeftParen => {
                            break;
                        }
                        _ => {
                            let popped = self.yard.pop_operator_stack().unwrap();
                            self.push_to_output_stack(popped)?
                        }
                    };
                }
                if !self
                    .yard
                    .pop_operator_stack()
                    .is_some_and(|v| matches!((&v).into(), TokenKind::LeftParen))
                {
                    return Err(ShuntError::UnbalancedParens);
                }
                if self
                    .yard
                    .peek_operator_stack()
                    .is_some_and(|v| matches!(v.into(), TokenKind::Function))
                {
                    let popped = self.yard.pop_operator_stack().unwrap();
                    self.push_to_output_stack(popped)?;
                }
            }
        };
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), ShuntError<Storage::StorageError>> {
        while let Some(v) = self.yard.pop_operator_stack() {
            match (&v).into() {
                traits::TokenKind::LeftParen => return Err(ShuntError::UnbalancedParens),
                _ => self.push_to_output_stack(v)?,
            };
        }
        if self.stack_size != 1 {
            return Err(ShuntError::Malformed);
        }
        Ok(())
    }
}
