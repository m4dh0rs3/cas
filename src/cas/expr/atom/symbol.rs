use super::super::super::lexer::{Lexer, LexerErr, E_TOKEN_LEN};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub(crate) String);

impl Symbol {
    pub(crate) fn parse(lexer: &mut Lexer) -> Result<Symbol, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LEN);

        if let Some((_, c)) = lexer.chars.peek() {
            if let 'α'..='ω' | 'Α'..='Ω' = c {
                string.push(
                    lexer
                        .chars
                        .next()
                        .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not")
                        .1,
                );
                return Ok(Symbol(string));
            }
        }

        loop {
            match lexer.chars.peek() {
                Some((_, c)) => match c {
                    'a'..='z' | 'A'..='Z' => string.push(
                        lexer
                            .chars
                            .next()
                            .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not")
                            .1,
                    ),
                    _ => break,
                },
                None => break,
            }

            // check if defined here
            /* if lexer.env.contains_key(&Symbol(string.clone())) {
                break;
            } */
        }

        Ok(Symbol(string))
    }
}
