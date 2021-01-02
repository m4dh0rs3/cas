use std::{fmt, iter::Peekable, str::Chars};

use super::{Def, Env};

pub(crate) struct Lexer<'s, 'e> {
    input: &'s str,
    chars: Peekable<Chars<'s>>,
    env: &'e Env,
    peek: Option<Result<Token, LexerErr>>,
}

const WHITESPACE: &'static str = " \t\n";
const OP_BEGIN: &'static str = "+-*/:^%()[]{}%,;_<>!~=";

impl<'s, 'e> Lexer<'s, 'e> {
    pub(crate) fn new(input: &'s str, env: &'e Env) -> Lexer<'s, 'e> {
        Lexer {
            input,
            chars: input.chars().peekable(),
            env,
            peek: None,
        }
    }

    pub(crate) fn peek(&mut self) -> Result<&Token, &LexerErr> {
        if let None = self.peek {
            self.peek = Some(self.advance());
        }

        match &self.peek {
            Some(peek) => peek.as_ref(),
            None => Err(&LexerErr::EOF),
        }
    }

    pub(crate) fn expect(&mut self, chars: Vec<char>) -> Result<char, LexerErr> {
        if let Some(next) = self.chars.peek().cloned() {
            for possible in &chars {
                if *possible == next {
                    self.chars.next();
                    return Ok(next);
                }
            }
            // TODO: Use Display trait instead?
            Err(LexerErr::Panic(format!(
                "expected `{:?}`, found `{:?}`",
                chars, next
            )))
        } else {
            Err(LexerErr::Panic(format!(
                "expected `{:?}`, but reached EOF",
                chars
            )))
        }
    }

    pub(crate) fn advance(&mut self) -> Result<Token, LexerErr> {
        if let Some(token) = self.peek.take() {
            self.peek = None;
            return token;
        }

        Ok(match self.chars.peek().ok_or(LexerErr::EOF)? {
            first if WHITESPACE.contains(*first) => {
                self.eat(|c| WHITESPACE.contains(c));
                self.advance()?
            }

            first if OP_BEGIN.contains(*first) => Token::Op(Op::parse(self).map_err(|e| {
                LexerErr::Panic(format!(
                    "first character was indicating an operator, but {}",
                    e
                ))
            })?),

            'a'..='z' | 'A'..='Z' | 'α'..='ω' | 'Α'..='Ω' | '0'..='9' | '.' => {
                match Atom::parse(self).map_err(|e| {
                    LexerErr::Panic(format!("first character was indicating an atom, but {}", e))
                })? {
                    Atom::Symbol(symbol) => match self.env.defs.get(&symbol) {
                        Some(Def::OSCall) => Token::Op(Op::Call(symbol)),
                        Some(Def::Call(_, _)) => Token::Op(Op::Call(symbol)),
                        _ => Token::Atom(Atom::Symbol(symbol)),
                    },
                    atom => Token::Atom(atom),
                }
            }

            invalid => {
                return Err(LexerErr::Panic(format!(
                    "found unknown character `{}`",
                    invalid
                )))
            }
        })
    }

    fn eat(&mut self, taste: impl Fn(char) -> bool) -> String {
        let mut food = String::with_capacity(E_TOKEN_LENGTH);

        loop {
            if let Some(possible) = self.chars.peek() {
                if taste(*possible) {
                    food.push(*possible);
                    self.chars.next();
                    continue;
                }
            }

            break;
        }

        food
    }
}

