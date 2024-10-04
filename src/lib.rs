mod impls;
mod types;

use types::Error;
pub use types::{Stack, TokenKind};

pub struct ShuntingYard<'a, Out, Op> {
    last_token_kind: Option<TokenKind>,
    stack_size: usize,
    outstack: &'a mut Out,
    opstack: &'a mut Op,
}

impl<'a, Token, Out, Op> ShuntingYard<'a, Out, Op>
where
    Out: Stack<Item = Token>,
    Op: Stack<Item = Token>,
    for<'i> &'i Token: Into<TokenKind>,
{
    pub fn new(outstack: &'a mut Out, opstack: &'a mut Op) -> Self {
        Self {
            last_token_kind: None,
            stack_size: 0,
            outstack,
            opstack,
        }
    }

    fn push_to_output_stack(&mut self, v: Token) -> Result<(), Error> {
        match (&v).into() {
            TokenKind::Value => self.stack_size += 1,
            TokenKind::Operator(_) => {
                if self.stack_size < 2 {
                    return Err(Error::Malformed);
                }
                self.stack_size -= 1
            }
            TokenKind::Function => {
                if self.stack_size < 1 {
                    return Err(Error::Malformed);
                }
            }
            TokenKind::LeftParen | TokenKind::RightParen => {
                panic!("Can't push parens to the output")
            }
        }
        self.outstack.push(v)?;
        Ok(())
    }

    pub fn process(&mut self, token: Token) -> Result<(), Error> {
        let kind = (&token).into();

        match (self.last_token_kind, kind) {
            (Some(TokenKind::LeftParen), TokenKind::Operator(_)) => return Err(Error::Malformed),
            (Some(TokenKind::Operator(_)), TokenKind::RightParen) => {
                return Err(Error::Malformed);
            }
            (Some(TokenKind::Value), TokenKind::Value) => return Err(Error::Malformed),
            (Some(TokenKind::Operator(_)), TokenKind::Operator(_)) => return Err(Error::Malformed),
            (Some(TokenKind::Function), x) if !matches!(x, TokenKind::LeftParen) => {
                return Err(Error::Malformed)
            }
            (_, _) => {}
        };

        self.last_token_kind = Some(kind);

        match kind {
            TokenKind::Value => self.push_to_output_stack(token)?,
            TokenKind::Function => self.opstack.push(token)?,
            TokenKind::Operator(o1_p) => {
                while let Some(top_of_op_stack) = self.opstack.peek() {
                    match top_of_op_stack.into() {
                        TokenKind::Operator(o2_p) if (o2_p >= o1_p) => {
                            let popped = self.opstack.pop().unwrap();
                            self.push_to_output_stack(popped)?;
                        }
                        _ => break,
                    }
                }
                self.opstack.push(token)?;
            }
            TokenKind::LeftParen => self.opstack.push(token)?,
            TokenKind::RightParen => {
                while let Some(top_of_op_stack) = self.opstack.peek() {
                    match top_of_op_stack.into() {
                        TokenKind::LeftParen => {
                            break;
                        }
                        _ => {
                            let popped = self.opstack.pop().unwrap();
                            self.push_to_output_stack(popped)?
                        }
                    };
                }
                if !self
                    .opstack
                    .pop()
                    .is_some_and(|v| matches!((&v).into(), TokenKind::LeftParen))
                {
                    return Err(Error::UnbalancedParens);
                }
                if self
                    .opstack
                    .peek()
                    .is_some_and(|v| matches!(v.into(), TokenKind::Function))
                {
                    let popped = self.opstack.pop().unwrap();
                    self.push_to_output_stack(popped)?;
                }
            }
        };
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), Error> {
        while let Some(v) = self.opstack.pop() {
            match (&v).into() {
                types::TokenKind::LeftParen => return Err(Error::UnbalancedParens),
                _ => self.push_to_output_stack(v)?,
            };
        }
        if self.stack_size != 1 {
            return Err(Error::Malformed);
        }
        Ok(())
    }
}
