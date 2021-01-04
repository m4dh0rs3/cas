use super::super::super::expr::atom::symbol::Symbol;
use super::super::super::lexer::{Lexer, LexerErr};

#[derive(Clone, PartialEq)]
pub(crate) enum Op {
    Call(Symbol),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Fact,
    Mod,
    Def,
    Child,
    List,
    Open,
    Close,
    Eq,
    Neq,
    Less,
    More,
    LessEq,
    MoreEq,
}

const DOUBLE_OP: [(char, char); 6] = [
    ('=', '='),
    ('!', '='),
    ('~', '='),
    (':', '='),
    ('<', '='),
    ('>', '='),
];

impl Op {
    pub(crate) fn parse(lexer: &mut Lexer) -> Result<Op, LexerErr> {
        let (at, first) = lexer
            .chars
            .next()
            .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not");

        let string = match lexer.chars.peek() {
            Some(&(_, next)) if DOUBLE_OP.contains(&(first, next)) => {
                let mut string = String::with_capacity(2);

                string.push(first);
                string.push(
                    lexer
                        .chars
                        .next()
                        .expect("Lexer.chars.next() None, but Lexer.chars.peek() was not")
                        .1,
                );

                string
            }
            _ => first.to_string(),
        };

        match Op::from_str(&string) {
            Some(op) => Ok(op),
            None => Err(LexerErr::panic(
                format!("op `{}` is undefined", &string),
                at,
            )),
        }
    }

    fn from_str(input: &str) -> Option<Op> {
        Some(match input {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "^" => Op::Pow,
            "!" => Op::Fact,
            "%" => Op::Mod,
            "<" => Op::Less,
            ">" => Op::More,
            "_" => Op::Child,
            "," | ";" => Op::List,
            "(" | "[" | "{" => Op::Open,
            ")" | "]" | "}" => Op::Close,
            "/" | ":" => Op::Div,
            "!=" | "~=" => Op::Neq,
            ":=" => Op::Def,
            "<=" => Op::LessEq,
            ">=" => Op::MoreEq,
            "=" | "==" => Op::Eq,
            _ => return None,
        })
    }
}
