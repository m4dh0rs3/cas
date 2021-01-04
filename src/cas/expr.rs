use self::atom::{number::Number, symbol::Symbol};

use super::{
    env::{Def, Env},
    lexer::E_TOKEN_LEN,
};

pub(crate) mod atom;
pub(crate) mod call;

use {
    atom::Atom,
    call::{op::Op, Call},
};

#[derive(Clone)]
pub(crate) enum Expr {
    Atom(Atom),
    Call(Call),
}

pub(crate) struct TypeErr(pub(crate) String);

impl Expr {
    pub(crate) fn list(self) -> Vec<Expr> {
        match self {
            Expr::Call(Call {
                op: Op::List,
                mut args,
            }) => {
                let mut list = Vec::with_capacity(2);
                list.push(args.remove(0));
                list.append(&mut args.remove(0).list());
                list
            }
            _ => vec![self],
        }
    }

    fn number(&self, env: &Env) -> Result<Number, TypeErr> {
        match self {
            Expr::Atom(Atom::Number(number)) => Ok(number.clone()),
            Expr::Atom(Atom::Symbol(symbol)) => {
                if let Def::Expr(Expr::Atom(Atom::Number(number))) = env.get(symbol)? {
                    Ok(number.clone())
                } else {
                    Err(TypeErr(format!("`{}` is not a number", self)))
                }
            }
            _ => Err(TypeErr(format!("`{}` is undefined", self))),
        }
    }

    fn eval(&self, env: &Env) -> Result<Expr, TypeErr> {
        if let Expr::Call(Call { op, args }) = self {
            Ok(match &args[..] {
                [x] => match op {
                    Op::Call(call) => match &call.0[..] {
                        "abs" => Expr::Atom(Atom::Number(x.number(env)?.abs())),
                        "signum" => Expr::Atom(Atom::Number(x.number(env)?.signum())),
                        "ceil" => Expr::Atom(Atom::Number(x.number(env)?.ceil())),
                        "floor" => Expr::Atom(Atom::Number(x.number(env)?.floor())),
                        "round" => Expr::Atom(Atom::Number(x.number(env)?.round())),
                        "trunc" => Expr::Atom(Atom::Number(x.number(env)?.trunc())),
                        "fract" => Expr::Atom(Atom::Number(x.number(env)?.fract())),

                        "sin" => Expr::Atom(Atom::Number(x.number(env)?.sin())),
                        "asin" => Expr::Atom(Atom::Number(x.number(env)?.asin())),
                        "sinh" => Expr::Atom(Atom::Number(x.number(env)?.sinh())),
                        "asinh" => Expr::Atom(Atom::Number(x.number(env)?.asinh())),

                        "cos" => Expr::Atom(Atom::Number(x.number(env)?.cos())),
                        "acos" => Expr::Atom(Atom::Number(x.number(env)?.acos())),
                        "cosh" => Expr::Atom(Atom::Number(x.number(env)?.cosh())),
                        "acosh" => Expr::Atom(Atom::Number(x.number(env)?.acosh())),

                        "tan" => Expr::Atom(Atom::Number(x.number(env)?.tan())),
                        "atan" => Expr::Atom(Atom::Number(x.number(env)?.atan())),
                        "tanh" => Expr::Atom(Atom::Number(x.number(env)?.tanh())),
                        "atanh" => Expr::Atom(Atom::Number(x.number(env)?.atanh())),

                        "ln" => Expr::Atom(Atom::Number(x.number(env)?.ln())),
                        "lg" => Expr::Atom(Atom::Number(x.number(env)?.lg())),

                        "sqrt" => Expr::Atom(Atom::Number(x.number(env)?.sqrt())),
                        "cbrt" => Expr::Atom(Atom::Number(x.number(env)?.cbrt())),

                        _ => return Err(TypeErr(format!("call `{}` undefined on (expr)", call))),
                    },

                    Op::Sub => Expr::Atom(Atom::Number(-x.number(env)?)),
                    Op::Add => self.clone(),
                    Op::Fact => Expr::Atom(Atom::Number(x.number(env)?.fact())),

                    _ => return Err(TypeErr(format!("op `{}` undefined on (expr)", op))),
                },

                [x, y] => match op {
                    Op::Call(call) => match &call.0[..] {
                        "root" => Expr::Atom(Atom::Number(x.number(env)?.root(y.number(env)?))),
                        "log" => Expr::Atom(Atom::Number(x.number(env)?.log(y.number(env)?))),
                        "angle" => Expr::Atom(Atom::Number(x.number(env)?.log(y.number(env)?))),

                        _ => {
                            return Err(TypeErr(format!("op `{}` undefined on (expr, expr)", call)))
                        }
                    },

                    Op::Add => Expr::Atom(Atom::Number(x.number(env)? + y.number(env)?)),
                    Op::Sub => Expr::Atom(Atom::Number(x.number(env)? - y.number(env)?)),
                    Op::Mul => Expr::Atom(Atom::Number(x.number(env)? * y.number(env)?)),
                    Op::Div => Expr::Atom(Atom::Number(x.number(env)? / y.number(env)?)),
                    Op::Pow => Expr::Atom(Atom::Number(x.number(env)?.pow(y.number(env)?))),
                    Op::Mod => Expr::Atom(Atom::Number(x.number(env)?.modulus(y.number(env)?))),
                    Op::Eq => Expr::Atom(Atom::Symbol(x.number(env)?.equal(y.number(env)?))),

                    _ => return Err(TypeErr(format!("op `{}` undefined on (expr, expr)", op))),
                },

                tuple => match op {
                    Op::Call(call) => match &call.0[..] {
                        "sum" => {
                            let mut nums = Vec::with_capacity(E_TOKEN_LEN);

                            for arg in tuple {
                                nums.push(arg.number(env)?);
                            }

                            Expr::Atom(Atom::Number(Number::sum(nums)))
                        }

                        _ => {
                            return Err(TypeErr(format!(
                                "call `{}` undefined on (expr, ...)",
                                call
                            )))
                        }
                    },

                    _ => return Err(TypeErr(format!("op `{}` undefined on (expr, ...)", op))),
                },
            })
        } else {
            Ok(self.clone())
        }
    }

    fn symbol(&self) -> Result<Symbol, TypeErr> {
        match self {
            Expr::Atom(Atom::Symbol(symbol)) => Ok(symbol.clone()),
            // Compare number to all symbols?
            _ => Err(TypeErr(format!("expected symbol, found `{}`", self))),
        }
    }

    fn order(&mut self) {}

    fn simplify(&mut self) {}
}
