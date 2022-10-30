use std::num::{IntErrorKind, ParseIntError};

use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'e> {
    Num {
        value: i32,
    },
    Add {
        left: Box<Expr<'e>>,
        right: Option<Box<Expr<'e>>>,
    },
    Id {
        name: &'e str,
    },
    Program,
}

#[derive(Debug)]
struct AstBuilder<'a> {
    tokens: Tokenizer<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError<'err> {
    Unexpected(Token<'err>),
    WrongFormat(IntErrorKind),
}

impl<'err> From<ParseIntError> for SyntaxError<'err> {
    fn from(err: ParseIntError) -> Self {
        SyntaxError::WrongFormat(err.kind().to_owned())
    }
}

impl<'a> AstBuilder<'a> {
    pub fn from(tokenizer: Tokenizer<'a>) -> AstBuilder {
        Self { tokens: tokenizer }
    }

    pub fn build(mut self) -> Result<Expr<'a>, SyntaxError<'a>> {
        use Expr::*;

        let mut tree = Program;
        while let Some(token) = self.tokens.next() {
            use Token::*;
            match token {
                Identifier(id) => match tree {
                    Program => tree = Id { name: id },
                    Add {
                        left: _,
                        ref mut right,
                    } => match *right {
                        Some(_) => return Err(SyntaxError::Unexpected(token)),
                        None => *right = Some(Box::new(Id { name: id })),
                    },
                    Num { value: _ } | Id { name: _ } => {
                        return Err(SyntaxError::Unexpected(token))
                    }
                },

                NumLiteral(nstr) => {
                    let num: i32 = nstr.parse()?;
                    match tree {
                        Add {
                            left: _,
                            ref mut right,
                        } => match *right {
                            Some(_) => return Err(SyntaxError::Unexpected(token)),
                            None => *right = Some(Box::new(Num { value: num })),
                        },
                        Program => tree = Num { value: num },
                        Num { value: _ } | Id { name: _ } => panic!("unexpected number"),
                    }
                }

                AddOperator => match tree {
                    Add { left: _, ref right } => match right {
                        None => return Err(SyntaxError::Unexpected(token)),
                        Some(_) => {
                            tree = Add {
                                left: Box::new(tree),
                                right: None,
                            }
                        }
                    },
                    Num { value: _ } | Id { name: _ } => {
                        tree = Expr::Add {
                            left: Box::new(tree),
                            right: None,
                        };
                    }
                    Program => return Err(SyntaxError::Unexpected(token)),
                },

                Space => continue,
                OpenParen => todo!(),
                CloseParen => todo!(),
            }
        }

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast_builder::Expr, tokenizer::Tokenizer};

    use super::{AstBuilder, SyntaxError, Token};

    use Expr::*;
    use Token::*;

    #[test]
    fn simple_add() -> Result<(), SyntaxError<'static>> {
        let input = "1 + 2";
        let tokenizer = Tokenizer::new(input);
        let tb = AstBuilder::from(tokenizer);
        let ast = tb.build()?;
        assert_eq!(
            ast,
            Expr::Add {
                left: Box::new(Num { value: 1 }),
                right: Some(Box::new(Num { value: 2 })),
            }
        );

        Ok(())
    }

    #[test]
    fn simple_add_2() -> Result<(), SyntaxError<'static>> {
        let input = "1 + 2 + 3";
        let tokenizer = Tokenizer::new(input);
        let tb = AstBuilder::from(tokenizer);
        let ast = tb.build()?;
        assert_eq!(
            ast,
            Add {
                left: Box::new(Add {
                    left: Box::new(Num { value: 1 }),
                    right: Some(Box::new(Num { value: 2 })),
                }),
                right: Some(Box::new(Num { value: 3 }))
            }
        );

        Ok(())
    }

    #[test]
    fn unexpected_operator() {
        let input = "+ 3";
        let tokenizer = Tokenizer::new(input);
        let astb = AstBuilder::from(tokenizer);
        let ast = astb.build();

        assert_eq!(ast, Err(SyntaxError::Unexpected(AddOperator)));
    }
}
