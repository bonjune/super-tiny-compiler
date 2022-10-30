use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub struct Tokenizer<'t> {
    input: &'t str,
    chars: Peekable<CharIndices<'t>>,
}

impl<'t> Tokenizer<'t> {
    pub fn new(input: &'t str) -> Self {
        Self {
            input: input,
            chars: input.char_indices().peekable(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'t> {
    Identifier(&'t str),
    NumLiteral(&'t str),
    Space,
    OpenParen,
    CloseParen,
    AddOperator,
}

impl<'t> Iterator for Tokenizer<'t> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        // Peek the next (utf-8) character
        if let Some(&(i, ch)) = self.chars.peek() {
            match ch {
                '(' => {
                    self.chars.next();
                    return Some(OpenParen);
                }
                ')' => {
                    self.chars.next();
                    return Some(CloseParen);
                }
                ' ' => {
                    self.chars.next();
                    while let Some(&(_, nc)) = self.chars.peek() {
                        if nc != ' ' {
                            break;
                        }
                        self.chars.next();
                    }

                    return Some(Space);
                }
                '+' => {
                    self.chars.next();
                    return Some(AddOperator);
                }

                // `take_while` is problematic because this adaptor
                // 1. comsumes a value, and then
                // 2. check the value is okay with the predicate, and then
                // 3. decides to take it or throw it away
                //
                // I want the adaptor to behave like
                // 1. peek next value, and then
                // 2. check the value is okay with the predicate, and then
                // 3. if okay, then comsume the next value,
                // 4. otherwise, stop advancing the iterator

                // Number Literal
                d if d.is_ascii_digit() => {
                    let (start, _nc) = self.chars.next().unwrap();
                    let mut last = start;
                    while let Some(&(_, c)) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            let (pos, _nc) = self.chars.next().unwrap();
                            last = pos;
                        } else {
                            break;
                        }
                    }
                    return Some(Token::NumLiteral(&self.input[start..last + 1]));
                }

                // Identifier
                c if c.is_ascii_alphabetic() => {
                    let (start, _nc) = self.chars.next().unwrap();
                    let mut last = start;
                    while let Some(&(_, c)) = self.chars.peek() {
                        if c.is_ascii_alphabetic() {
                            let (current, _nc) = self.chars.next().unwrap();
                            last = current;
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Identifier(&self.input[start..last + 1]));
                }
                _ => panic!("unexpected character {} at {}", ch, i),
            }
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};
    use Token::*;

    #[test]
    fn tokenize_parens() {
        let input = "()";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(OpenParen));
        assert_eq!(tokenizer.next(), Some(CloseParen));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn tokenize_number() {
        let mut tokenizer = Tokenizer::new("123");
        assert_eq!(tokenizer.next(), Some(NumLiteral("123")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn tokenize_numbers() {
        let mut tokenizer = Tokenizer::new("123 456 789");
        assert_eq!(tokenizer.next(), Some(NumLiteral("123")));
        assert_eq!(tokenizer.next(), Some(Space));
        assert_eq!(tokenizer.next(), Some(NumLiteral("456")));
        assert_eq!(tokenizer.next(), Some(Space));
        assert_eq!(tokenizer.next(), Some(NumLiteral("789")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn tokenize_numbers_and_parens() {
        let mut tokenizer = Tokenizer::new("123 (456)  789");

        assert_eq!(tokenizer.next(), Some(NumLiteral("123")));
        assert_eq!(tokenizer.next(), Some(Space));
        assert_eq!(tokenizer.next(), Some(OpenParen));
        assert_eq!(tokenizer.next(), Some(NumLiteral("456")));
        assert_eq!(tokenizer.next(), Some(CloseParen));
        assert_eq!(tokenizer.next(), Some(Space));
        assert_eq!(tokenizer.next(), Some(NumLiteral("789")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn tokenize_id() {
        let mut tokenizer = Tokenizer::new("add");

        assert_eq!(tokenizer.next(), Some(Identifier("add")));
        assert_eq!(tokenizer.next(), None);
    }
}
