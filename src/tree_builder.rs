use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'e> {
    Num(i32),
    Add {
        left: Box<Expr<'e>>,
        right: Option<Box<Expr<'e>>>,
    },
    Id(&'e str),
    Program,
}

struct TreeBuilder<'a> {
    tokens: Tokenizer<'a>,
}

impl<'a> TreeBuilder<'a> {
    pub fn from(tokenizer: Tokenizer<'a>) -> TreeBuilder {
        Self { tokens: tokenizer }
    }

    pub fn build(mut self) -> Expr<'a> {
        use Expr::*;

        let mut tree = Program;
        while let Some(token) = self.tokens.next() {
            use Token::*;
            match token {
                Identifier(id) => match tree {
                    Program => tree = Id(id),
                    Add {
                        left: _,
                        ref mut right,
                    } => {
                        if *right != None {
                            panic!("unexpected identifier")
                        }
                        *right = Some(Box::new(Id(id)));
                    }
                    Num(_) | Id(_) => panic!("unexpected identifier"),
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
                            *right = Some(Box::new(Num(num)));
                        }
                        Program => tree = Num(num),
                        Num(_) | Id(_) => panic!("unexpected number"),
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
                    Id(_) | Num(_) => {
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
    use crate::{tokenizer::Tokenizer, tree_builder::Expr};

    use super::TreeBuilder;

    #[test]
    fn simple_add() {
        let input = "1 + 2";
        let tokenizer = Tokenizer::new(input);
        let tb = TreeBuilder::from(tokenizer);
        let ast = tb.build();
        assert_eq!(
            ast,
            Expr::Add {
                left: Box::new(Expr::Num(1)),
                right: Some(Box::new(Expr::Num(2))),
            }
        );
    }
}
