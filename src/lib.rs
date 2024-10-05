mod impls;
mod types;

use types::TokenKind as TK;
pub use types::{Error, Stack, Token, TokenKind};

pub struct ShuntingYard<'a, Out, Op> {
    last_token_kind: Option<TK>,
    stack_size: usize,
    outstack: &'a mut Out,
    opstack: &'a mut Op,
}

impl<'a, Token, Out, Op> ShuntingYard<'a, Out, Op>
where
    Out: Stack<Item = Token>,
    Op: Stack<Item = Token>,
    Token: types::Token,
{
    pub fn new(outstack: &'a mut Out, opstack: &'a mut Op) -> Self {
        Self {
            last_token_kind: None,
            stack_size: 0,
            outstack,
            opstack,
        }
    }

    fn check_adjacency(before: Option<TK>, after: TK) -> Result<(), Error> {
        let is_okay = match before {
            None => matches!(
                after,
                TK::Function | TK::LeftParen | TK::UnaryOperator | TK::Value
            ),
            Some(TK::Function) => matches!(after, TK::LeftParen),
            Some(TK::InfixOperator(_) | TK::UnaryOperator | TK::LeftParen) => matches!(
                after,
                TK::Function | TK::LeftParen | TK::UnaryOperator | TK::Value
            ),
            Some(TK::RightParen | TK::Value) => {
                matches!(after, TK::InfixOperator(_) | TK::RightParen)
            }
        };
        if is_okay {
            Ok(())
        } else {
            Err(Error::Malformed)
        }
    }

    fn push_to_output_stack(&mut self, v: Token) -> Result<(), Error> {
        let kind = v.kind();
        match kind {
            TK::Value => self.stack_size += 1,
            TK::InfixOperator(_) => {
                if self.stack_size < 2 {
                    return Err(Error::Malformed);
                }
                self.stack_size -= 1
            }
            TK::UnaryOperator => {
                if self.stack_size < 1 {
                    return Err(Error::Malformed);
                }
            }
            TK::Function => {
                if self.stack_size < 1 {
                    return Err(Error::Malformed);
                }
            }
            TK::LeftParen | TK::RightParen => {
                panic!("Can't push parens to the output")
            }
        }
        self.outstack.push(v)?;
        Ok(())
    }

    fn pop_operators_while(&mut self, predicate: impl Fn(TK) -> bool) -> Result<(), Error> {
        while self.opstack.peek().is_some_and(|v| predicate(v.kind())) {
            let popped = self.opstack.pop().unwrap();
            self.push_to_output_stack(popped)?
        }
        Ok(())
    }

    pub fn process(&mut self, token: Token) -> Result<(), Error> {
        let kind = token.kind();
        Self::check_adjacency(self.last_token_kind, kind)?;
        self.last_token_kind = Some(kind);

        match kind {
            TK::Value => self.push_to_output_stack(token)?,
            TK::Function => self.opstack.push(token)?,
            TK::InfixOperator(o1_p) => {
                self.pop_operators_while(|t| {
                    matches!(t, TK::UnaryOperator) || matches!(t, TK::InfixOperator(v) if v <= o1_p)
                })?;
                self.opstack.push(token)?;
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
            }
        };
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), Error> {
        while let Some(v) = self.opstack.pop() {
            match v.kind() {
                TK::LeftParen => return Err(Error::UnbalancedParens),
                _ => self.push_to_output_stack(v)?,
            };
        }
        if self.stack_size != 1 {
            return Err(Error::Malformed);
        }
        Ok(())
    }
}
