use super::{
    lexer::{Atom, Lexer, LexerErr, Number, Op, Symbol, Token},
    Def, Env,
};
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Atom(Atom),
    Call(Call),
}

#[derive(Debug, Clone)]
pub(crate) struct Call {
    op: Op,
    args: Vec<Expr>,
}

impl Call {
    pub(crate) fn new(op: Op, args: Vec<Expr>) -> Call {
        Call { op, args }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum ParserErr {
    Panic(String),
    LexerErr(LexerErr),
    Empty,
}

impl Expr {
    pub(crate) fn new(input: &str, env: &Env) -> Result<Expr, ParserErr> {
        let mut lexer = Lexer::new(input, env);
        if let Err(LexerErr::EOF) = lexer.peek() {
            return Err(ParserErr::Empty);
        }

        Expr::new_bp(&mut lexer, 0)
    }

    fn new_bp(lexer: &mut Lexer, min_bp: u8) -> Result<Expr, ParserErr> {
        let mut lhs = match lexer.advance().map_err(|e| ParserErr::LexerErr(e))? {
            Token::Atom(atom) => Expr::Atom(atom),
            Token::Op(Op::Open) => {
                let lhs = Expr::new_bp(lexer, 0)?;
                match lexer.advance() {
                    Ok(Token::Op(Op::Close)) => lhs,
                    Err(e) => {
                        return Err(ParserErr::Panic(format!(
                            "expected `{:?}`, found `{:?}`",
                            Op::Close,
                            e
                        )))
                    }
                    _ => return Err(ParserErr::Panic(format!("expected `{:?}`", Op::Close))),
                }
            }
            Token::Op(op) => {
                let right_bp = prefix_bp(op.clone()).map_err(|e| {
                    ParserErr::Panic(format!("expected prefix operator, but {}", e))
                })?;
                let rhs = Expr::new_bp(lexer, right_bp).map_err(|e| {
                    ParserErr::Panic(format!(
                        "expected right-hand-side expression of prefix operator, but {}",
                        e
                    ))
                })?;

                match op {
                    Op::Call(_) => Expr::Call(Call::new(op, rhs.list())),
                    _ => Expr::Call(Call::new(op, vec![rhs])),
                }
            }
            token => {
                return Err(ParserErr::Panic(format!(
                    "found unexpected token `{:?}`",
                    token
                )))
            }
        };

        loop {
            let op: Op = match lexer.peek() {
                Err(error) => match error {
                    LexerErr::EOF => break,
                    _ => Err(ParserErr::Panic(format!(
                        "expected in- or postfix operator, found `{}`",
                        error
                    ))),
                },
                Ok(token) => match token {
                    Token::Op(Op::Open) | Token::Atom(_) => Ok(Op::Mul),
                    Token::Op(op) => Ok(op.clone()),
                    _ => Err(ParserErr::Panic(format!(
                        "expected in- or postfix operator, found `{}`",
                        token
                    ))),
                },
            }?;

            if let Ok(left_bp) = postfix_bp(op.clone()) {
                if left_bp < min_bp {
                    break;
                }

                lexer.next();
                lhs = Expr::Call(Call::new(op, vec![lhs]));
                continue;
            }

            if let Ok((left_bp, right_bp)) = infix_bp(op.clone()) {
                if left_bp < min_bp {
                    break;
                }

                if Ok(&Token::Op(op.clone())) == lexer.peek() {
                    lexer.next();
                }

                let rhs = Expr::new_bp(lexer, right_bp).map_err(|e| {
                    ParserErr::Panic(format!(
                        "expected right-hand-side expression of infix operator, but {}",
                        e
                    ))
                })?;

                lhs = Expr::Call(Call::new(op, vec![lhs, rhs]));
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}

fn infix_bp(op: Op) -> Result<(u8, u8), ParserErr> {
    Ok(match op {
        Op::Def => (2, 1),
        Op::List => (4, 3),
        Op::Eq | Op::Neq | Op::Small | Op::Big | Op::SmallEq | Op::BigEq => (5, 6),
        Op::Mod => (8, 6),
        Op::Add | Op::Sub => (9, 10),
        Op::Mul | Op::Div => (11, 12),
        Op::Exp => (14, 13),
        Op::Child => (16, 15),
        op => {
            return Err(ParserErr::Panic(format!(
                "`{}` is not an infix operator",
                op
            )))
        }
    })
}

fn prefix_bp(op: Op) -> Result<u8, ParserErr> {
    match op {
        Op::Add | Op::Sub => Ok(13),
        Op::Call(_) => Ok(10),
        op => Err(ParserErr::Panic(format!(
            "`{}` is not an prefix operator",
            op
        ))),
    }
}

fn postfix_bp(op: Op) -> Result<u8, ParserErr> {
    match op {
        Op::Fact => Ok(15),
        op => Err(ParserErr::Panic(format!(
            "`{}` is not an postfix operator",
            op
        ))),
    }
}

pub(crate) enum TypeErr {
    Panic(String),
}

impl Expr {
    pub fn number(&mut self, env: &mut Env) -> Result<Number, TypeErr> {
        self.apply_env(env); // -> Assuming Ok, there should not be any symbol or function in root

        Ok(Number(match self {
            Expr::Atom(Atom::Number(Number(number))) => *number,
            Expr::Call(Call { op, args }) => {
                match &mut args[..] {
                    [arg] => {
                        match arg.number(env) {
                            Ok(Number(number)) => {
                                match op {
                                    Op::Call(Symbol(call)) => match call.as_ref() {
                                        "sin" => number.sin(),
                                        "asin" => number.asin(),
                                        "sinh" => number.sinh(),
                                        "asinh" => number.asinh(),

                                        "cos" => number.cos(),
                                        "acos" => number.acos(),
                                        "cosh" => number.cosh(),
                                        "acosh" => number.acosh(),

                                        "tan" => number.tan(),
                                        "atan" => number.atan(),
                                        "tanh" => number.tanh(),
                                        "atanh" => number.atanh(),

                                        "sqrt" => number.sqrt(),

                                        _ => {
                                            return Err(TypeErr::Panic(format!(
                                                "function `{}`(x: number) is not defined",
                                                call
                                            )))
                                        }
                                    },
                                    Op::Sub => -number,
                                    Op::Add => number,
                                    Op::Fact => todo!(), // Implement factorial for f64
                                    _ => {
                                        return Err(TypeErr::Panic(format!(
                                            "operator `{}` on (x: number) is not defined",
                                            op
                                        )))
                                    }
                                }
                            }
                            // Here implement functions of expressions
                            _ => {
                                return Err(TypeErr::Panic(format!(
                                    "function `{}`(x: expression) is not defined",
                                    op
                                )))
                            }
                        }
                    }
                    [first, second] => match (first.number(env), second.number(env)) {
                        (Ok(Number(first)), Ok(Number(second))) => match op {
                            Op::Call(Symbol(call)) => match call.as_ref() {
                                "log" => first.log(second),
                                "root" => first.powf(1.0 / second),
                                _ => {
                                    return Err(TypeErr::Panic(format!(
                                        "function `{}`(number, number) is not defined",
                                        call
                                    )))
                                }
                            },
                            Op::Sub => first - second,
                            Op::Add => first + second,
                            Op::Div => first / second,
                            Op::Mul => first * second,
                            Op::Exp => first.powf(second),
                            Op::Mod => first % second,
                            Op::Eq => (first == second) as u8 as f64,
                            Op::Neq => (first != second) as u8 as f64,
                            Op::Small => (first < second) as u8 as f64,
                            Op::Big => (first > second) as u8 as f64,
                            Op::SmallEq => (first <= second) as u8 as f64,
                            Op::BigEq => (first >= second) as u8 as f64,
                            _ => {
                                return Err(TypeErr::Panic(format!(
                                    "operator `{}` on (number, number) is not defined",
                                    op
                                )))
                            }
                        },
                        (Err(_), Ok(second)) => match first.symbol(env) {
                            Ok(symbol) => match op {
                                Op::Def => {
                                    env.defs.insert(
                                        symbol.clone(),
                                        Def::Atom(Expr::Atom(Atom::Number(second.clone()))),
                                    );

                                    second.0
                                }
                                _ => {
                                    return Err(TypeErr::Panic(format!(
                                        "operator `{}` on (symbol, number) not defined",
                                        op
                                    )))
                                }
                            },
                            Err(error) => return Err(error),
                        },
                        _ => {
                            return Err(TypeErr::Panic(format!(
                                "`{}`({}, {}) not defined",
                                op, first, second
                            )))
                        }
                    },
                    all => {
                        let mut args = Vec::new();
                        for expr in all {
                            args.push(expr.number(env)?);
                        }
                        let all = args;

                        match op {
                            Op::Call(Symbol(call)) => match call.as_ref() {
                                "sum" => todo!(), // Implement sum!
                                _ => {
                                    return Err(TypeErr::Panic(format!(
                                        "function `{}` is not defined for so much arguments, but may be defined for less",
                                        call
                                    )))
                                }
                            },
                            _ => {
                                return Err(TypeErr::Panic(format!(
                                    "operator `{}` is not defined for so much arguments, but may be defined for less",
                                    op
                                )))
                            }
                        }
                    }
                }
            }
            _ => return Err(TypeErr::Panic(format!("`{}` is undefined", self))),
        }))
    }

    fn symbol(&mut self, env: &Env) -> Result<Symbol, TypeErr> {
        match self {
            Expr::Atom(Atom::Symbol(symbol)) => Ok(symbol.clone()),
            _ => Err(TypeErr::Panic(format!("expected symbol, found `{}`", self))),
        }
    }

    fn apply_env(&mut self, env: &mut Env) {
        match self {
            Expr::Atom(Atom::Symbol(symbol)) => {
                if let Some(Def::Atom(rhs)) = env.defs.get(&symbol) {
                    *self = rhs.clone();
                }
            }
            Expr::Call(Call {
                op: Op::Call(call),
                args,
            }) => {
                if let Some(Def::Call(symbols, expr)) = env.defs.get(call).cloned() {
                    let mut old = Env::new();

                    for (var, ident) in args.remove(0).list().into_iter().zip(symbols.into_iter()) {
                        match env.defs.insert(ident.clone(), Def::Atom(var.clone())) {
                            Some(def) => {
                                old.defs.insert(ident, def);
                            }
                            None => {}
                        }
                    }

                    // Does this realy work? Casting a box from heap into stack?
                    let mut expr = *expr.clone();
                    (&mut expr).apply_env(env);
                    env.update(old);
                    *self = expr;
                }
            }
            _ => {}
        }
    }

    fn list(self) -> Vec<Expr> {
        match self {
            Expr::Call(Call {
                op: Op::List,
                mut args,
            }) => {
                // Optimize capacity?
                let mut list = Vec::new();
                list.push(args.remove(0));
                list.append(&mut args.remove(0).list());
                list
            }
            _ => vec![self],
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(atom) => write!(f, "{}", atom),
            Expr::Call(call) => write!(f, "{}", call),
        }
    }
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErr::Panic(panic) => write!(f, "{}", panic),
            ParserErr::LexerErr(lexer_err) => write!(f, "{}", lexer_err),
            ParserErr::Empty => write!(f, "expression is empty"),
        }
    }
}

impl fmt::Display for TypeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeErr::Panic(panic) => write!(f, "{}", panic),
        }
    }
}
