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

struct AstBuilder<'a> {
    tokens: Tokenizer<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn from(tokenizer: Tokenizer<'a>) -> AstBuilder {
        Self { tokens: tokenizer }
    }

    pub fn build(mut self) -> Expr<'a> {
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
                    } => {
                        if *right != None {
                            panic!("unexpected identifier")
                        }
                        *right = Some(Box::new(Id { name: id }));
                    }
                    Num { value: _ } | Id { name: _ } => panic!("unexpected identifier"),
                },

                NumLiteral(nstr) => {
                    let num: i32 = nstr.parse().expect("not a number");
                    match tree {
                        Add {
                            left: _,
                            ref mut right,
                        } => {
                            if *right != None {
                                panic!("unexpected number")
                            }
                            *right = Some(Box::new(Num { value: num }));
                        }
                        Program => tree = Num { value: num },
                        Num { value: _ } | Id { name: _ } => panic!("unexpected number"),
                    }
                }

                AddOperator => match tree {
                    Add { left: _, ref right } => {
                        if *right == None {
                            panic!("unexpected operator");
                        }
                        tree = Add {
                            left: Box::new(tree),
                            right: None,
                        }
                    }
                    Num { value: _ } | Id { name: _ } => {
                        tree = Expr::Add {
                            left: Box::new(tree),
                            right: None,
                        };
                    }
                    Program => panic!("unexpected operator"),
                },

                Space => continue,
                OpenParen => todo!(),
                CloseParen => todo!(),
            }
        }

        tree
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast_builder::Expr, tokenizer::Tokenizer};

    use super::AstBuilder;
    use Expr::*;

    #[test]
    fn simple_add() {
        let input = "1 + 2";
        let tokenizer = Tokenizer::new(input);
        let tb = AstBuilder::from(tokenizer);
        let ast = tb.build();
        assert_eq!(
            ast,
            Expr::Add {
                left: Box::new(Num { value: 1 }),
                right: Some(Box::new(Num { value: 2 })),
            }
        );
    }
}
