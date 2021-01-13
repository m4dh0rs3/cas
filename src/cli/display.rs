use {
    crate::cas::{
        expr::{
            atom::{number::Number, symbol::Symbol, Atom},
            call::{op::Op, Call},
            Expr, TypeErr,
        },
        lexer::{LexerErr, Token},
        parser::ParserErr,
    },
    std::fmt,
};

impl fmt::Display for LexerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse token: ")?;

        match self {
            // maybe before: "could not parse token: "
            LexerErr::Panic { msg, at } => {
                write!(f, "{}", msg)
            }
            LexerErr::EOF => write!(f, "reached `{}`", &Op::Call(Symbol(String::from("EOF")))),
        }
    }
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse expr: ")?;

        match self {
            ParserErr::Panic(msg) => write!(f, "{}", msg),
            ParserErr::LexerErr(msg) => write!(f, "{}", msg),
            ParserErr::Empty => write!(f, "empty"),
        }
    }
}

impl fmt::Display for TypeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(atom) => write!(f, "{:?}", atom),
            Expr::Call(call) => write!(f, "{:?}", call),
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

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Number(number) => write!(f, "{:?}", number),
            Atom::Symbol(symbol) => write!(f, "{:?}", symbol),
        }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(\u{1b}[0m{}", self.op)?;
        for expr in &self.args {
            write!(f, " {}", expr)?
        }
        write!(f, ")\u{1b}[0m")
    }
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}", self.op)?;
        for expr in &self.args {
            write!(f, " {:?}", expr)?
        }
        write!(f, ")")
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{1b}[91m{}\u{1b}[0m", self.0)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{1b}[96m{}\u{1b}[0m", self.0)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{1b}[92;1m{:?}\u{1b}[0m", self)
    }
}

impl fmt::Debug for Op {
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
                Op::Open => "(",
                Op::Close => ")",
                Op::Child => "_",
                Op::Call(string) => &string.0,
            }
        )
    }
}
