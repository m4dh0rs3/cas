use {
    super::{
        env::{Def, Env},
        lexer::E_TOKEN_LEN,
    },
    atom::{number::Number, symbol::Symbol, Atom},
    call::{op::Op, Call},
};

pub(crate) mod atom;
pub(crate) mod call;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Call(Call),
}

pub struct TypeErr(pub(crate) String);

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

    pub fn number(&self, env: &mut Env) -> Result<Number, TypeErr> {
        match self {
            Expr::Atom(Atom::Number(number)) => Ok(number.clone()),
            Expr::Atom(Atom::Symbol(symbol)) => {
                if let Def::Expr(Expr::Atom(Atom::Number(number))) = env.get(symbol)? {
                    Ok(number.clone())
                } else {
                    Err(TypeErr(format!("`{}` is not a number", self)))
                }
            }
            _ => self.eval(env)?.number(env),
        }
    }

    pub fn eval(&self, env: &mut Env) -> Result<Expr, TypeErr> {
        match self {
            Expr::Call(Call { op, args }) => {
                if let Op::Call(call) = op {
                    if let Def::Call {
                        args: symbols,
                        call: expr,
                    } = env.get(&call)?.clone()
                    {
                        let mut vars = Env::new();

                        for (var, symbol) in args.into_iter().zip(symbols.into_iter()) {
                            vars.insert(symbol, Def::Expr(var.clone()));
                        }

                        return expr.apply_env(&mut vars);
                    }
                }

                match &args[..] {
                    [x] => Expr::eval_one_arg(op, x, env),
                    [x, y] => Expr::eval_two_args(op, x, y, env),
                    tuple => Expr::eval_many_args(op, tuple, env),
                }
            }
            _ => Ok(self.clone()),
        }
    }

    fn eval_one_arg(op: &Op, x: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match op {
            Op::Call(call) => Expr::eval_call_one(call, x, env)?,

            Op::Sub => Expr::Atom(Atom::Number(-x.number(env)?)),
            Op::Add => x.clone(),
            Op::Fact => Expr::Atom(Atom::Number(x.number(env)?.fact())),

            _ => return Err(TypeErr(format!("op `{}` undefined on (expr)", op))),
        })
    }

    fn eval_call_one(call: &Symbol, x: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match &call.0[..] {
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
        })
    }

    fn eval_two_args(op: &Op, x: &Expr, y: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match op {
            Op::Call(call) => Expr::eval_call_two(call, x, y, env)?,

            Op::Def => Expr::def(x, y, env)?,

            Op::Child => Expr::child(x, y, env)?,

            Op::Add => Expr::Atom(Atom::Number(x.number(env)? + y.number(env)?)),
            Op::Sub => Expr::Atom(Atom::Number(x.number(env)? - y.number(env)?)),
            Op::Mul => Expr::Atom(Atom::Number(x.number(env)? * y.number(env)?)),
            Op::Div => Expr::Atom(Atom::Number(x.number(env)? / y.number(env)?)),
            Op::Pow => Expr::Atom(Atom::Number(x.number(env)?.pow(y.number(env)?))),
            Op::Mod => Expr::Atom(Atom::Number(x.number(env)?.modulus(y.number(env)?))),

            Op::Eq => Expr::Atom(Atom::Symbol(x.number(env)?.equal(y.number(env)?))),

            _ => return Err(TypeErr(format!("op `{}` undefined on (expr, expr)", op))),
        })
    }

    fn expr(&self, env: &mut Env) -> Result<Expr, TypeErr> {
        match self {
            Expr::Atom(Atom::Symbol(symbol)) => match env.get(symbol) {
                Ok(Def::Expr(expr)) => Ok(expr.clone()),
                _ => Err(TypeErr(format!("`{}` is an undefined symbol", symbol))),
            },
            _ => Ok(self.clone()),
        }
    }

    fn child(parent: &Expr, index: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        let index = index.number(env)?;
        let parent = parent.expr(env)?.list();

        Ok(parent
            .get(index.0 as usize)
            .ok_or(TypeErr(format!("op `{}` index out of bounds", Op::Child)))?
            .clone())
    }

    fn def(x: &Expr, y: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        match x {
            Expr::Call(Call { op: Op::Mul, args }) => {
                if let [call, list] = &args[..] {
                    let call = call.symbol()?;
                    let mut symbols = Vec::new();

                    for symbol in &list.clone().list() {
                        symbols.push(symbol.symbol()?);
                    }

                    env.insert(
                        call,
                        Def::Call {
                            args: symbols,
                            call: y.clone(),
                        },
                    );

                    Ok(Expr::Atom(Atom::Symbol(Symbol(format!("Def")))))
                } else {
                    Err(TypeErr(format!(
                        "op `{}` is undefined on (symbol / symbol(symbols, ...), expr)",
                        Op::Def
                    )))
                }
            }

            Expr::Atom(Atom::Symbol(symbol)) => {
                env.insert(
                    symbol.clone(),
                    match y {
                        Expr::Atom(Atom::Symbol(Symbol(call))) if call == "OSCall" => Def::OSCall,
                        _ => Def::Expr(y.clone()),
                    },
                );

                Ok(Expr::Atom(Atom::Symbol(Symbol(format!("Def")))))
            }

            _ => Err(TypeErr(format!(
                "op `{}` is undefined on (symbol / symbol(symbols, ...), expr)",
                Op::Def
            ))),
        }
    }

    fn eval_call_two(call: &Symbol, x: &Expr, y: &Expr, env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match &call.0[..] {
            "root" => Expr::Atom(Atom::Number(x.number(env)?.root(y.number(env)?))),
            "log" => Expr::Atom(Atom::Number(x.number(env)?.log(y.number(env)?))),
            "angle" => Expr::Atom(Atom::Number(x.number(env)?.log(y.number(env)?))),

            _ => return Err(TypeErr(format!("op `{}` undefined on (expr, expr)", call))),
        })
    }

    fn eval_many_args(op: &Op, tuple: &[Expr], env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match op {
            Op::Call(call) => match &call.0[..] {
                "sum" => {
                    let mut nums = Vec::with_capacity(E_TOKEN_LEN);

                    for arg in tuple {
                        nums.push(arg.number(env)?);
                    }

                    Expr::Atom(Atom::Number(Number::sum(nums)))
                }

                _ => return Err(TypeErr(format!("call `{}` undefined on (expr, ...)", call))),
            },

            _ => return Err(TypeErr(format!("op `{}` undefined on (expr, ...)", op))),
        })
    }

    fn symbol(&self) -> Result<Symbol, TypeErr> {
        match self {
            Expr::Atom(Atom::Symbol(symbol)) => Ok(symbol.clone()),
            // Compare number to all symbols?
            _ => Err(TypeErr(format!("expected symbol, found `{}`", self))),
        }
    }

    fn apply_env(&self, env: &mut Env) -> Result<Expr, TypeErr> {
        Ok(match self {
            Expr::Atom(Atom::Symbol(symbol)) => {
                if let Ok(Def::Expr(expr)) = env.get(&symbol) {
                    expr.clone()
                } else {
                    self.clone()
                }
            }
            Expr::Call(Call { op, args }) => {
                let mut nodes = Vec::new();
                for expr in args {
                    nodes.push(expr.apply_env(env)?);
                }

                Expr::Call(Call::new(op.clone(), nodes))
            }
            _ => self.clone(),
        })
    }

    fn order(&mut self) {}

    fn simplify(&mut self) {}
}
