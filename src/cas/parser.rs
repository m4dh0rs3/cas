use super::{
    env::Env,
    expr::{
        call::{op::Op, Call},
        Expr,
    },
    lexer::{Lexer, LexerErr, Token},
};

pub(crate) enum ParserErr {
    Empty,
    LexerErr(LexerErr),
    Panic(String),
}

impl Expr {
    pub(crate) fn parse(input: &str, env: &Env) -> Result<Expr, ParserErr> {
        let mut lexer = Lexer::new(input, env);

        if let Err(LexerErr::EOF) = lexer.peek() {
            Err(ParserErr::Empty)
        } else {
            Expr::parse_bp(&mut lexer, 0)
        }
    }

    fn parse_bp(lexer: &mut Lexer, min_bp: u8) -> Result<Expr, ParserErr> {
        let mut lhs = match lexer.token().map_err(|error| error.parser_err())? {
            Token::Atom(atom) => Ok(Expr::Atom(atom)),

            Token::Op(Op::Open) => {
                let lhs = Expr::parse_bp(lexer, 0)?;

                match lexer.token() {
                    Ok(Token::Op(Op::Close)) => Ok(lhs),
                    Err(error) => Err(ParserErr::Panic(format!(
                        "expected `{}`, but `{}`",
                        Op::Close,
                        error,
                    ))),
                    Ok(token) => Err(ParserErr::Panic(format!(
                        "expected `{}`, found `{}`",
                        Op::Close,
                        token
                    ))),
                }
            }

            Token::Op(op) => {
                let right_bp = prefix_bp(&op).ok_or(ParserErr::Panic(format!(
                    "expected prefix op, but found `{}`",
                    &op
                )))?;

                let rhs = Expr::parse_bp(lexer, right_bp).map_err(|error| {
                    ParserErr::Panic(format!("expected rhs of expr, but {}", error))
                })?;

                Ok(match op {
                    // Replace with rhs.list(), when iplemneted (only for Op::Call(_)!)
                    Op::Call(_) => Expr::Call(Call::new(op, rhs.list())),
                    _ => Expr::Call(Call::new(op, vec![rhs])),
                })
            }

            token => Err(ParserErr::Panic(format!("unexpected token `{}`", token))),
        }?;

        loop {
            let op = match lexer.peek() {
                Err(error) => match error {
                    LexerErr::EOF => break,
                    _ => Err(ParserErr::Panic(format!(
                        "expected in- or postfix op, but {}",
                        error
                    ))),
                },
                Ok(token) => match token {
                    Token::Op(Op::Open) | Token::Atom(_) => Ok(&Op::Mul),
                    Token::Op(op) => Ok(op),
                    _ => Err(ParserErr::Panic(format!(
                        "expected in- or postfix op, found `{}`",
                        token
                    ))),
                },
            }?
            .clone();

            if let Some(left_bp) = postfix_bp(&op) {
                if left_bp < min_bp {
                    break;
                }

                lexer.token();
                lhs = Expr::Call(Call::new(op, vec![lhs]));

                continue;
            }

            if let Some((left_bp, right_bp)) = infix_bp(&op) {
                if left_bp < min_bp {
                    break;
                }

                if Ok(&Token::Op(op.clone())) == lexer.peek() {
                    lexer.token();
                }

                let rhs = Expr::parse_bp(lexer, right_bp).map_err(|error| {
                    ParserErr::Panic(format!("expected rhs of expr, but {}", error))
                })?;

                lhs = Expr::Call(Call::new(op, vec![lhs, rhs]));

                continue;
            }

            break;
        }

        Ok(lhs)
    }
}

fn infix_bp(op: &Op) -> Option<(u8, u8)> {
    Some(match op {
        Op::Def => (2, 1),
        Op::List => (4, 3),
        Op::Eq | Op::Neq | Op::Less | Op::More | Op::LessEq | Op::MoreEq => (5, 6),
        Op::Mod => (8, 6),
        Op::Add | Op::Sub => (9, 10),
        Op::Mul | Op::Div => (11, 12),
        Op::Pow => (14, 13),
        Op::Child => (15, 16),
        _ => return None,
    })
}

fn prefix_bp(op: &Op) -> Option<u8> {
    Some(match op {
        Op::Add | Op::Sub => 13,
        Op::Call(_) => 10,
        _ => return None,
    })
}

fn postfix_bp(op: &Op) -> Option<u8> {
    Some(match op {
        Op::Fact => 15,
        _ => return None,
    })
}
