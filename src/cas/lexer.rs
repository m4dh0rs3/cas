use {
    super::{
        env::{Def, Env},
        expr::{atom::Atom, call::op::Op},
        parser::ParserErr,
    },
    std::{iter::Peekable, str::CharIndices},
};

pub(crate) struct Lexer<'s, 'e> {
    input: &'s str,
    pub(crate) chars: Peekable<CharIndices<'s>>,
    env: &'e Env,
    peek: Option<Result<Token, LexerErr>>,
}

#[derive(PartialEq)]
pub(crate) enum LexerErr {
    EOF,
    Panic { msg: String, at: usize },
}

#[derive(PartialEq)]
pub(crate) enum Token {
    Atom(Atom),
    Op(Op),
}

const WHITESPACE: &'static str = " \t\n";
const OP_BEGIN: &'static str = "+-*/:^%()[]{},;_<>!~=";
pub(crate) const DEC_DIGITS: &'static str = "0123456789";
pub(crate) const HEX_DIGITS: &'static str = "0123456789ABCDEF";
pub(crate) const BIN_DIGITS: &'static str = "01";
pub(crate) const E_TOKEN_LEN: usize = 6;

impl<'s, 'e> Lexer<'s, 'e> {
    pub(crate) fn new(input: &'s str, env: &'e Env) -> Lexer<'s, 'e> {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            env,
            peek: None,
        }
    }

    pub(crate) fn peek(&mut self) -> Result<&Token, &LexerErr> {
        if let None = self.peek {
            self.peek = Some(self.token());
        }

        match &self.peek {
            Some(peek) => peek.as_ref(),
            None => panic!("Lexer::peek still None after definition"),
        }
    }

    pub(crate) fn expect(&mut self, chars: Vec<char>) -> Result<(usize, char), LexerErr> {
        if let Some((at, next)) = self.chars.peek() {
            for pot in &chars {
                if pot == next {
                    return Ok(self
                        .chars
                        .next()
                        .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not"));
                }
            }

            // TODO: Not using debug here
            Err(LexerErr::panic(
                format!("expected `{:?}`, found `{}`", &chars, next),
                *at,
            ))
        } else {
            Err(LexerErr::panic(
                format!("expected `{:?}`, but {}", &chars, LexerErr::EOF),
                self.input.len() - 1,
            ))
        }
    }

    pub(crate) fn token(&mut self) -> Result<Token, LexerErr> {
        if let Some(token) = self.peek.take() {
            self.peek = None;
            return token;
        }

        let (at, first) = self.chars.peek().ok_or(LexerErr::EOF)?;

        match first {
            &first if WHITESPACE.contains(first) => {
                self.eat(|pot| WHITESPACE.contains(*pot));
                self.token()
            }

            &first if OP_BEGIN.contains(first) => Ok(Token::Op(Op::parse(self)?)),

            'a'..='z' | 'A'..='Z' | 'α'..='ω' | 'Α'..='Ω' | '0'..='9' | '.' => {
                Ok(match Atom::parse(self)? {
                    Atom::Symbol(symbol) => match self.env.0.get(&symbol) {
                        Some(Def::OSCall) => Token::Op(Op::Call(symbol)),
                        Some(Def::Call { args: _, call: _ }) => Token::Op(Op::Call(symbol)),
                        _ => Token::Atom(Atom::Symbol(symbol)),
                    },
                    atom => Token::Atom(atom),
                })
            }

            _ => Err(LexerErr::panic(format!("unknown char `{}`", first), *at)),
        }
    }

    pub(crate) fn eat(&mut self, taste: impl Fn(&char) -> bool) -> String {
        let mut food = String::with_capacity(E_TOKEN_LEN);

        loop {
            if let Some((_, pot)) = self.chars.peek() {
                if taste(pot) {
                    food.push(
                        self.chars
                            .next()
                            .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not")
                            .1,
                    );
                    continue;
                }
            }

            break;
        }

        food
    }

    pub(crate) fn start(&mut self) -> usize {
        self.chars
            .peek()
            .map(|ci| ci.0)
            .unwrap_or_else(|| self.input.len() - 1)
    }
}

impl LexerErr {
    pub(crate) fn panic(msg: String, at: usize) -> LexerErr {
        LexerErr::Panic { msg, at }
    }

    pub(crate) fn parser_err(self) -> ParserErr {
        ParserErr::LexerErr(self)
    }
}
