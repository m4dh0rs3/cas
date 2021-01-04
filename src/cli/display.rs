use std::fmt;

use crate::cas::expr::call::{op::Op, Call};
use crate::cas::expr::{
    atom::{number::Number, symbol::Symbol, Atom},
    Expr,
};
use crate::cas::lexer::LexerErr;
use crate::cas::lexer::Token;
use crate::cas::parser::ParserErr;

impl fmt::Display for LexerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // maybe before: "could not parse token: "
            LexerErr::Panic { msg, at } => write!(f, "{}", msg),
            LexerErr::EOF => write!(f, "reached `EOF`"),
        }
    }
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErr::Panic(msg) => write!(f, "{}", msg),
            ParserErr::LexerErr(msg) => write!(f, "{}", msg),
            ParserErr::Empty => write!(f, "empty"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(atom) => write!(f, "{}", atom),
            Expr::Call(call) => write!(f, "{}", call),
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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Number(number) => write!(f, "{}", number),
            Atom::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op {
            Op::Call(symbol) => write!(f, "{}", self.fmt_call()),
            Op::Fact => write!(
                f,
                "{}{}",
                self.args.first().expect("expected function argument"),
                Op::Fact
            ),
            _ => match self.args.len() {
                1 => write!(
                    f,
                    "{}{}",
                    self.op,
                    self.args.first().expect("expected function argument")
                ),
                2 => write!(
                    f,
                    "{}{}{}",
                    self.args.first().expect("expected function argument"),
                    self.op,
                    self.args.get(1).expect("expected function argument")
                ),
                _ => write!(f, "{}", self.fmt_call()),
            },
        }
    }
}

impl Call {
    fn fmt_call(&self) -> String {
        let mut string = String::new();

        string.push_str(&format!("{}(", self.op));

        if let Some(expr) = self.args.first() {
            string.push_str(&format!("{}", expr));
        }

        for expr in self.args.iter().skip(1) {
            string.push_str(&format!(", {}", expr));
        }

        string.push_str(")");

        string
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
                Op::Pow => "^",
                Op::Fact => "!",
                Op::Mod => "%",
                Op::Eq => "=",
                Op::Neq => "!=",
                Op::Def => ":=",
                Op::Less => "<",
                Op::More => ">",
                Op::LessEq => "<=",
                Op::MoreEq => ">=",
                Op::List => ";",
                Op::Open => "{",
                Op::Close => "}",
                Op::Child => "_",
                Op::Call(string) => &string.0,
            }
        )
    }
}