impl<'s, 'e> Iterator for Lexer<'s, 'e> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance().ok()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Token {
    Atom(Atom),
    Op(Op),
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Atom {
    Number(Number),
    Symbol(Symbol),
}

impl Atom {
    fn parse(lexer: &mut Lexer) -> Result<Atom, LexerErr> {
        let first = lexer.chars.peek().ok_or(LexerErr::EOF).map_err(|e| {
            LexerErr::Panic(format!("expected first character of an atom, but {}", e))
        })?;

        Ok(match first {
            '0'..='9' | '.' => Atom::Number(Number::parse(lexer).map_err(|e| {
                LexerErr::Panic(format!("expected first character of a number, but {}", e))
            })?),
            _ => Atom::Symbol(Symbol::parse(lexer).map_err(|e| {
                LexerErr::Panic(format!("expected first character of an symbol, but {}", e))
            })?),
        })
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Op {
    Call(Symbol),
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Fact,
    Mod,
    Eq,
    Neq,
    Big,
    Small,
    BigEq,
    SmallEq,
    Def,
    List,
    Open,
    Close,
    Child,
}

impl Op {
    fn parse(lexer: &mut Lexer) -> Result<Op, LexerErr> {
        let first = lexer.chars.next().ok_or(LexerErr::EOF)?;

        let string: String = match lexer.chars.peek() {
            Some(next) => match (first, next) {
                ('=', '=') | ('!', '=') | ('~', '=') | (':', '=') | ('<', '=') | ('>', '=') => {
                    let mut string = first.to_string();
                    string.push(*next);

                    lexer.chars.next();
                    string
                }
                _ => first.to_string(),
            },
            _ => first.to_string(),
        };

        match Op::from(&string) {
            Some(op) => Ok(op),
            None => Err(LexerErr::Panic(format!("operator `{}` is unknown", string))),
        }
    }

    fn from(input: &str) -> Option<Op> {
        Some(match input {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "^" => Op::Exp,
            "!" => Op::Fact,
            "%" => Op::Mod,
            "<" => Op::Small,
            ">" => Op::Big,
            "_" => Op::Child,
            "," | ";" => Op::List,
            "(" | "[" | "{" => Op::Open,
            ")" | "]" | "}" => Op::Close,
            "/" | ":" => Op::Div,
            "!=" | "~=" => Op::Neq,
            ":=" => Op::Def,
            "<=" => Op::SmallEq,
            ">=" => Op::BigEq,
            "=" | "==" => Op::Eq,
            _ => return None,
        })
    }
}
// Implementing arbitrary precision is too complex for now, see Rep `number`
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Number(pub(crate) f64);

impl Number {
    fn parse(lexer: &mut Lexer) -> Result<Number, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LENGTH);

        string.push_str(&lexer.eat(|c| "+-".contains(c)));
        string.push_str(&lexer.eat(is_digit));

        match lexer.expect(vec!['.']) {
            Ok(c) => {
                string.push(c);
                string.push_str(&lexer.eat(is_digit));
            }
            _ => {}
        }

        match lexer.expect(vec!['e', 'E']) {
            Ok(c) => {
                string.push(c);
                string.push_str(&lexer.eat(|c| "+-".contains(c)));
                string.push_str(&lexer.eat(is_digit));
            }
            _ => {}
        }

        string
            .parse::<f64>()
            .map_err(|_| LexerErr::Panic(format!("could not parse `{}` into `f64`", string)))
            .map(|n| Number(n))
    }
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub(crate) struct Symbol(pub(crate) String);

// Average symbol length, just a optimaziation of the symbol String capacity
const E_TOKEN_LENGTH: usize = 6;

impl Symbol {
    fn parse(lexer: &mut Lexer) -> Result<Symbol, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LENGTH);

        if let Some(c) = lexer.chars.peek() {
            if let 'α'..='ω' | 'Α'..='Ω' = c {
                string.push(*c);
                lexer.chars.next();
                return Ok(Symbol(string));
            }
        }

        loop {
            match lexer.chars.peek() {
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        string.push(*c);
                        lexer.chars.next();
                    }
                    _ => break,
                },
                _ => break,
            }

            // TODO: This is horrible!
            /* if lexer.env.defs.contains_key(&Symbol(string.clone())) {
                break;
            } */
            // Solved by commenting ;)
        }

        Ok(Symbol(string))
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum LexerErr {
    EOF,
    Panic(String),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{1b}[31m{}\u{1b}[0m", self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{1b}[34m{}\u{1b}[0m", self.0)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{1b}[1m\u{1b}[32m{}\u{1b}[0m",
            match self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
                Op::Exp => "^",
                Op::Fact => "!",
                Op::Mod => "%",
                Op::Eq => "=",
                Op::Neq => "!=",
                Op::Def => ":=",
                Op::Small => "<",
                Op::Big => ">",
                Op::SmallEq => "<=",
                Op::BigEq => ">=",
                Op::List => ";",
                Op::Open => "{",
                Op::Close => "}",
                Op::Child => "_",
                Op::Call(call) => &call.0,
            }
        )
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Number(number) => write!(f, "{}", number),
            Atom::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}

impl fmt::Display for LexerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerErr::Panic(panic) => write!(f, "{}", panic),
            LexerErr::EOF => write!(f, "reached EOF"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Atom(atom) => write!(f, "{}", atom),
            Token::Op(op) => write!(f, "{}", op),
        }
    }
}
