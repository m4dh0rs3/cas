use {
    super::super::lexer::{Lexer, LexerErr},
    number::Number,
    symbol::Symbol,
};

pub(crate) mod number;
pub(crate) mod symbol;

/// A leaf node, containing a actual value.
#[derive(Clone, PartialEq)]
pub enum Atom {
    Number(Number),
    Symbol(Symbol),
}

impl Atom {
    pub(crate) fn parse(lexer: &mut Lexer) -> Result<Atom, LexerErr> {
        let (_, first) = lexer
            .chars
            .peek()
            .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not");

        Ok(match first {
            '0'..='9' | '.' => Atom::Number(Number::parse(lexer)?),
            _ => Atom::Symbol(Symbol::parse(lexer)?),
        })
    }
}
